defmodule KreuzbergTest.Integration.ImageExtractionTest do
  @moduledoc """
  Integration tests for image extraction functionality.

  Tests cover:
  - Image struct creation and manipulation
  - Image metadata (width, height, format, DPI)
  - Image data (binary/base64 handling)
  - OCR text extraction from images
  - Pattern matching on image structures
  - Image serialization and deserialization
  """

  use ExUnit.Case, async: true

  # Sample 1x1 PNG in base64 (valid PNG with minimal data)
  @sample_png_base64 "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg=="

  describe "Image struct creation" do
    @tag :integration
    test "creates Image struct with format" do
      image = Kreuzberg.Image.new("png")

      assert image.format == "png"
      assert %Kreuzberg.Image{} = image
    end

    @tag :integration
    test "creates Image struct with metadata" do
      image = Kreuzberg.Image.new("jpeg", width: 1920, height: 1080)

      assert image.format == "jpeg"
      assert image.width == 1920
      assert image.height == 1080
    end

    @tag :integration
    test "creates Image struct with color space" do
      image = Kreuzberg.Image.new("png", colorspace: "RGB")

      assert image.format == "png"
      assert image.colorspace == "RGB"
    end

    @tag :integration
    test "creates Image struct with OCR result" do
      image =
        Kreuzberg.Image.new(
          "png",
          width: 640,
          height: 480,
          ocr_result: %Kreuzberg.ExtractionResult{content: "Extracted text from image"}
        )

      assert image.ocr_result.content == "Extracted text from image"
      assert image.width == 640
      assert image.height == 480
    end

    @tag :integration
    test "creates Image struct with image index" do
      image =
        Kreuzberg.Image.new(
          "png",
          image_index: 0
        )

      assert image.format == "png"
      assert image.image_index == 0
    end

    @tag :integration
    test "creates Image struct with page number" do
      image =
        Kreuzberg.Image.new(
          "jpeg",
          page_number: 5
        )

      assert image.page_number == 5
    end

    @tag :integration
    test "creates Image struct from map" do
      image_map = %{
        "format" => "png",
        "width" => 800,
        "height" => 600,
        "image_index" => 0
      }

      image = Kreuzberg.Image.from_map(image_map)

      assert %Kreuzberg.Image{} = image
      assert image.format == "png"
      assert image.width == 800
      assert image.height == 600
      assert image.image_index == 0
    end
  end

  describe "Image data and metadata" do
    @tag :integration
    test "stores binary image data" do
      png_binary = Base.decode64!(@sample_png_base64)

      image =
        Kreuzberg.Image.new(
          "png",
          data: png_binary
        )

      assert image.data == png_binary
      assert is_binary(image.data)
      assert byte_size(image.data) > 0
    end

    @tag :integration
    test "calculates aspect ratio" do
      image = %Kreuzberg.Image{
        format: "png",
        width: 1920,
        height: 1080
      }

      aspect = image.width / image.height

      assert is_float(aspect) or is_integer(aspect)
      # 16:9 aspect ratio
      assert aspect > 1.0
    end

    @tag :integration
    test "aspect ratio for square image" do
      image = %Kreuzberg.Image{
        format: "jpeg",
        width: 512,
        height: 512
      }

      aspect = image.width / image.height

      assert aspect == 1.0
    end

    @tag :integration
    test "aspect ratio returns nil for missing dimensions" do
      image = %Kreuzberg.Image{
        format: "png"
      }

      aspect = if image.width && image.height, do: image.width / image.height, else: nil

      assert aspect == nil
    end

    @tag :integration
    test "checks if image has data" do
      image_with_data =
        Kreuzberg.Image.new(
          "png",
          data: Base.decode64!(@sample_png_base64)
        )

      image_without_data = Kreuzberg.Image.new("png")

      assert Kreuzberg.Image.has_data?(image_with_data)
      refute Kreuzberg.Image.has_data?(image_without_data)
    end

    @tag :integration
    test "stores binary data correctly" do
      png_binary = Base.decode64!(@sample_png_base64)

      image =
        Kreuzberg.Image.new(
          "png",
          data: png_binary
        )

      assert byte_size(image.data) == byte_size(png_binary)
    end
  end

  describe "Image format handling" do
    @tag :integration
    test "supports PNG format" do
      image = Kreuzberg.Image.new("png")

      assert image.format == "png"
    end

    @tag :integration
    test "supports JPEG format" do
      image = Kreuzberg.Image.new("jpeg")

      assert image.format == "jpeg"
    end

    @tag :integration
    test "supports WebP format" do
      image = Kreuzberg.Image.new("webp")

      assert image.format == "webp"
    end

    @tag :integration
    test "supports TIFF format" do
      image = Kreuzberg.Image.new("tiff")

      assert image.format == "tiff"
    end

    @tag :integration
    test "format field is set correctly" do
      formats = ["png", "jpeg", "webp", "gif"]

      Enum.each(formats, fn format ->
        image = Kreuzberg.Image.new(format)
        assert image.format == format
      end)
    end
  end

  describe "Image OCR results" do
    @tag :integration
    test "stores OCR extracted text" do
      ocr_text = """
      This is text extracted from an image.
      It can span multiple lines.
      And contain various formatting.
      """

      image =
        Kreuzberg.Image.new(
          "png",
          ocr_result: %Kreuzberg.ExtractionResult{content: ocr_text}
        )

      assert image.ocr_result.content == ocr_text
      assert String.length(image.ocr_result.content) > 0
    end

    @tag :integration
    test "handles empty OCR text" do
      image =
        Kreuzberg.Image.new(
          "png",
          ocr_result: %Kreuzberg.ExtractionResult{content: ""}
        )

      assert image.ocr_result.content == ""
    end

    @tag :integration
    test "OCR text with unicode characters" do
      unicode_text = "Chinese: 你好, Arabic: مرحبا, Emoji: 🖼️"

      image =
        Kreuzberg.Image.new(
          "jpeg",
          ocr_result: %Kreuzberg.ExtractionResult{content: unicode_text}
        )

      assert image.ocr_result.content == unicode_text
    end

    @tag :integration
    test "OCR text with special characters" do
      special_text = "Special chars: !@#$%^&*()_+-=[]{}|;:',.<>?/\\`~"

      image =
        Kreuzberg.Image.new(
          "png",
          ocr_result: %Kreuzberg.ExtractionResult{content: special_text}
        )

      assert image.ocr_result.content == special_text
    end
  end

  describe "Image serialization" do
    @tag :integration
    test "converts Image to map" do
      image =
        Kreuzberg.Image.new(
          "png",
          width: 640,
          height: 480,
          image_index: 0
        )

      image_map = Kreuzberg.Image.to_map(image)

      assert is_map(image_map)
      assert image_map["format"] == "png"
      assert image_map["width"] == 640
      assert image_map["height"] == 480
      assert image_map["image_index"] == 0
    end

    @tag :integration
    test "round-trips through serialization" do
      original =
        Kreuzberg.Image.new(
          "jpeg",
          width: 1024,
          height: 768,
          image_index: 0,
          ocr_result: %Kreuzberg.ExtractionResult{content: "Sample OCR text"}
        )

      image_map = Kreuzberg.Image.to_map(original)
      restored = Kreuzberg.Image.from_map(image_map)

      assert restored.format == original.format
      assert restored.width == original.width
      assert restored.height == original.height
      assert restored.image_index == original.image_index
      assert restored.ocr_result.content == original.ocr_result.content
    end

    @tag :integration
    test "serializes to JSON" do
      image =
        Kreuzberg.Image.new(
          "png",
          width: 800,
          height: 600,
          image_index: 0,
          ocr_result: %Kreuzberg.ExtractionResult{content: "Text in image"}
        )

      image_map = Kreuzberg.Image.to_map(image)
      json = Jason.encode!(image_map)

      assert is_binary(json)
      {:ok, decoded} = Jason.decode(json)
      assert decoded["format"] == "png"
      assert decoded["width"] == 800
      assert decoded["ocr_result"]["content"] == "Text in image"
    end

    @tag :integration
    test "preserves metadata in serialization" do
      image =
        Kreuzberg.Image.new(
          "tiff",
          width: 2048,
          height: 1536,
          page_number: 3
        )

      image_map = Kreuzberg.Image.to_map(image)
      json = Jason.encode!(image_map)
      {:ok, decoded} = Jason.decode(json)

      assert decoded["width"] == 2048
      assert decoded["height"] == 1536
      assert decoded["page_number"] == 3
    end
  end

  describe "Pattern matching on images" do
    @tag :integration
    test "matches on Image struct with format" do
      image = Kreuzberg.Image.new("png")

      case image do
        %Kreuzberg.Image{format: "png"} ->
          assert true

        _ ->
          flunk("Pattern match failed")
      end
    end

    @tag :integration
    test "matches on Image with dimensions" do
      image =
        Kreuzberg.Image.new(
          "jpeg",
          width: 1920,
          height: 1080
        )

      case image do
        %Kreuzberg.Image{width: w, height: h} when w > 1000 and h > 1000 ->
          assert true

        _ ->
          flunk("Dimension pattern match failed")
      end
    end

    @tag :integration
    test "matches on Image with OCR result" do
      image =
        Kreuzberg.Image.new(
          "png",
          ocr_result: %Kreuzberg.ExtractionResult{content: "Some extracted text"}
        )

      case image do
        %Kreuzberg.Image{ocr_result: result} when result != nil ->
          assert true

        _ ->
          flunk("OCR result pattern match failed")
      end
    end

    @tag :integration
    test "matches on Image with high bits per component" do
      image =
        Kreuzberg.Image.new(
          "png",
          bits_per_component: 8
        )

      case image do
        %Kreuzberg.Image{bits_per_component: bits} when bits >= 8 ->
          assert true

        _ ->
          flunk("bits per component pattern match failed")
      end
    end
  end

  describe "Image dimensions and quality" do
    @tag :integration
    test "handles various width/height combinations" do
      test_dimensions = [
        # VGA
        {640, 480},
        # SVGA
        {800, 600},
        # XGA
        {1024, 768},
        # Full HD
        {1920, 1080},
        # QHD
        {2560, 1440}
      ]

      Enum.each(test_dimensions, fn {width, height} ->
        image = Kreuzberg.Image.new("jpeg", width: width, height: height)
        assert image.width == width
        assert image.height == height
      end)
    end

    @tag :integration
    test "handles various colorspace values" do
      test_colorspaces = ["RGB", "CMYK", "Grayscale", "Lab"]

      Enum.each(test_colorspaces, fn colorspace ->
        image = Kreuzberg.Image.new("png", colorspace: colorspace)
        assert image.colorspace == colorspace
      end)
    end

    @tag :integration
    test "stores page number for multi-page documents" do
      image1 = Kreuzberg.Image.new("png", page_number: 1)
      image2 = Kreuzberg.Image.new("png", page_number: 2)
      image3 = Kreuzberg.Image.new("png", page_number: 3)

      assert image1.page_number == 1
      assert image2.page_number == 2
      assert image3.page_number == 3
    end
  end

  describe "Image edge cases" do
    @tag :integration
    test "handles very large dimensions" do
      image =
        Kreuzberg.Image.new(
          "tiff",
          width: 10_000,
          height: 10_000
        )

      assert image.width == 10_000
      assert image.height == 10_000
    end

    @tag :integration
    test "handles minimal dimensions" do
      image =
        Kreuzberg.Image.new(
          "png",
          width: 1,
          height: 1
        )

      assert image.width == 1
      assert image.height == 1
      aspect = image.width / image.height
      assert aspect == 1.0
    end

    @tag :integration
    test "handles empty OCR result gracefully" do
      image =
        Kreuzberg.Image.new(
          "jpeg",
          ocr_result: %Kreuzberg.ExtractionResult{content: ""}
        )

      assert image.ocr_result.content == ""
      refute String.length(image.ocr_result.content) > 0
    end

    @tag :integration
    test "handles very long OCR text" do
      long_text = String.duplicate("A", 100_000)

      image =
        Kreuzberg.Image.new(
          "png",
          ocr_result: %Kreuzberg.ExtractionResult{content: long_text}
        )

      assert String.length(image.ocr_result.content) == 100_000
    end

    @tag :integration
    test "handles various bits per component values" do
      bits_values = [1, 4, 8, 16, 32]

      Enum.each(bits_values, fn bits ->
        image = Kreuzberg.Image.new("jpeg", bits_per_component: bits)
        assert image.bits_per_component == bits
      end)
    end

    @tag :integration
    test "handles nil optional fields" do
      image = %Kreuzberg.Image{
        format: "png",
        data: nil,
        width: nil,
        height: nil,
        ocr_result: nil
      }

      assert image.format == "png"
      assert image.data == nil
      assert image.width == nil
      refute Kreuzberg.Image.has_data?(image)
    end
  end

  describe "Image struct completeness" do
    @tag :integration
    test "includes all fields in to_map" do
      image =
        Kreuzberg.Image.new(
          "png",
          width: 640,
          height: 480,
          image_index: 0,
          colorspace: "RGB",
          ocr_result: %Kreuzberg.ExtractionResult{content: "test"},
          page_number: 1,
          bits_per_component: 8
        )

      image_map = Kreuzberg.Image.to_map(image)

      assert Map.has_key?(image_map, "format")
      assert Map.has_key?(image_map, "width")
      assert Map.has_key?(image_map, "height")
      assert Map.has_key?(image_map, "image_index")
      assert Map.has_key?(image_map, "colorspace")
      assert Map.has_key?(image_map, "ocr_result")
      assert Map.has_key?(image_map, "page_number")
      assert Map.has_key?(image_map, "bits_per_component")
    end

    @tag :integration
    test "restores all fields from map" do
      original_map = %{
        "format" => "jpeg",
        "width" => 1024,
        "height" => 768,
        "image_index" => 0,
        "colorspace" => "RGB",
        "ocr_result" => %{"content" => "restored text", "mime_type" => ""},
        "page_number" => 2,
        "bits_per_component" => 8
      }

      image = Kreuzberg.Image.from_map(original_map)

      assert image.format == "jpeg"
      assert image.width == 1024
      assert image.height == 768
      assert image.image_index == 0
      assert image.colorspace == "RGB"
      assert image.ocr_result.content == "restored text"
      assert image.page_number == 2
      assert image.bits_per_component == 8
    end
  end
end
