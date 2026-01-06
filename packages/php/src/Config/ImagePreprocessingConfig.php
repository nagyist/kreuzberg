<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * Image preprocessing configuration for OCR.
 *
 * Configuration class for controlling how images are preprocessed before
 * OCR processing. Provides settings for resolution scaling, quality, color
 * mode, and various image enhancement techniques.
 */
readonly class ImagePreprocessingConfig
{
    public function __construct(
        /**
         * Target DPI for image upscaling/downscaling.
         *
         * Adjusts image resolution to the specified dots-per-inch.
         * Higher DPI improves OCR accuracy but increases processing time.
         * Standard DPI for OCR is 300.
         *
         * Valid range: 50-600 DPI
         * Recommended values:
         * - 150: Fast processing, lower quality
         * - 300: Standard, good balance (DEFAULT)
         * - 400: Better for small text
         * - 600: Maximum quality, slower
         *
         * @var int
         * @default 300
         */
        public int $targetDpi = 300,

        /**
         * JPEG compression quality for processed images.
         *
         * Controls the compression level when saving processed images.
         * Higher values preserve more detail but result in larger files.
         * Lower values reduce file size but may lose image quality.
         *
         * Valid range: 1-100
         * Recommended values:
         * - 70-80: Good balance
         * - 90: High quality (DEFAULT)
         * - 95+: Maximum quality, larger files
         *
         * @var int
         * @default 90
         */
        public int $quality = 90,

        /**
         * Convert images to grayscale before processing.
         *
         * When enabled, converts color images to grayscale (single channel).
         * Reduces file size and can improve OCR performance for text-heavy
         * documents. May reduce quality for documents requiring color information.
         *
         * @var bool
         * @default false
         */
        public bool $grayscale = false,

        /**
         * Apply noise reduction/denoising to images.
         *
         * When enabled, reduces visual noise in images which can improve
         * OCR accuracy for low-quality scans, faxes, or damaged documents.
         * May slightly blur sharp edges.
         *
         * @var bool
         * @default false
         */
        public bool $denoise = false,

        /**
         * Correct image skew (perspective distortion).
         *
         * When enabled, detects and corrects slight skewing or perspective
         * distortion in scanned documents. Improves OCR accuracy for
         * imperfectly scanned or photographed documents.
         *
         * @var bool
         * @default false
         */
        public bool $deskew = false,

        /**
         * Remove background from images.
         *
         * When enabled, attempts to detect and remove backgrounds from images,
         * keeping only the foreground content (text/objects). Useful for
         * documents with complex or colored backgrounds.
         *
         * @var bool
         * @default false
         */
        public bool $removeBackground = false,
    ) {
    }

    /**
     * Create configuration from array data.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        /** @var int $targetDpi */
        $targetDpi = $data['target_dpi'] ?? 300;
        if (!is_int($targetDpi)) {
            /** @var int $targetDpi */
            $targetDpi = (int) $targetDpi;
        }

        /** @var int $quality */
        $quality = $data['quality'] ?? 90;
        if (!is_int($quality)) {
            /** @var int $quality */
            $quality = (int) $quality;
        }

        /** @var bool $grayscale */
        $grayscale = $data['grayscale'] ?? false;
        if (!is_bool($grayscale)) {
            /** @var bool $grayscale */
            $grayscale = (bool) $grayscale;
        }

        /** @var bool $denoise */
        $denoise = $data['denoise'] ?? false;
        if (!is_bool($denoise)) {
            /** @var bool $denoise */
            $denoise = (bool) $denoise;
        }

        /** @var bool $deskew */
        $deskew = $data['deskew'] ?? false;
        if (!is_bool($deskew)) {
            /** @var bool $deskew */
            $deskew = (bool) $deskew;
        }

        /** @var bool $removeBackground */
        $removeBackground = $data['remove_background'] ?? false;
        if (!is_bool($removeBackground)) {
            /** @var bool $removeBackground */
            $removeBackground = (bool) $removeBackground;
        }

        return new self(
            targetDpi: $targetDpi,
            quality: $quality,
            grayscale: $grayscale,
            denoise: $denoise,
            deskew: $deskew,
            removeBackground: $removeBackground,
        );
    }

    /**
     * Create configuration from JSON string.
     */
    public static function fromJson(string $json): self
    {
        $data = json_decode($json, true);
        if (json_last_error() !== JSON_ERROR_NONE) {
            throw new \InvalidArgumentException('Invalid JSON: ' . json_last_error_msg());
        }
        if (!is_array($data)) {
            throw new \InvalidArgumentException('JSON must decode to an object/array');
        }
        /** @var array<string, mixed> $data */
        return self::fromArray($data);
    }

    /**
     * Create configuration from JSON file.
     */
    public static function fromFile(string $path): self
    {
        if (!file_exists($path)) {
            throw new \InvalidArgumentException("File not found: {$path}");
        }
        $contents = file_get_contents($path);
        if ($contents === false) {
            throw new \InvalidArgumentException("Unable to read file: {$path}");
        }
        return self::fromJson($contents);
    }

    /**
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        return [
            'target_dpi' => $this->targetDpi,
            'quality' => $this->quality,
            'grayscale' => $this->grayscale,
            'denoise' => $this->denoise,
            'deskew' => $this->deskew,
            'remove_background' => $this->removeBackground,
        ];
    }

    /**
     * Convert configuration to JSON string.
     */
    public function toJson(): string
    {
        $json = json_encode($this->toArray(), JSON_PRETTY_PRINT);
        if ($json === false) {
            throw new \RuntimeException('Failed to encode configuration to JSON');
        }
        return $json;
    }
}
