<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Image artifact extracted from a document page.
 *
 * @property-read string $data Image data (bytes)
 * @property-read string $format Image format (e.g., "png", "jpeg")
 * @property-read int $imageIndex Image index within document
 * @property-read int|null $pageNumber Page number where image was found
 * @property-read int|null $width Image width in pixels
 * @property-read int|null $height Image height in pixels
 * @property-read string|null $colorspace Image colorspace
 * @property-read int|null $bitsPerComponent Bits per color component
 * @property-read bool $isMask Whether image is a mask
 * @property-read string|null $description Image description/alt text
 * @property-read ExtractionResult|null $ocrResult OCR result if OCR was performed on this image
 */
readonly class ExtractedImage
{
    public function __construct(
        public string $data,
        public string $format,
        public int $imageIndex,
        public ?int $pageNumber = null,
        public ?int $width = null,
        public ?int $height = null,
        public ?string $colorspace = null,
        public ?int $bitsPerComponent = null,
        public bool $isMask = false,
        public ?string $description = null,
        public ?ExtractionResult $ocrResult = null,
    ) {
    }

    /**
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        return new self(
            data: $data['data'] ?? '',
            format: $data['format'] ?? '',
            imageIndex: $data['image_index'] ?? 0,
            pageNumber: $data['page_number'] ?? null,
            width: $data['width'] ?? null,
            height: $data['height'] ?? null,
            colorspace: $data['colorspace'] ?? null,
            bitsPerComponent: $data['bits_per_component'] ?? null,
            isMask: $data['is_mask'] ?? false,
            description: $data['description'] ?? null,
            ocrResult: isset($data['ocr_result'])
                ? ExtractionResult::fromArray($data['ocr_result'])
                : null,
        );
    }
}
