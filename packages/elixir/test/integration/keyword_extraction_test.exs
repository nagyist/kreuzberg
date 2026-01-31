defmodule KreuzbergTest.Integration.KeywordExtractionTest do
  @moduledoc """
  Integration tests for keyword extraction functionality.

  Note: The ExtractionResult struct does NOT have a `keywords` field.
  Extracted keywords are available via `Metadata` which contains metadata keywords
  (document-level keywords from file metadata like PDF keywords, title keywords, etc.).

  This test module validates:
  - Metadata keywords extraction (from document metadata)
  - Document content extraction with various configurations
  - Pattern matching on extraction results
  """

  use ExUnit.Case, async: true

  @sample_text """
  Natural Language Processing (NLP) is a subfield of linguistics, computer science, and artificial intelligence
  concerned with the interactions between computers and human language. NLP is used to apply machine learning
  algorithms to text and speech. Some NLP tasks include text classification, named entity recognition, and
  machine translation. Python is the most popular language for NLP development.
  """

  describe "document extraction with content" do
    @tag :integration
    test "extracts content from plain text" do
      config = %Kreuzberg.ExtractionConfig{}

      {:ok, result} = Kreuzberg.extract(@sample_text, "text/plain", config)

      assert result != nil
      assert is_binary(result.content)
      assert String.length(result.content) > 0
    end

    @tag :integration
    test "maintains metadata when available" do
      config = %Kreuzberg.ExtractionConfig{}

      {:ok, result} = Kreuzberg.extract(@sample_text, "text/plain", config)

      assert result.metadata != nil
      assert is_map(result.metadata)
    end

    @tag :integration
    test "handles empty text gracefully" do
      config = %Kreuzberg.ExtractionConfig{}

      # Empty input is rejected by the Rust core - this is correct behavior
      {:error, _reason} = Kreuzberg.extract("", "text/plain", config)
    end
  end

  describe "pattern matching on extraction results" do
    @tag :integration
    test "matches on extraction result structure" do
      config = %Kreuzberg.ExtractionConfig{}

      {:ok, result} = Kreuzberg.extract(@sample_text, "text/plain", config)

      case result do
        %Kreuzberg.ExtractionResult{content: _content, metadata: _metadata} ->
          assert true

        _ ->
          assert false, "Result should be an ExtractionResult struct"
      end
    end

    @tag :integration
    test "extraction result has expected fields" do
      config = %Kreuzberg.ExtractionConfig{}

      {:ok, result} = Kreuzberg.extract(@sample_text, "text/plain", config)

      # Verify result has expected structure
      assert is_binary(result.content)
      assert is_binary(result.mime_type)
      assert result.metadata != nil
      assert is_list(result.tables)
    end
  end

  describe "extraction configuration validation" do
    @tag :integration
    test "accepts extraction config with keywords option" do
      config = %Kreuzberg.ExtractionConfig{
        keywords: %{
          "enabled" => true,
          "algorithm" => "yake",
          "max_keywords" => 10,
          "min_score" => 0.1
        }
      }

      assert config.keywords["algorithm"] == "yake"
      assert config.keywords["max_keywords"] == 10
    end

    @tag :integration
    test "handles nil keyword config" do
      config = %Kreuzberg.ExtractionConfig{
        keywords: nil
      }

      {:ok, result} = Kreuzberg.extract(@sample_text, "text/plain", config)
      assert result != nil
      assert is_binary(result.content)
    end

    @tag :integration
    test "extraction works with and without keyword config" do
      config_with_keywords = %Kreuzberg.ExtractionConfig{
        keywords: %{
          "algorithm" => "yake",
          "max_keywords" => 10,
          "min_score" => 0.1
        }
      }

      config_without_keywords = %Kreuzberg.ExtractionConfig{
        keywords: nil
      }

      {:ok, result_with} = Kreuzberg.extract(@sample_text, "text/plain", config_with_keywords)
      {:ok, result_without} =
        Kreuzberg.extract(@sample_text, "text/plain", config_without_keywords)

      assert result_with != nil
      assert result_without != nil
      assert is_binary(result_with.content)
      assert is_binary(result_without.content)
    end
  end

  describe "multi-language content processing" do
    @tag :integration
    test "extraction detects content language" do
      config = %Kreuzberg.ExtractionConfig{}

      {:ok, result} = Kreuzberg.extract(@sample_text, "text/plain", config)

      assert result != nil
      assert is_list(result.detected_languages) or result.detected_languages == nil
    end
  end

  describe "extraction result structure" do
    @tag :integration
    test "result contains expected fields" do
      config = %Kreuzberg.ExtractionConfig{}

      {:ok, result} = Kreuzberg.extract(@sample_text, "text/plain", config)

      assert Map.has_key?(result, :content)
      assert Map.has_key?(result, :metadata)
      assert Map.has_key?(result, :tables)
      refute Map.has_key?(result, :keywords)
    end

    @tag :integration
    test "metadata may contain keywords from document metadata" do
      config = %Kreuzberg.ExtractionConfig{}

      {:ok, result} = Kreuzberg.extract(@sample_text, "text/plain", config)

      metadata = result.metadata
      assert metadata != nil

      if metadata.keywords != nil do
        assert is_list(metadata.keywords)
        Enum.each(metadata.keywords, fn keyword ->
          assert is_binary(keyword)
        end)
      end
    end

    @tag :integration
    test "can serialize extraction result to JSON" do
      config = %Kreuzberg.ExtractionConfig{}

      {:ok, result} = Kreuzberg.extract(@sample_text, "text/plain", config)

      result_map = Kreuzberg.ExtractionResult.to_map(result)
      json = Jason.encode!(result_map)
      assert is_binary(json)
      {:ok, decoded} = Jason.decode(json)
      assert is_map(decoded)
      assert Map.has_key?(decoded, "content")
      assert Map.has_key?(decoded, "metadata")
    end
  end
end
