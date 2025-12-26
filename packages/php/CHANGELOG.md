# Changelog

All notable changes to the Kreuzberg PHP package will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [4.0.0-rc.20] - 2025-12-26

### Added
- Initial PHP package structure
- Main `Kreuzberg` class for OOP API
- Procedural API functions (`extract_file`, `extract_bytes`, etc.)
- Complete configuration classes:
  - `ExtractionConfig` - Main extraction configuration
  - `OcrConfig` - OCR settings
  - `TesseractConfig` - Tesseract-specific options
  - `ImagePreprocessingConfig` - Image preprocessing options
  - `PdfConfig` - PDF extraction settings
  - `ChunkingConfig` - Text chunking configuration
  - `EmbeddingConfig` - Embedding generation settings
  - `ImageExtractionConfig` - Image extraction settings
  - `PageConfig` - Page extraction settings
  - `LanguageDetectionConfig` - Language detection settings
  - `KeywordConfig` - Keyword extraction settings
- Type-safe result classes:
  - `ExtractionResult` - Main extraction result
  - `Metadata` - Document metadata
  - `Table` - Extracted table structure
  - `Chunk` - Text chunk with embedding
  - `ChunkMetadata` - Chunk offset metadata
  - `ExtractedImage` - Extracted image with OCR
  - `PageContent` - Per-page content
- Exception handling with `KreuzbergException`
- Extension function stubs for IDE support
- Comprehensive README with examples
- Example files demonstrating all features
- PHPStan configuration (level: max)
- PHP CS Fixer configuration
- PHPUnit configuration
- MIT License

### Notes
- Requires PHP 8.2+
- Requires Kreuzberg PHP extension (kreuzberg.so/.dll)
- Optional dependencies: Tesseract OCR, ONNX Runtime
- Full type hints with readonly classes
- PSR-4 autoloading
- Compatible with Kreuzberg 4.0.0-rc.20 core
