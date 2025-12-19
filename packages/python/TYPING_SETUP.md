# Kreuzberg Python Package - Typing Setup

## Overview

The kreuzberg Python package provides comprehensive type hints for all public APIs, enabling:
- Full IDE autocomplete and type checking
- mypy strict mode compliance
- Pylance/Pyright type stubs support

## Components

### 1. py.typed Marker
- **File**: `kreuzberg/py.typed`
- **Purpose**: PEP 561 marker indicating this package has inline type hints
- **Status**: ✓ Included in wheel distribution

### 2. _internal_bindings.pyi Stub
- **File**: `kreuzberg/_internal_bindings.pyi`
- **Purpose**: Type definitions for the compiled Rust module
- **Size**: 44 KB with complete type coverage
- **Status**: ✓ Included in wheel distribution (via build hook)
- **Contents**: Type stubs for ~100+ classes/functions from Rust core

### 3. Python Source Type Hints
- **Location**: All Python modules (`__init__.py`, `types.py`, `exceptions.py`, etc.)
- **Coverage**: 100% on public APIs
- **Style**: Google-style docstrings with type annotations

## Type Coverage

### Exported Types
- **Total Exports**: 67 public items in `__all__`
- **Config Classes**: 15 (ExtractionConfig, OcrConfig, ChunkingConfig, etc.)
- **Result Classes**: 5 (ExtractionResult, Chunk, ExtractedImage, etc.)
- **TypedDicts**: 15 (Metadata, ChunkMetadata, etc. with 68 total fields)
- **Exceptions**: 7 exception classes
- **Protocols**: PostProcessorProtocol, ValidatorProtocol, etc.

### Function Signatures
All public functions have complete type hints:
- Parameters with proper types
- Return types explicitly annotated
- Optional parameters properly typed with `|` union syntax

## Build Process

The `build.py` file contains custom build hooks that ensure:

1. **ensure_stub_file()** - Verifies `.pyi` file is present before build
   - Called before wheel, sdist, and editable builds
   - Fallback creation if file is missing

2. **Integration Points**:
   - `build_wheel()` - Standard wheel builds
   - `build_sdist()` - Source distribution builds
   - `build_editable()` - Development editable installs

## Usage

### For Package Users
```python
from kreuzberg import extract_file_sync, ExtractionConfig, OcrConfig

# IDE will provide:
# - Autocomplete for all imports
# - Type hints on function parameters and returns
# - Inline documentation in hover tooltips

config = ExtractionConfig(ocr=OcrConfig(backend="tesseract"))
result = extract_file_sync("document.pdf", config=config)
```

### For Type Checking
```bash
# mypy strict mode
mypy --strict my_script.py

# pyright
pyright my_script.py

# Both will understand kreuzberg's complete type structure
```

## IDE Integration

### VSCode
1. Install Pylance extension
2. Enable type checking: `"python.analysis.typeCheckingMode": "strict"`
3. Type completions will appear immediately

### PyCharm
1. Settings → Python Integrated Tools → Python Tests
2. Type completions enabled by default
3. All kreuzberg APIs will show proper types

### Vim/Neovim
1. Install pylsp or pyright language server
2. Completions will use type information from `.pyi` files

## Testing Type Coverage

Run type checking on user code:
```bash
mypy --strict --disallow-any-unimported \
  --warn-unreachable \
  --disallow-incomplete-defs \
  my_kreuzberg_script.py
```

Expected: No type errors for correct kreuzberg usage.

## Troubleshooting

If IDE completions not appearing:

1. **Check Installation**
   ```python
   from pathlib import Path
   import kreuzberg
   pyi = Path(kreuzberg.__file__).parent / "_internal_bindings.pyi"
   print(f"Stub file: {pyi.exists()}")  # Should be True
   ```

2. **Verify Package Version**
   ```bash
   pip show kreuzberg  # Should be 4.0.0rc13 or later
   ```

3. **IDE Cache**
   - VSCode: Reload window (Cmd+Shift+P → "Reload Window")
   - PyCharm: Invalidate Caches (File → Invalidate Caches)
   - Vim/Neovim: Restart language server

4. **Python Environment**
   - Ensure kreuzberg is installed in the correct Python
   - Check IDE uses same Python interpreter

## Developer Notes

### Adding New Public APIs
1. Add full type hints to the function/class signature
2. Include docstring with parameter/return type documentation
3. Add to `__all__` in `__init__.py`
4. Update `_internal_bindings.pyi` if it's a Rust-exposed API

### Updating Type Stubs
When Rust APIs change:
1. Rust engineer updates crate
2. Type stub is regenerated from PyO3 macros
3. `build.py` ensures it's in wheel
4. Version bump triggers rebuild

### Testing Types Locally
```bash
# In development
maturin develop  # Rebuilds with stub file

# Type check against development version
mypy --strict my_test.py
```
