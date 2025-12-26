<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * Tesseract OCR configuration.
 */
readonly class TesseractConfig
{
    public function __construct(
        public ?int $psm = null,
        public ?int $oem = null,
        public bool $enableTableDetection = false,
        public ?string $tesseditCharWhitelist = null,
        public ?string $tesseditCharBlacklist = null,
    ) {
    }

    /**
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        return array_filter([
            'psm' => $this->psm,
            'oem' => $this->oem,
            'enable_table_detection' => $this->enableTableDetection,
            'tessedit_char_whitelist' => $this->tesseditCharWhitelist,
            'tessedit_char_blacklist' => $this->tesseditCharBlacklist,
        ], static fn ($value): bool => $value !== null);
    }
}
