<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * OCR configuration.
 */
readonly class OcrConfig
{
    public function __construct(
        public string $backend = 'tesseract',
        public string $language = 'eng',
        public ?TesseractConfig $tesseractConfig = null,
        public ?ImagePreprocessingConfig $imagePreprocessing = null,
    ) {
    }

    /**
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        return array_filter([
            'backend' => $this->backend,
            'language' => $this->language,
            'tesseract_config' => $this->tesseractConfig?->toArray(),
            'image_preprocessing' => $this->imagePreprocessing?->toArray(),
        ], static fn ($value): bool => $value !== null);
    }
}
