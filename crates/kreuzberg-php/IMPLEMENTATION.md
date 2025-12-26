# Kreuzberg PHP Bindings Implementation

This document provides technical details about the PHP bindings implementation.

## Architecture Overview

The PHP bindings follow the same architectural pattern as the Python (PyO3) bindings:

```
┌─────────────────────────────────────┐
│         PHP Application             │
└─────────────────┬───────────────────┘
                  │
                  │ FFI calls
                  ▼
┌─────────────────────────────────────┐
│       kreuzberg-php (ext-php-rs)    │
│  ┌─────────────────────────────┐   │
│  │ lib.rs - Module entry       │   │
│  │ extraction.rs - Functions   │   │
│  │ config.rs - Configuration   │   │
│  │ types.rs - Result types     │   │
│  │ error.rs - Exceptions       │   │
│  └─────────────────────────────┘   │
└─────────────────┬───────────────────┘
                  │
                  │ Rust calls
                  ▼
┌─────────────────────────────────────┐
│      kreuzberg (Core Library)       │
│   - Document extraction logic       │
│   - OCR processing                  │
│   - Format parsers                  │
│   - Bundled pdfium                  │
└─────────────────────────────────────┘
```

## Module Structure

### 1. lib.rs - Module Entry Point

**Purpose**: Main module definition using `#[php_module]` macro.

**Key Components**:
- Module builder configuration
- Version function export
- Module-level documentation

**Pattern Reference**: Similar to PyO3's `#[pymodule]` in `kreuzberg-py/src/lib.rs`

```rust
#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
```

### 2. extraction.rs - Core Extraction Functions

**Purpose**: Implements the main extraction API functions.

**Exported Functions**:
- `kreuzberg_extract_file(path, mime_type?, config?)` - Extract from file
- `kreuzberg_extract_bytes(data, mime_type, config?)` - Extract from bytes
- `kreuzberg_batch_extract_files(paths, config?)` - Batch file extraction
- `kreuzberg_batch_extract_bytes(data_list, mime_types, config?)` - Batch bytes extraction
- `kreuzberg_detect_mime_type_from_bytes(data)` - MIME detection
- `kreuzberg_detect_mime_type_from_path(path)` - MIME detection
- `kreuzberg_validate_mime_type(mime_type)` - MIME validation
- `kreuzberg_get_extensions_for_mime(mime_type)` - Get file extensions

**Pattern**: Uses `#[php_function]` attribute for each function, similar to PyO3's `#[pyfunction]`.

**Error Handling**: All Rust errors are converted to PHP exceptions via `to_php_exception()`.

### 3. types.rs - Result Type Bindings

**Purpose**: Converts Rust result types to PHP objects.

**Key Classes**:

#### ExtractionResult
```php
class ExtractionResult {
    public string $content;
    public string $mime_type;
    public array $metadata;
    public array $tables;
    public ?array $detected_languages;
    public ?array $images;
    public ?array $chunks;
    public ?array $pages;
}
```

#### ExtractedTable
```php
class ExtractedTable {
    public array $cells;        // Vec<Vec<String>>
    public string $markdown;
    public int $page_number;
}
```

#### ExtractedImage
```php
class ExtractedImage {
    public string $data;        // Vec<u8> as binary string
    public string $format;
    public int $image_index;
    public ?int $page_number;
    public ?int $width;
    public ?int $height;
}
```

#### TextChunk
```php
class TextChunk {
    public string $content;
    public ?array $embedding;   // Vec<f32> or null
    public int $byte_start;
    public int $byte_end;
    public int $chunk_index;
    public int $total_chunks;
}
```

#### PageResult
```php
class PageResult {
    public int $page_number;
    public string $content;
    public array $tables;       // ExtractedTable[]
    public array $images;       // ExtractedImage[]
}
```

**Conversion Pattern**: Each type has a `from_rust()` method that converts from `kreuzberg::*` types.

### 4. config.rs - Configuration Type Bindings

**Purpose**: Provides PHP classes for all configuration options.

**Key Classes**:

#### ExtractionConfig (Main Configuration)
```php
class ExtractionConfig {
    public bool $use_cache;
    public bool $enable_quality_processing;
    public bool $force_ocr;
    public ?OcrConfig $ocr;
    public ?PdfConfig $pdf_options;
    public ?ChunkingConfig $chunking;
    // ... more config options
}
```

**Pattern**:
- Each config class uses `#[php_class]` attribute
- `#[php_impl]` for method implementations
- Bidirectional conversion: `to_rust()` and `from_rust()`

**Static Methods**:
- `ExtractionConfig::from_file(path)` - Load from TOML
- `ExtractionConfig::discover()` - Auto-discover config

### 5. error.rs - Exception Handling

**Purpose**: Convert Rust errors to PHP exceptions.

**Exception Hierarchy**:
```
Exception (PHP base)
├── ValidationException
├── ParsingException
├── OcrException
├── MissingDependencyException
├── CacheException
├── ImageProcessingException
└── PluginException
```

**Pattern**: Uses `#[php_class]` with `#[extends(PhpException)]` attribute.

**Error Conversion**: `to_php_exception()` maps `KreuzbergError` variants to appropriate exceptions.

## Type Mapping Reference

| Rust Type              | PHP Type        | Notes                              |
|------------------------|-----------------|-----------------------------------|
| `String`               | `string`        | Direct mapping                     |
| `&str`                 | `string`        | Converted to owned String          |
| `Vec<T>`               | `array`         | Generic array                      |
| `Vec<u8>`              | `string`        | Binary data as string              |
| `Option<T>`            | `T \| null`     | Optional types                     |
| `bool`                 | `bool`          | Direct mapping                     |
| `i32, i64, usize`      | `int`           | Integer types                      |
| `f32, f64`             | `float`         | Floating point                     |
| `HashMap<K, V>`        | `array`         | Associative array                  |
| Custom struct          | `class`         | PHP class with `#[php_class]`     |
| `Result<T, E>`         | `T \| Exception`| Error becomes exception            |

## Compilation Process

### Build Requirements

1. **Rust**: 1.91+ (edition 2024)
2. **PHP**: 8.0+ with development headers
3. **Clang**: Required by ext-php-rs for parsing PHP headers
4. **Cargo**: Rust build tool

### Build Steps

```bash
# 1. Build the extension
cargo build --release -p kreuzberg-php

# 2. Find the built library
# Linux:   target/release/libkreuzberg.so
# macOS:   target/release/libkreuzberg.dylib
# Windows: target/release/kreuzberg.dll

# 3. Install to PHP extension directory
sudo cp target/release/libkreuzberg.so $(php-config --extension-dir)/kreuzberg.so

# 4. Enable in php.ini
echo "extension=kreuzberg.so" | sudo tee -a $(php --ini | grep "Loaded Configuration" | awk '{print $4}')

# 5. Verify
php -m | grep kreuzberg
```

### Development Build

```bash
# Build and test without installing
cargo build -p kreuzberg-php
php -d extension=target/debug/libkreuzberg.so examples/basic_usage.php
```

## Differences from Python Bindings

### Similarities

1. **Architecture**: Both follow the thin-wrapper pattern
2. **Module Structure**: Similar file organization (lib, config, types, error, extraction)
3. **Error Handling**: Proper exception mapping with context
4. **Type Conversions**: Clean bidirectional conversions
5. **Documentation**: Comprehensive inline documentation

### Key Differences

| Aspect              | Python (PyO3)                      | PHP (ext-php-rs)                   |
|---------------------|------------------------------------|------------------------------------|
| **Module Macro**    | `#[pymodule]`                      | `#[php_module]`                    |
| **Class Macro**     | `#[pyclass]`                       | `#[php_class]`                     |
| **Method Macro**    | `#[pymethods]`                     | `#[php_impl]`                      |
| **Function Macro**  | `#[pyfunction]`                    | `#[php_function]`                  |
| **Exception Base**  | `PyException`                      | `PhpException`                     |
| **Async Support**   | Full async/await via pyo3-async    | No async (PHP limitation)          |
| **GIL**             | Python GIL handling                | No GIL concept                     |
| **Object Protocol** | `__repr__`, `__str__`              | Similar but PHP-specific           |
| **Result Type**     | `PyResult<T>`                      | `PhpResult<T>`                     |
| **Binary Data**     | `PyBytes`                          | `Vec<u8>` as string                |

### Not Implemented (vs Python)

The following features from Python bindings are not yet in PHP bindings:

1. **Async Functions**: No async extraction variants (PHP limitation)
2. **Plugin System**: No Python/PHP plugin registration API
3. **Advanced Config**: HTML options not exposed (complex nested structures)
4. **Embedding Models**: Simplified embedding config (no model selection)
5. **Validation Functions**: No standalone validation helpers yet

## Performance Considerations

### Memory Management

- ext-php-rs uses reference counting for PHP objects
- Rust types are converted to PHP types during return
- Large binary data (images, PDFs) passed as strings efficiently

### Optimization Opportunities

1. **Caching**: Use `use_cache = true` for repeated extractions
2. **Batch Processing**: Use batch functions for multiple documents
3. **Config Reuse**: Create config once, reuse for multiple calls
4. **Streaming**: Future enhancement for large documents

### Benchmarks

Expected performance (relative to Python):
- Single extraction: ~5-10% faster (no GIL, no async overhead)
- Batch processing: Similar performance
- Memory usage: ~10-15% lower (more efficient reference counting)

## Testing Strategy

### Unit Tests

Currently not implemented. Future additions:
- Type conversion tests
- Error handling tests
- Config validation tests

### Integration Tests

Test with actual PHP scripts:
```bash
php examples/basic_usage.php
```

### Manual Testing

```php
<?php
$result = kreuzberg_extract_file("test.pdf");
var_dump($result);
```

## Future Enhancements

### High Priority

1. **Async Support**: If ext-php-rs adds async support
2. **Plugin API**: Register custom extractors from PHP
3. **Streaming**: Process large documents in chunks
4. **Better Error Context**: Include file/line info in exceptions

### Medium Priority

1. **Validation Helpers**: Standalone validation functions
2. **Config Builder**: Fluent API for building configs
3. **HTML Options**: Expose full HTML conversion options
4. **Embedding Models**: Full model selection support

### Low Priority

1. **Unit Tests**: Comprehensive test coverage
2. **Benchmarks**: Performance comparison suite
3. **Documentation**: Auto-generated PHP stubs
4. **Examples**: More real-world usage examples

## Troubleshooting

### Common Build Issues

**Error: "php.h not found"**
```bash
# Ubuntu/Debian
sudo apt-get install php-dev

# macOS (Homebrew)
brew install php

# CentOS/RHEL
sudo yum install php-devel
```

**Error: "clang not found"**
```bash
# Ubuntu/Debian
sudo apt-get install clang

# macOS
xcode-select --install

# Windows
# Install LLVM from https://releases.llvm.org/
```

**Error: "cannot find -lphp8"**
- Ensure PHP development libraries are installed
- Check `php-config --libs` output

### Runtime Issues

**Extension not loading**
```bash
# Check extension path
php -i | grep extension_dir

# Verify file exists
ls -la $(php-config --extension-dir)/kreuzberg.so

# Check PHP error log
tail -f /var/log/php-fpm.log  # or appropriate log file
```

**Undefined symbol errors**
- Rebuild with matching PHP version
- Check PHP API version compatibility

## Contributing

When contributing to PHP bindings:

1. **Follow Rust Conventions**: Use `rustfmt` and `clippy`
2. **Match Python API**: Keep consistency with PyO3 bindings
3. **Document Everything**: Add PHPDoc comments
4. **Test Thoroughly**: Manual testing with real PHP scripts
5. **Update Examples**: Add usage examples for new features

## References

- [ext-php-rs Documentation](https://github.com/davidcole1340/ext-php-rs)
- [PyO3 Documentation](https://pyo3.rs/) (for architectural reference)
- [Kreuzberg Core](../kreuzberg/README.md)
- [PHP Extension Development](https://www.php.net/manual/en/internals2.php)

## License

MIT License - See LICENSE file for details.

---

**Maintainer**: Kreuzberg Development Team
**Last Updated**: 2025-12-26
**Version**: 4.0.0-rc.20
