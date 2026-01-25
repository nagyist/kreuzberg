# API Consistency Validator

This document describes the automated API consistency validator that ensures all language bindings expose the same parameters as the Rust core library.

## Overview

The **API Consistency Validator** is a Python script that:

1. **Extracts API fields** from the Rust core `ExtractionConfig` struct
2. **Compares against all language bindings** (TypeScript, Go, Python, Ruby, PHP, Java, C#, Elixir, WASM)
3. **Reports missing fields** that break API parity across ecosystems
4. **Runs automatically** in CI on every pull request and push to ensure consistency

## Running Locally

### Basic Usage

```bash
python scripts/verify_api_parity.py
```

This will:
- Extract field definitions from each language binding
- Compare them against the Rust core reference
- Display a detailed report with any missing or extra fields
- Exit with code 0 if all bindings have parity, 1 if any are missing fields

### Expected Output

```
Rust ExtractionConfig fields (16): ['chunking', 'enable_quality_processing', 'force_ocr', ...]

TypeScript:
  Fields extracted: 18
  MISSING (critical): ['result_format']
  Extra fields: ['embedding', 'hierarchy', 'image_preprocessing']

Go:
  Fields extracted: 16
  Status: API parity OK

...

================================================================================
API PARITY VALIDATION REPORT
================================================================================

TypeScript: FAILED
  Missing required fields: ['result_format']

Go: PASSED
...
```

## CI Integration

The validator runs automatically on:

- **Pull requests** when any config files are modified
- **Pushes to main** and feature branches when config files are modified

### Workflow File

The GitHub Actions workflow is defined in `.github/workflows/api-consistency.yaml`:

- Triggers on changes to any config file
- Runs on Ubuntu latest
- Executes the validation script
- Fails the CI if any bindings are missing required fields

### Workflow Triggers

The workflow monitors these files for changes:

**Rust Core:**
- `crates/kreuzberg/src/core/config/extraction/core.rs`

**Language Bindings:**
- `packages/typescript/core/src/types/config.ts`
- `packages/python/kreuzberg/_internal_bindings.pyi`
- `packages/go/v4/config_types.go`
- `packages/ruby/lib/kreuzberg/config.rb`
- `packages/php/src/Config/ExtractionConfig.php`
- `packages/java/src/main/java/dev/kreuzberg/config/ExtractionConfig.java`
- `packages/csharp/Kreuzberg/Models.cs`
- `packages/elixir/lib/kreuzberg/config.ex`

**Tools:**
- `scripts/verify_api_parity.py`
- `.github/workflows/api-consistency.yaml`

## Understanding Results

### PASSED vs FAILED

A binding **PASSES** when it exposes all required fields from the Rust core, even if it has extra fields (which are typically implementation details like getter methods).

A binding **FAILS** when it's missing any required fields from the Rust core.

### Field Mapping

Fields are normalized across languages:

- **Rust**: `snake_case` (e.g., `use_cache`)
- **TypeScript/JavaScript**: `camelCase` (e.g., `useCache`) → converted to `snake_case` for comparison
- **Go**: JSON tags in `snake_case` (e.g., `use_cache`) → used directly
- **Python**: `snake_case` (e.g., `use_cache`)
- **Ruby**: `snake_case` with `attr_reader` (e.g., `use_cache`)
- **PHP**: camelCase properties (e.g., `useCache`) → converted to `snake_case`
- **Java**: camelCase fields (e.g., `useCache`) → converted to `snake_case`
- **C#**: PascalCase properties (e.g., `UseCache`) → converted to `snake_case`
- **Elixir**: atoms (e.g., `:use_cache`)

### Common Issues

**Missing Fields in a Binding:**

If a binding is missing a field, you must add it:

1. **Identify the missing field** from the error report
2. **Add the field** to the binding's config class/interface
3. **Update any serialization** code to handle the new field
4. **Test the binding** locally
5. **Run the validator** to confirm parity is restored

**Example**: TypeScript is missing `result_format`

```typescript
// In packages/typescript/core/src/types/config.ts
export interface ExtractionConfig {
  // ... other fields
  resultFormat?: "unified" | "element_based";  // ADD THIS
  // ... methods
}
```

**Extra Fields:**

Extra fields are allowed if they are:
- Getter/accessor methods
- Utility methods
- Language-specific implementation details
- Configuration sub-objects

Extra fields only trigger a failure if they appear INSTEAD of required fields.

## Validator Architecture

### Extraction Strategy

Each language binding has a custom extractor that:

1. **Locates** the configuration file for that language
2. **Parses** the file using language-appropriate patterns (regex, AST)
3. **Extracts** field names
4. **Normalizes** names to snake_case
5. **Returns** a set of field names

### Supported Patterns

- **Rust**: Public struct fields with `pub` keyword
- **TypeScript**: Interface properties with `?:` or `: type`
- **Go**: Struct fields with JSON tags
- **Python**: Docstring attributes section and @property decorators
- **Ruby**: `attr_reader` declarations
- **PHP**: Public properties and constructor parameters
- **Java**: Private fields declared with `private` keyword
- **C#**: Public properties with `{ get; set; }`
- **Elixir**: `defstruct` field list

## Maintenance

### When to Update Extractors

- Language binding config file structure changes
- Field naming conventions change
- Config class/interface location moves
- Parser no longer accurately extracts fields

### Testing Extractors

To test a specific extractor:

```python
from pathlib import Path
from scripts.verify_api_parity import APIParityValidator

validator = APIParityValidator(Path("."))
fields, errors = validator.extract_go_fields()
print(f"Go fields: {fields}")
print(f"Errors: {errors}")
```

### Adding a New Language Binding

1. **Create an extractor method** in `APIParityValidator`
   ```python
   def extract_newlang_fields(self) -> Tuple[Set[str], list[str]]:
       # ... implementation
   ```

2. **Add to extractors dict** in `validate_all()`
   ```python
   extractors = {
       # ...
       "NewLang": self.extract_newlang_fields,
   }
   ```

3. **Update workflow** to monitor the binding's config file
4. **Test locally** before committing

## Debugging

### Enable verbose output

Add print statements in extraction methods:

```python
print(f"Extracted from {language}: {fields}")
```

### Test individual extractors

```bash
python3 << 'EOF'
from pathlib import Path
from scripts.verify_api_parity import APIParityValidator

v = APIParityValidator(Path("."))
fields, errors = v.extract_ruby_fields()
print(f"Ruby fields: {sorted(fields)}")
for err in errors:
    print(f"  Error: {err}")
EOF
```

### Inspect parsed files

Look at the actual config files to understand their structure:

```bash
# Rust
grep "pub " crates/kreuzberg/src/core/config/extraction/core.rs | head -20

# TypeScript
grep "^\s*\w*:" packages/typescript/core/src/types/config.ts | head -20

# Go
grep "^\s*\w* \*" packages/go/v4/config_types.go | head -20

# Ruby
grep "attr_reader" packages/ruby/lib/kreuzberg/config.rb
```

## Related Documentation

- [API Reference Parity Rule](../CLAUDE.md#api-reference-parity-across-bindings)
- [Polyglot Architecture](../docs/architecture.md)
- Language-specific binding documentation:
  - [TypeScript](../packages/typescript/README.md)
  - [Python](../packages/python/README.md)
  - [Go](../packages/go/README.md)
  - [Ruby](../packages/ruby/README.md)
  - [PHP](../packages/php/README.md)
  - [Java](../packages/java/README.md)
  - [C#](../packages/csharp/README.md)
  - [Elixir](../packages/elixir/README.md)
