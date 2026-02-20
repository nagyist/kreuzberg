//! Bridge between pdfium extraction APIs and the markdown pipeline.
//!
//! Two conversion paths:
//! 1. Structure tree: `ExtractedBlock` → `PdfParagraph` (for tagged PDFs)
//! 2. Page objects: `PdfPage` → `(Vec<SegmentData>, Vec<ImagePosition>)` (heuristic extraction)
//!
//! The page objects path includes post-processing ligature repair for pages
//! with broken font encodings (detected via `PdfPageTextChar::has_unicode_map_error()`).

use crate::pdf::hierarchy::SegmentData;
use pdfium_render::prelude::*;

use super::constants::{MAX_HEADING_WORD_COUNT, MIN_HEADING_FONT_GAP, MIN_HEADING_FONT_RATIO};
use super::types::{PdfLine, PdfParagraph};

// Alias to distinguish from our local PdfParagraph type.
use pdfium_render::prelude::PdfParagraph as PdfiumParagraph;

/// Position and metadata of an image detected during object-based extraction.
#[derive(Debug, Clone)]
pub(super) struct ImagePosition {
    /// 1-indexed page number.
    pub page_number: usize,
    /// Global image index across the document.
    pub image_index: usize,
}

/// Convert extracted blocks from the structure tree API into PdfParagraphs.
///
/// Structure tree heading levels are validated against font size and word count
/// to prevent broken structure trees from marking body text as headings.
pub(super) fn extracted_blocks_to_paragraphs(blocks: &[ExtractedBlock]) -> Vec<PdfParagraph> {
    // First pass: collect font sizes to determine body font size
    let body_font_size = estimate_body_font_size(blocks);

    // Second pass: convert blocks with validated heading levels
    let mut paragraphs = Vec::new();
    convert_blocks(blocks, body_font_size, &mut paragraphs);
    paragraphs
}

/// Recursively estimate the body (most common) font size from all leaf blocks.
fn estimate_body_font_size(blocks: &[ExtractedBlock]) -> f32 {
    let mut sizes: Vec<f32> = Vec::new();
    collect_font_sizes(blocks, &mut sizes);

    if sizes.is_empty() {
        return 12.0;
    }

    super::lines::most_frequent_font_size(sizes.into_iter())
}

fn collect_font_sizes(blocks: &[ExtractedBlock], sizes: &mut Vec<f32>) {
    for block in blocks {
        if !block.children.is_empty() {
            collect_font_sizes(&block.children, sizes);
        } else if !block.text.trim().is_empty() {
            sizes.push(block.font_size.unwrap_or(12.0));
        }
    }
}

/// Recursively convert blocks to paragraphs with heading validation.
fn convert_blocks(blocks: &[ExtractedBlock], body_font_size: f32, paragraphs: &mut Vec<PdfParagraph>) {
    for block in blocks {
        if !block.children.is_empty() {
            convert_blocks(&block.children, body_font_size, paragraphs);
            continue;
        }

        if block.text.trim().is_empty() {
            continue;
        }

        let is_list_item = matches!(&block.role, ContentRole::ListItem { .. });

        let full_text = if let ContentRole::ListItem { label: Some(ref l) } = block.role {
            format!("{} {}", l, block.text)
        } else {
            block.text.clone()
        };

        let font_size = block.font_size.unwrap_or(12.0);
        let word_count = full_text.split_whitespace().count();

        // Validate heading level from structure tree:
        // Only accept if font size is meaningfully larger than body AND word count is low
        let heading_level = match &block.role {
            ContentRole::Heading { level } => {
                let ratio_ok = font_size >= body_font_size * MIN_HEADING_FONT_RATIO;
                let gap_ok = font_size - body_font_size >= MIN_HEADING_FONT_GAP;
                let words_ok = word_count <= MAX_HEADING_WORD_COUNT;
                if (ratio_ok || gap_ok) && words_ok {
                    Some(*level)
                } else {
                    None
                }
            }
            _ => None,
        };

        // Create segments from the block text (one per whitespace-delimited word)
        let segments: Vec<SegmentData> = full_text
            .split_whitespace()
            .map(|w| SegmentData {
                text: w.to_string(),
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
                font_size,
                is_bold: block.is_bold,
                is_italic: block.is_italic,
                is_monospace: false,
                baseline_y: 0.0,
            })
            .collect();

        if segments.is_empty() {
            continue;
        }

        let line = PdfLine {
            segments,
            baseline_y: 0.0,
            dominant_font_size: font_size,
            is_bold: block.is_bold,
            is_monospace: false,
        };

        paragraphs.push(PdfParagraph {
            lines: vec![line],
            dominant_font_size: font_size,
            heading_level,
            is_bold: block.is_bold,
            is_list_item,
            is_code_block: false,
        });
    }
}

/// Extract text segments and image positions from a PDF page.
///
/// Uses the page objects API with column detection for text extraction.
/// For pages with broken font encodings (ligature corruption), applies
/// per-character repair using `PdfPageTextChar::has_unicode_map_error()`.
///
/// Also detects image objects and records their positions for interleaving.
pub(super) fn objects_to_page_data(
    page: &PdfPage,
    page_number: usize,
    image_offset: &mut usize,
) -> (Vec<SegmentData>, Vec<ImagePosition>) {
    let objects: Vec<PdfPageObject> = page.objects().iter().collect();

    // Image scan BEFORE column partitioning (partition consumes the vec).
    let mut images = Vec::new();
    for obj in &objects {
        if obj.as_image_object().is_some() {
            images.push(ImagePosition {
                page_number,
                image_index: *image_offset,
            });
            *image_offset += 1;
        }
    }

    // Extract text via page objects API with column detection.
    // Partition objects into column groups by moving (not cloning) them.
    let mut segments = Vec::new();
    let column_groups = super::columns::split_objects_into_columns(&objects);
    let column_vecs = partition_objects_by_columns(objects, &column_groups);
    for column_objects in &column_vecs {
        let paragraphs: Vec<PdfiumParagraph> = PdfiumParagraph::from_objects(column_objects);
        extract_paragraphs_to_segments(paragraphs, &mut segments);
    }

    // Apply ligature repair using per-char font error detection.
    if let Some(repair_map) = build_ligature_repair_map(page) {
        for seg in &mut segments {
            seg.text = apply_ligature_repairs(&seg.text, &repair_map);
        }
    }

    (segments, images)
}

/// Partition page objects into column groups by moving objects out of the source vec.
///
/// Each column group is a `Vec<usize>` of indices into `objects`. This function
/// consumes the objects vec and returns one `Vec<PdfPageObject>` per column.
fn partition_objects_by_columns<'a>(
    objects: Vec<PdfPageObject<'a>>,
    column_groups: &[Vec<usize>],
) -> Vec<Vec<PdfPageObject<'a>>> {
    if column_groups.len() <= 1 {
        return vec![objects];
    }

    let total = objects.len();
    let num_columns = column_groups.len();
    let mut col_for_obj = vec![0usize; total];
    for (col_idx, group) in column_groups.iter().enumerate() {
        for &obj_idx in group {
            if obj_idx < total {
                col_for_obj[obj_idx] = col_idx;
            }
        }
    }

    let mut result: Vec<Vec<PdfPageObject<'a>>> = (0..num_columns).map(|_| Vec::new()).collect();
    for (i, obj) in objects.into_iter().enumerate() {
        result[col_for_obj[i]].push(obj);
    }

    result
}

/// Build a mapping of corrupted characters → correct ligature expansions for a page.
///
/// Walks the per-character API to find characters with `has_unicode_map_error()`,
/// then determines the correct ligature expansion based on the character's raw
/// unicode value and font-specific encoding patterns.
///
/// Returns `None` if the page has no encoding errors (most pages).
fn build_ligature_repair_map(page: &PdfPage) -> Option<Vec<(char, &'static str)>> {
    let text = match page.text() {
        Ok(t) => t,
        Err(_) => return None,
    };

    let chars = text.chars();
    let char_count = chars.len();
    if char_count == 0 {
        return None;
    }

    let mut repair_map: Vec<(char, &'static str)> = Vec::new();

    for i in 0..char_count {
        let ch = match chars.get(i) {
            Ok(c) => c,
            Err(_) => continue,
        };

        if ch.is_generated().unwrap_or(false) {
            continue;
        }

        if !ch.has_unicode_map_error().unwrap_or(false) {
            continue;
        }

        // Skip symbol/math fonts — their encodings are intentional
        if ch.font_is_symbolic() {
            continue;
        }

        let unicode_val = ch.unicode_value();
        let mapped_char = match char::from_u32(unicode_val) {
            Some(c) => c,
            None => continue,
        };

        // Check if we already have a mapping for this character
        if repair_map.iter().any(|(c, _)| *c == mapped_char) {
            continue;
        }

        // Determine the correct ligature based on raw unicode value.
        // Different fonts encode ligatures at different positions. We check
        // both the low-byte encoding (CM fonts) and ASCII fallback positions.
        let ligature = match unicode_val {
            // Standard Type1/CM ligature positions (low bytes)
            0x0B => "ff",
            0x0C => "fi",
            0x0D => "fl",
            0x0E => "ffi",
            0x0F => "ffl",
            // Alternate low-byte positions used by some fonts
            0x01 => "fi",
            0x02 => "fl",
            0x03 => "ff",
            0x04 => "ffi",
            0x05 => "ffl",
            // When broken CMap maps ligature codes to ASCII positions,
            // we need context from the specific font. Since we can't
            // determine the exact original glyph code, we use the most
            // common mapping for each ASCII character.
            // These are determined by the font encoding, not universal.
            _ => continue,
        };

        repair_map.push((mapped_char, ligature));
    }

    if repair_map.is_empty() { None } else { Some(repair_map) }
}

/// Apply ligature repairs to a text string using a page-specific repair map.
fn apply_ligature_repairs(text: &str, repair_map: &[(char, &str)]) -> String {
    let mut result = String::with_capacity(text.len() + 16);
    for ch in text.chars() {
        if let Some((_, replacement)) = repair_map.iter().find(|(c, _)| *c == ch) {
            result.push_str(replacement);
        } else {
            result.push(ch);
        }
    }
    result
}

/// Convert pdfium paragraphs into SegmentData, preserving per-line positions.
fn extract_paragraphs_to_segments(paragraphs: Vec<PdfiumParagraph>, segments: &mut Vec<SegmentData>) {
    for para in paragraphs {
        for line in para.into_lines() {
            let line_baseline = line.bottom.value;
            let line_left = line.left.value;
            let mut running_x = line_left;

            for fragment in &line.fragments {
                match fragment {
                    PdfParagraphFragment::StyledString(styled) => {
                        let text = normalize_text_encoding(styled.text());
                        if text.trim().is_empty() {
                            continue;
                        }

                        let font_size = styled.font_size().value;
                        let is_bold = styled.is_bold();
                        let is_italic = styled.is_italic();
                        let is_monospace = styled.is_monospace();
                        let estimated_width = text.len() as f32 * font_size * 0.5;

                        segments.push(SegmentData {
                            text,
                            x: running_x,
                            y: line_baseline,
                            width: estimated_width,
                            height: font_size,
                            font_size,
                            is_bold,
                            is_italic,
                            is_monospace,
                            baseline_y: line_baseline,
                        });

                        running_x += estimated_width;
                    }
                    PdfParagraphFragment::NonTextObject(_) | PdfParagraphFragment::LineBreak { .. } => {}
                }
            }
        }
    }
}

/// Normalize text encoding: handle soft hyphens and strip control characters.
///
/// - `\u{00AD}` (soft hyphen) at end of text → replaced with `-` so downstream
///   hyphen-rejoining logic can merge word fragments.
/// - `\u{00AD}` mid-text → removed (invisible break hint).
/// - C0 control characters (U+0000–U+001F except `\t`, `\n`, `\r`) → removed.
fn normalize_text_encoding(text: &str) -> String {
    // Fast path: no special characters present
    if !text.contains('\u{00AD}') && !text.bytes().any(|b| b < 0x20 && b != b'\t' && b != b'\n' && b != b'\r') {
        return text.to_string();
    }

    let mut result = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '\u{00AD}' => {
                // Soft hyphen at end of text (or before whitespace): convert to regular
                // hyphen so rendering code can rejoin word fragments.
                let at_end = chars.peek().is_none_or(|c| c.is_whitespace());
                if at_end {
                    result.push('-');
                }
                // Mid-word soft hyphen: drop (invisible break hint)
            }
            c if c.is_control() && c != '\n' && c != '\r' && c != '\t' => {
                // Strip other control characters
            }
            _ => result.push(ch),
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_block(role: ContentRole, text: &str) -> ExtractedBlock {
        ExtractedBlock {
            role,
            text: text.to_string(),
            bounds: None,
            font_size: Some(12.0),
            is_bold: false,
            is_italic: false,
            children: Vec::new(),
        }
    }

    fn make_block_with_font(role: ContentRole, text: &str, font_size: f32) -> ExtractedBlock {
        ExtractedBlock {
            role,
            text: text.to_string(),
            bounds: None,
            font_size: Some(font_size),
            is_bold: false,
            is_italic: false,
            children: Vec::new(),
        }
    }

    #[test]
    fn test_heading_block() {
        // Heading must have meaningfully larger font than body for validation to pass
        let blocks = vec![
            make_block_with_font(ContentRole::Heading { level: 2 }, "Section Title", 18.0),
            make_block_with_font(ContentRole::Paragraph, "Body text line one", 12.0),
            make_block_with_font(ContentRole::Paragraph, "Body text line two", 12.0),
            make_block_with_font(ContentRole::Paragraph, "Body text line three", 12.0),
        ];
        let paragraphs = extracted_blocks_to_paragraphs(&blocks);
        assert_eq!(paragraphs.len(), 4);
        assert_eq!(paragraphs[0].heading_level, Some(2));
    }

    #[test]
    fn test_heading_rejected_when_same_font_as_body() {
        // Heading with same font size as body should be rejected
        let blocks = vec![
            make_block(ContentRole::Heading { level: 3 }, "Not really a heading"),
            make_block(ContentRole::Paragraph, "Body text"),
            make_block(ContentRole::Paragraph, "More body text"),
        ];
        let paragraphs = extracted_blocks_to_paragraphs(&blocks);
        assert_eq!(paragraphs.len(), 3);
        assert_eq!(paragraphs[0].heading_level, None); // Rejected: same font size
    }

    #[test]
    fn test_body_block() {
        let blocks = vec![make_block(ContentRole::Paragraph, "Body text")];
        let paragraphs = extracted_blocks_to_paragraphs(&blocks);
        assert_eq!(paragraphs.len(), 1);
        assert_eq!(paragraphs[0].heading_level, None);
        assert!(!paragraphs[0].is_list_item);
    }

    #[test]
    fn test_list_item_block() {
        let blocks = vec![ExtractedBlock {
            role: ContentRole::ListItem {
                label: Some("1.".to_string()),
            },
            text: "First item".to_string(),
            bounds: None,
            font_size: Some(12.0),
            is_bold: false,
            is_italic: false,
            children: Vec::new(),
        }];
        let paragraphs = extracted_blocks_to_paragraphs(&blocks);
        assert_eq!(paragraphs.len(), 1);
        assert!(paragraphs[0].is_list_item);
        // Check that the label is prepended
        let first_seg_text = &paragraphs[0].lines[0].segments[0].text;
        assert_eq!(first_seg_text, "1.");
    }

    #[test]
    fn test_normalize_plain_text_unchanged() {
        assert_eq!(normalize_text_encoding("hello world"), "hello world");
    }

    #[test]
    fn test_normalize_trailing_soft_hyphen() {
        assert_eq!(normalize_text_encoding("soft\u{00AD}"), "soft-");
    }

    #[test]
    fn test_normalize_mid_word_soft_hyphen_removed() {
        assert_eq!(normalize_text_encoding("soft\u{00AD}ware"), "software");
    }

    #[test]
    fn test_normalize_soft_hyphen_before_space() {
        assert_eq!(normalize_text_encoding("soft\u{00AD} ware"), "soft- ware");
    }

    #[test]
    fn test_normalize_strips_control_chars() {
        assert_eq!(normalize_text_encoding("he\x01llo\x02"), "hello");
    }

    #[test]
    fn test_normalize_preserves_tabs_newlines() {
        assert_eq!(normalize_text_encoding("a\tb\nc\r"), "a\tb\nc\r");
    }

    #[test]
    fn test_empty_text_skipped() {
        let blocks = vec![make_block(ContentRole::Paragraph, "")];
        let paragraphs = extracted_blocks_to_paragraphs(&blocks);
        assert!(paragraphs.is_empty());
    }

    #[test]
    fn test_whitespace_only_skipped() {
        let blocks = vec![make_block(ContentRole::Paragraph, "   ")];
        let paragraphs = extracted_blocks_to_paragraphs(&blocks);
        assert!(paragraphs.is_empty());
    }

    #[test]
    fn test_children_processed() {
        let blocks = vec![ExtractedBlock {
            role: ContentRole::Other("Table".to_string()),
            text: String::new(),
            bounds: None,
            font_size: None,
            is_bold: false,
            is_italic: false,
            children: vec![
                make_block(ContentRole::Paragraph, "Cell 1"),
                make_block(ContentRole::Paragraph, "Cell 2"),
            ],
        }];
        let paragraphs = extracted_blocks_to_paragraphs(&blocks);
        assert_eq!(paragraphs.len(), 2);
    }

    #[test]
    fn test_apply_ligature_repairs_fi() {
        let map = vec![('\x0C', "fi")];
        assert_eq!(apply_ligature_repairs("classi\x0Ccation", &map), "classification");
    }

    #[test]
    fn test_apply_ligature_repairs_ff() {
        let map = vec![('\x0B', "ff")];
        assert_eq!(apply_ligature_repairs("e\x0Bective", &map), "effective");
    }

    #[test]
    fn test_apply_ligature_repairs_fl() {
        let map = vec![('\x0D', "fl")];
        assert_eq!(apply_ligature_repairs("re\x0Dection", &map), "reflection");
    }

    #[test]
    fn test_apply_ligature_repairs_ffi() {
        let map = vec![('\x0E', "ffi")];
        assert_eq!(apply_ligature_repairs("e\x0Ecient", &map), "efficient");
    }

    #[test]
    fn test_apply_ligature_repairs_ffl() {
        let map = vec![('\x0F', "ffl")];
        assert_eq!(apply_ligature_repairs("ba\x0Fe", &map), "baffle");
    }

    #[test]
    fn test_apply_ligature_repairs_no_map() {
        let map: Vec<(char, &str)> = Vec::new();
        assert_eq!(apply_ligature_repairs("hello world!", &map), "hello world!");
    }

    #[test]
    fn test_apply_ligature_repairs_multiple() {
        let map = vec![('\x0C', "fi"), ('\x0E', "ffi")];
        assert_eq!(
            apply_ligature_repairs("e\x0Ecient and classi\x0Ccation", &map),
            "efficient and classification"
        );
    }
}
