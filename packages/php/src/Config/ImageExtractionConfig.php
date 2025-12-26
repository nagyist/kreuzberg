<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * Image extraction configuration.
 */
readonly class ImageExtractionConfig
{
    public function __construct(
        public bool $extractImages = false,
        public bool $performOcr = false,
        public ?int $minWidth = null,
        public ?int $minHeight = null,
    ) {
    }

    /**
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        return array_filter([
            'extract_images' => $this->extractImages,
            'perform_ocr' => $this->performOcr,
            'min_width' => $this->minWidth,
            'min_height' => $this->minHeight,
        ], static fn ($value): bool => $value !== null);
    }
}
