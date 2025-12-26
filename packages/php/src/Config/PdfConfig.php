<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * PDF extraction configuration.
 */
readonly class PdfConfig
{
    public function __construct(
        public bool $extractImages = false,
        public bool $extractMetadata = true,
        public bool $ocrFallback = false,
        public ?int $startPage = null,
        public ?int $endPage = null,
    ) {
    }

    /**
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        return array_filter([
            'extract_images' => $this->extractImages,
            'extract_metadata' => $this->extractMetadata,
            'ocr_fallback' => $this->ocrFallback,
            'start_page' => $this->startPage,
            'end_page' => $this->endPage,
        ], static fn ($value): bool => $value !== null);
    }
}
