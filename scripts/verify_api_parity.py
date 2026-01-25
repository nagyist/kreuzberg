#!/usr/bin/env python3
"""
Verify API parity across all language bindings.

This script extracts the field list from each language binding and compares
them against the Rust core ExtractionConfig struct to ensure all bindings
expose the same parameters.

This validator focuses on MISSING fields (critical failures) and is more lenient
with extra fields that may be implementation-specific (like accessor methods).
"""

import re
import sys
import json
from pathlib import Path
from typing import Dict, Set, Optional, Tuple
from dataclasses import dataclass


@dataclass
class ValidationResult:
    """Result of API parity validation."""
    language: str
    has_parity: bool
    missing_fields: Set[str]
    extra_fields: Set[str]
    errors: list[str]


class APIParityValidator:
    """Validator for API parity across language bindings."""

    def __init__(self, repo_root: Path):
        self.repo_root = repo_root
        self.rust_fields = set()
        self.results: Dict[str, ValidationResult] = {}

    def extract_rust_fields(self) -> Set[str]:
        """Extract field names from Rust ExtractionConfig."""
        rust_file = self.repo_root / "crates/kreuzberg/src/core/config/extraction/core.rs"

        if not rust_file.exists():
            raise FileNotFoundError(f"Rust config file not found: {rust_file}")

        content = rust_file.read_text()

        # Extract pub fields from the struct
        # Match patterns like: pub use_cache: bool,
        pattern = r'pub (\w+):'
        fields = set(re.findall(pattern, content))

        if not fields:
            raise ValueError("No fields found in Rust ExtractionConfig")

        return fields

    def extract_typescript_fields(self) -> Tuple[Set[str], list[str]]:
        """Extract field names from TypeScript ExtractionConfig interface."""
        errors = []
        ts_file = self.repo_root / "packages/typescript/core/src/types/config.ts"

        if not ts_file.exists():
            errors.append(f"TypeScript config file not found: {ts_file}")
            return set(), errors

        content = ts_file.read_text()

        # Find the ExtractionConfig interface
        pattern = r'export interface ExtractionConfig\s*\{([^}]+)\}'
        match = re.search(pattern, content, re.DOTALL)

        if not match:
            errors.append("ExtractionConfig interface not found")
            return set(), errors

        interface_body = match.group(1)

        # Extract field names (including optional ones with ?)
        # Match patterns like: useCache?: boolean; or outputFormat?: string;
        # Exclude methods (toJson, getField, merge, etc.)
        field_pattern = r'(\w+)\??\s*:\s*(?!.*\()'  # Negative lookahead to exclude methods
        fields = set()
        for match in re.finditer(field_pattern, interface_body):
            field_name = match.group(1)
            # Exclude method names
            if field_name not in ['toJson', 'getField', 'merge', 'config', 'content', 'default']:
                fields.add(field_name)

        # Convert camelCase to snake_case for comparison
        fields = {self._camel_to_snake(f) for f in fields}

        return fields, errors

    def extract_python_fields(self) -> Tuple[Set[str], list[str]]:
        """Extract field names from Python type stubs."""
        errors = []
        pyi_file = self.repo_root / "packages/python/kreuzberg/_internal_bindings.pyi"

        if not pyi_file.exists():
            errors.append(f"Python pyi file not found: {pyi_file}")
            return set(), errors

        content = pyi_file.read_text()

        # Find the ExtractionConfig class
        pattern = r'class ExtractionConfig:(.+?)(?=\nclass |\Z)'
        match = re.search(pattern, content, re.DOTALL)

        if not match:
            errors.append("ExtractionConfig class not found")
            return set(), errors

        class_body = match.group(1)

        fields = set()

        # Extract from docstring attributes section
        attr_section_pattern = r'Attributes:(.+?)(?=\n\n|\nclass |\Z)'
        attr_match = re.search(attr_section_pattern, class_body, re.DOTALL)

        if attr_match:
            attr_text = attr_match.group(1)
            # Match patterns like "use_cache (bool):"
            attr_pattern = r'(\w+)\s*\([^)]+\):'
            fields.update(re.findall(attr_pattern, attr_text))

        # Also look for @property decorators
        prop_pattern = r'@property\s+def (\w+)\('
        fields.update(re.findall(prop_pattern, class_body))

        return fields, errors

    def extract_go_fields(self) -> Tuple[Set[str], list[str]]:
        """Extract field names from Go ExtractionConfig struct."""
        errors = []
        go_file = self.repo_root / "packages/go/v4/config_types.go"

        if not go_file.exists():
            errors.append(f"Go config file not found: {go_file}")
            return set(), errors

        content = go_file.read_text()

        # Find the ExtractionConfig struct
        pattern = r'type ExtractionConfig struct\s*\{([^}]+)\}'
        match = re.search(pattern, content, re.DOTALL)

        if not match:
            errors.append("ExtractionConfig struct not found")
            return set(), errors

        struct_body = match.group(1)

        # Extract field names - Go uses PascalCase, convert to snake_case
        # Match patterns like: UseCache *bool `json:"use_cache,omitempty"`
        field_pattern = r'(\w+)\s+\*?\w+\s+`json:"(\w+),'
        matches = re.findall(field_pattern, struct_body)

        # Use the JSON tag names for comparison (they're snake_case)
        fields = {json_name for _, json_name in matches}

        return fields, errors

    def extract_ruby_fields(self) -> Tuple[Set[str], list[str]]:
        """Extract field names from Ruby Config::Extraction class."""
        errors = []
        ruby_file = self.repo_root / "packages/ruby/lib/kreuzberg/config.rb"

        if not ruby_file.exists():
            errors.append(f"Ruby config file not found: {ruby_file}")
            return set(), errors

        content = ruby_file.read_text()

        # Find the Extraction class specifically
        pattern = r'class Extraction\s+(.*?)(?=\n\s{4}class |\n\s{2}end\s*$|\Z)'
        match = re.search(pattern, content, re.DOTALL | re.MULTILINE)

        if not match:
            errors.append("Extraction class not found")
            return set(), errors

        class_body = match.group(1)

        # Extract from attr_reader declarations
        # Handle multi-line attr_reader like:
        # attr_reader :use_cache, :enable_quality_processing, :force_ocr,
        #             :ocr, :chunking, :language_detection, :pdf_options,
        attr_pattern = r'attr_reader\s+((?::[\w_]+,?\s*)+)'
        attr_matches = re.findall(attr_pattern, class_body)

        fields = set()
        for attr_list in attr_matches:
            # Extract all symbol names
            symbols = re.findall(r':(\w+)', attr_list)
            fields.update(symbols)

        return fields, errors

    def extract_php_fields(self) -> Tuple[Set[str], list[str]]:
        """Extract field names from PHP ExtractionConfig class."""
        errors = []
        php_file = self.repo_root / "packages/php/src/Config/ExtractionConfig.php"

        if not php_file.exists():
            errors.append(f"PHP config file not found: {php_file}")
            return set(), errors

        content = php_file.read_text()

        # Find the class definition
        pattern = r'class ExtractionConfig(.+?)(?=\nclass |\nfinal |\Z)'
        match = re.search(pattern, content, re.DOTALL)

        if not match:
            errors.append("ExtractionConfig class not found")
            return set(), errors

        class_body = match.group(1)

        # Look for public properties or __construct parameters
        fields = set()

        # Match public properties
        prop_pattern = r'public\s+(?:\?)?[\w\\]+\s+\$(\w+)'
        fields.update(re.findall(prop_pattern, class_body))

        # Also extract from constructor parameters
        construct_pattern = r'public\s+function\s+__construct\((.*?)\)'
        construct_match = re.search(construct_pattern, class_body, re.DOTALL)

        if construct_match:
            params = construct_match.group(1)
            # Match patterns like: ?OcrConfig $ocr = null
            param_pattern = r'\$(\w+)'
            param_fields = set(re.findall(param_pattern, params))
            fields.update(param_fields)

        # Convert camelCase properties to snake_case for comparison
        fields = {self._camel_to_snake(f) for f in fields}

        return fields, errors

    def extract_java_fields(self) -> Tuple[Set[str], list[str]]:
        """Extract field names from Java ExtractionConfig class."""
        errors = []
        java_file = (
            self.repo_root / "packages/java/src/main/java/dev/kreuzberg/config/ExtractionConfig.java"
        )

        if not java_file.exists():
            errors.append(f"Java config file not found: {java_file}")
            return set(), errors

        content = java_file.read_text()

        # Find the class definition
        pattern = r'public class ExtractionConfig(.+?)(?=\nclass |\Z)'
        match = re.search(pattern, content, re.DOTALL)

        if not match:
            errors.append("ExtractionConfig class not found")
            return set(), errors

        class_body = match.group(1)

        # Match private fields (not getter/setter methods)
        # Pattern: private OcrConfig ocr;
        field_pattern = r'private\s+(?:final\s+)?[\w<>]+\s+(\w+)\s*;'
        fields = set(re.findall(field_pattern, class_body))

        # Convert camelCase to snake_case
        fields = {self._camel_to_snake(f) for f in fields}

        return fields, errors

    def extract_csharp_fields(self) -> Tuple[Set[str], list[str]]:
        """Extract field names from C# ExtractionConfig class."""
        errors = []
        csharp_file = self.repo_root / "packages/csharp/Kreuzberg/Models.cs"

        if not csharp_file.exists():
            errors.append(f"C# models file not found: {csharp_file}")
            return set(), errors

        content = csharp_file.read_text()

        # Find the ExtractionConfig class
        pattern = r'public sealed class ExtractionConfig\s*\{([^}]+?(?:public\s+[\w?]+\s+\w+\s*\{[^}]*\})*[^}]*?)\}'
        match = re.search(pattern, content, re.DOTALL)

        if not match:
            errors.append("ExtractionConfig class not found in Models.cs")
            return set(), errors

        class_body = match.group(1)

        # Match public properties
        # Patterns like: public string? UseCache { get; set; }
        prop_pattern = r'public\s+\w+\??\s+(\w+)\s*\{'
        fields = set(re.findall(prop_pattern, class_body))

        # Convert PascalCase to snake_case
        fields = {self._camel_to_snake(f) for f in fields}

        return fields, errors

    def extract_wasm_fields(self) -> Tuple[Set[str], list[str]]:
        """Extract field names from WebAssembly (same as TypeScript)."""
        # WASM uses the same TypeScript definitions
        return self.extract_typescript_fields()

    def extract_elixir_fields(self) -> Tuple[Set[str], list[str]]:
        """Extract field names from Elixir config."""
        errors = []
        elixir_file = self.repo_root / "packages/elixir/lib/kreuzberg/config.ex"

        if not elixir_file.exists():
            errors.append(f"Elixir config file not found: {elixir_file}")
            return set(), errors

        content = elixir_file.read_text()

        # Look for defstruct fields
        defstruct_pattern = r'defstruct\s*\[\s*([^\]]+)\s*\]'
        match = re.search(defstruct_pattern, content)

        if match:
            struct_def = match.group(1)
            # Extract field names from defstruct
            # Patterns like: :use_cache or use_cache: nil or :use_cache => true
            field_pattern = r':(\w+)|(\w+):'
            matches = re.findall(field_pattern, struct_def)
            fields = set()
            for match_group in matches:
                # match_group is a tuple: (symbol_name, keyword_name)
                fields.add(match_group[0] or match_group[1])
        else:
            errors.append("defstruct not found")
            fields = set()

        return fields, errors

    def validate_all(self) -> Dict[str, ValidationResult]:
        """Validate API parity across all language bindings."""
        # Extract Rust fields first
        try:
            self.rust_fields = self.extract_rust_fields()
            print(f"Rust ExtractionConfig fields ({len(self.rust_fields)}): {sorted(self.rust_fields)}")
        except (FileNotFoundError, ValueError) as e:
            print(f"ERROR: Failed to extract Rust fields: {e}")
            return {}

        # Define extractors for each language
        extractors = {
            "TypeScript": self.extract_typescript_fields,
            "Go": self.extract_go_fields,
            "Python": self.extract_python_fields,
            "Ruby": self.extract_ruby_fields,
            "PHP": self.extract_php_fields,
            "Java": self.extract_java_fields,
            "C#": self.extract_csharp_fields,
            "WASM": self.extract_wasm_fields,
            "Elixir": self.extract_elixir_fields,
        }

        # Validate each language
        for language, extractor in extractors.items():
            try:
                fields, errors = extractor()

                missing = self.rust_fields - fields
                extra = fields - self.rust_fields

                # Only fail if there are MISSING fields
                # Extra fields (getter methods, implementation details) are acceptable
                has_parity = len(missing) == 0

                result = ValidationResult(
                    language=language,
                    has_parity=has_parity,
                    missing_fields=missing,
                    extra_fields=extra,
                    errors=errors,
                )
                self.results[language] = result

                print(f"\n{language}:")
                print(f"  Fields extracted: {len(fields)}")
                if missing:
                    print(f"  MISSING (critical): {sorted(missing)}")
                if len(extra) <= 5:
                    if extra:
                        print(f"  Extra fields: {sorted(extra)}")
                else:
                    print(f"  Extra fields: {len(extra)} (likely implementation details)")
                if errors:
                    for error in errors:
                        print(f"  ERROR: {error}")
                if has_parity:
                    print(f"  Status: API parity OK")

            except Exception as e:
                result = ValidationResult(
                    language=language,
                    has_parity=False,
                    missing_fields=set(),
                    extra_fields=set(),
                    errors=[str(e)],
                )
                self.results[language] = result
                print(f"\n{language}: ERROR - {e}")

        return self.results

    def report(self) -> bool:
        """Generate validation report and return True if all bindings have parity."""
        print("\n" + "=" * 80)
        print("API PARITY VALIDATION REPORT")
        print("=" * 80)

        all_ok = True
        for result in self.results.values():
            if not result.has_parity:
                all_ok = False
                status = "FAILED"
            else:
                status = "PASSED"

            print(f"\n{result.language}: {status}")
            if result.errors:
                for error in result.errors:
                    print(f"  Error: {error}")
            if result.missing_fields:
                print(f"  Missing required fields: {sorted(result.missing_fields)}")

        print("\n" + "=" * 80)
        if all_ok:
            print("All language bindings expose the required parameters")
            return True
        else:
            print("Some bindings are missing required API parameters")
            return False

    @staticmethod
    def _camel_to_snake(name: str) -> str:
        """Convert camelCase to snake_case."""
        # Insert underscore before uppercase letters (except first char)
        s1 = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
        # Insert underscore before uppercase letters preceded by lowercase
        return re.sub('([a-z0-9])([A-Z])', r'\1_\2', s1).lower()


def main():
    """Main entry point."""
    repo_root = Path(__file__).parent.parent

    validator = APIParityValidator(repo_root)
    validator.validate_all()
    success = validator.report()

    sys.exit(0 if success else 1)


if __name__ == "__main__":
    main()
