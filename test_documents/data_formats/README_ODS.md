# ODS Test Documents

This directory contains OpenDocument Spreadsheet (ODS) files for benchmarking and testing the Kreuzberg document processing library.

## Files

### sample.ods (1.0 KB)
**Type**: Minimal test spreadsheet
**Content**: Basic empty spreadsheet structure created with LibreOffice
**Purpose**: Minimal test case for basic ODS parsing
**Source**: Original fixture

### excel_multi_sheet.ods (9.3 KB)
**Type**: Multi-sheet spreadsheet
**Content**: Excel file with multiple sheets converted to ODS format
**Purpose**: Tests handling of multi-sheet ODS documents
**Source**: Converted from `../xlsx/excel_multi_sheet.xlsx`

### stanley_cups.ods (12 KB)
**Type**: Data-rich spreadsheet
**Content**: Stanley Cup championship data with multiple columns
**Purpose**: Real-world data extraction and benchmarking
**Source**: Converted from `../xlsx/stanley_cups.xlsx`

### test_01.ods (179 KB)
**Type**: Comprehensive test document
**Content**: Complex spreadsheet with embedded objects and multiple sheets
**Purpose**: Stress testing and performance benchmarking with large documents
**Source**: Converted from `../xlsx/test_01.xlsx`

## File Format Verification

All files have been verified to be:
- Valid ZIP archives (ODS files are ZIP-based)
- Proper OpenDocument Spreadsheet format
- Containing required `content.xml` file
- Valid UTF-8 encoded

### Verification Commands

```bash
# Check file type
file *.ods

# Validate ZIP structure
unzip -t *.ods

# List archive contents
unzip -l *.ods
```

## Conversion Process

These ODS files were created by converting existing XLSX test documents using LibreOffice:

```bash
soffice --headless --convert-to ods --outdir . source.xlsx
```

This approach ensures the ODS files are created by the same standards-compliant tool that most users work with, making them representative of real-world ODS documents.

## Size Distribution

- **Minimal**: sample.ods (1.0 KB)
- **Small**: excel_multi_sheet.ods (9.3 KB)
- **Medium**: stanley_cups.ods (12 KB)
- **Large**: test_01.ods (179 KB)

This range provides comprehensive coverage for benchmarking different performance scenarios.

## Usage

These files can be used for:
- Benchmarking ODS parsing performance
- Testing data extraction accuracy
- Validating multi-sheet handling
- Stress testing with various file sizes
- Verifying content extraction consistency

## Notes

- All files are read-only test fixtures
- Do not modify these files in place; copy them for testing
- These are representative of real LibreOffice-generated ODS files
- File sizes reflect actual content, not artificial padding
