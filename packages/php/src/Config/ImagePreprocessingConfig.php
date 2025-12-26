<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * Image preprocessing configuration for OCR.
 */
readonly class ImagePreprocessingConfig
{
    public function __construct(
        public ?int $targetDpi = null,
        public bool $autoRotate = false,
        public bool $deskew = false,
        public ?string $binarizationMethod = null,
        public bool $denoise = false,
        public bool $sharpen = false,
        public ?float $contrastAdjustment = null,
        public ?float $brightnessAdjustment = null,
    ) {
    }

    /**
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        return array_filter([
            'target_dpi' => $this->targetDpi,
            'auto_rotate' => $this->autoRotate,
            'deskew' => $this->deskew,
            'binarization_method' => $this->binarizationMethod,
            'denoise' => $this->denoise,
            'sharpen' => $this->sharpen,
            'contrast_adjustment' => $this->contrastAdjustment,
            'brightness_adjustment' => $this->brightnessAdjustment,
        ], static fn ($value): bool => $value !== null);
    }
}
