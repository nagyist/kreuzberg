<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * Configuration for document extraction.
 *
 * @example
 * ```php
 * use Kreuzberg\Config\ExtractionConfig;
 * use Kreuzberg\Config\OcrConfig;
 * use Kreuzberg\Config\PdfConfig;
 * use Kreuzberg\Config\ChunkingConfig;
 *
 * $config = new ExtractionConfig(
 *     ocr: new OcrConfig(backend: 'tesseract', language: 'eng'),
 *     pdf: new PdfConfig(extractImages: true),
 *     chunking: new ChunkingConfig(maxChunkSize: 1000),
 * );
 * ```
 */
readonly class ExtractionConfig
{
    public function __construct(
        public ?OcrConfig $ocr = null,
        public ?PdfConfig $pdf = null,
        public ?ChunkingConfig $chunking = null,
        public ?EmbeddingConfig $embedding = null,
        public ?ImageExtractionConfig $imageExtraction = null,
        public ?PageConfig $page = null,
        public ?LanguageDetectionConfig $languageDetection = null,
        public ?KeywordConfig $keyword = null,
        public bool $extractImages = false,
        public bool $extractTables = true,
        public bool $preserveFormatting = false,
        public ?string $outputFormat = null,
    ) {
    }

    /**
     * Convert configuration to array for FFI.
     *
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        return array_filter([
            'ocr' => $this->ocr?->toArray(),
            'pdf' => $this->pdf?->toArray(),
            'chunking' => $this->chunking?->toArray(),
            'embedding' => $this->embedding?->toArray(),
            'image_extraction' => $this->imageExtraction?->toArray(),
            'page' => $this->page?->toArray(),
            'language_detection' => $this->languageDetection?->toArray(),
            'keyword' => $this->keyword?->toArray(),
            'extract_images' => $this->extractImages,
            'extract_tables' => $this->extractTables,
            'preserve_formatting' => $this->preserveFormatting,
            'output_format' => $this->outputFormat,
        ], static fn ($value): bool => $value !== null);
    }
}
