// Auto-generated tests for office fixtures.
// Run with: deno test --allow-read

import type { ExtractionResult } from "./helpers.ts";
import { assertions, buildConfig, extractBytes, initWasm, resolveDocument, shouldSkipFixture } from "./helpers.ts";

// Initialize WASM module once at module load time
await initWasm();

Deno.test("office_bibtex_basic", { permissions: { read: true } }, async () => {
	const documentBytes = await resolveDocument("bibtex/comprehensive.bib");
	const config = buildConfig(undefined);
	let result: ExtractionResult | null = null;
	try {
		// Sync file extraction - WASM uses extractBytes with pre-read bytes
		result = await extractBytes(documentBytes, "application/x-bibtex", config);
	} catch (error) {
		if (shouldSkipFixture(error, "office_bibtex_basic", [], undefined)) {
			return;
		}
		throw error;
	}
	if (result === null) {
		return;
	}
	assertions.assertExpectedMime(result, ["application/x-bibtex"]);
	assertions.assertMinContentLength(result, 10);
});

Deno.test("office_doc_legacy", { permissions: { read: true } }, async () => {
	const documentBytes = await resolveDocument("doc/unit_test_lists.doc");
	const config = buildConfig(undefined);
	let result: ExtractionResult | null = null;
	try {
		// Sync file extraction - WASM uses extractBytes with pre-read bytes
		result = await extractBytes(documentBytes, "application/msword", config);
	} catch (error) {
		if (
			shouldSkipFixture(error, "office_doc_legacy", ["libreoffice"], "LibreOffice must be installed for conversion.")
		) {
			return;
		}
		throw error;
	}
	if (result === null) {
		return;
	}
	assertions.assertExpectedMime(result, ["application/msword"]);
	assertions.assertMinContentLength(result, 20);
});

Deno.test("office_docx_basic", { permissions: { read: true } }, async () => {
	const documentBytes = await resolveDocument("docx/sample_document.docx");
	const config = buildConfig(undefined);
	let result: ExtractionResult | null = null;
	try {
		// Sync file extraction - WASM uses extractBytes with pre-read bytes
		result = await extractBytes(
			documentBytes,
			"application/vnd.openxmlformats-officedocument.wordprocessingml.document",
			config,
		);
	} catch (error) {
		if (shouldSkipFixture(error, "office_docx_basic", [], undefined)) {
			return;
		}
		throw error;
	}
	if (result === null) {
		return;
	}
	assertions.assertExpectedMime(result, ["application/vnd.openxmlformats-officedocument.wordprocessingml.document"]);
	assertions.assertMinContentLength(result, 10);
});

Deno.test("office_docx_equations", { permissions: { read: true } }, async () => {
	const documentBytes = await resolveDocument("docx/equations.docx");
	const config = buildConfig(undefined);
	let result: ExtractionResult | null = null;
	try {
		// Sync file extraction - WASM uses extractBytes with pre-read bytes
		result = await extractBytes(
			documentBytes,
			"application/vnd.openxmlformats-officedocument.wordprocessingml.document",
			config,
		);
	} catch (error) {
		if (shouldSkipFixture(error, "office_docx_equations", [], undefined)) {
			return;
		}
		throw error;
	}
	if (result === null) {
		return;
	}
	assertions.assertExpectedMime(result, ["application/vnd.openxmlformats-officedocument.wordprocessingml.document"]);
	assertions.assertMinContentLength(result, 20);
});

Deno.test("office_docx_fake", { permissions: { read: true } }, async () => {
	const documentBytes = await resolveDocument("docx/fake.docx");
	const config = buildConfig(undefined);
	let result: ExtractionResult | null = null;
	try {
		// Sync file extraction - WASM uses extractBytes with pre-read bytes
		result = await extractBytes(
			documentBytes,
			"application/vnd.openxmlformats-officedocument.wordprocessingml.document",
			config,
		);
	} catch (error) {
		if (shouldSkipFixture(error, "office_docx_fake", [], undefined)) {
			return;
		}
		throw error;
	}
	if (result === null) {
		return;
	}
	assertions.assertExpectedMime(result, ["application/vnd.openxmlformats-officedocument.wordprocessingml.document"]);
	assertions.assertMinContentLength(result, 20);
});

Deno.test("office_docx_formatting", { permissions: { read: true } }, async () => {
	const documentBytes = await resolveDocument("docx/unit_test_formatting.docx");
	const config = buildConfig(undefined);
	let result: ExtractionResult | null = null;
	try {
		// Sync file extraction - WASM uses extractBytes with pre-read bytes
		result = await extractBytes(
			documentBytes,
			"application/vnd.openxmlformats-officedocument.wordprocessingml.document",
			config,
		);
	} catch (error) {
		if (shouldSkipFixture(error, "office_docx_formatting", [], undefined)) {
			return;
		}
		throw error;
	}
	if (result === null) {
		return;
	}
	assertions.assertExpectedMime(result, ["application/vnd.openxmlformats-officedocument.wordprocessingml.document"]);
	assertions.assertMinContentLength(result, 20);
});

Deno.test("office_docx_headers", { permissions: { read: true } }, async () => {
	const documentBytes = await resolveDocument("docx/unit_test_headers.docx");
	const config = buildConfig(undefined);
	let result: ExtractionResult | null = null;
	try {
		// Sync file extraction - WASM uses extractBytes with pre-read bytes
		result = await extractBytes(
			documentBytes,
			"application/vnd.openxmlformats-officedocument.wordprocessingml.document",
			config,
		);
	} catch (error) {
		if (shouldSkipFixture(error, "office_docx_headers", [], undefined)) {
			return;
		}
		throw error;
	}
	if (result === null) {
		return;
	}
	assertions.assertExpectedMime(result, ["application/vnd.openxmlformats-officedocument.wordprocessingml.document"]);
	assertions.assertMinContentLength(result, 20);
});

Deno.test("office_docx_lists", { permissions: { read: true } }, async () => {
	const documentBytes = await resolveDocument("docx/unit_test_lists.docx");
	const config = buildConfig(undefined);
	let result: ExtractionResult | null = null;
	try {
		// Sync file extraction - WASM uses extractBytes with pre-read bytes
		result = await extractBytes(
			documentBytes,
			"application/vnd.openxmlformats-officedocument.wordprocessingml.document",
			config,
		);
	} catch (error) {
		if (shouldSkipFixture(error, "office_docx_lists", [], undefined)) {
			return;
		}
		throw error;
	}
	if (result === null) {
		return;
	}
	assertions.assertExpectedMime(result, ["application/vnd.openxmlformats-officedocument.wordprocessingml.document"]);
	assertions.assertMinContentLength(result, 20);
});

Deno.test("office_docx_tables", { permissions: { read: true } }, async () => {
	const documentBytes = await resolveDocument("docx/docx_tables.docx");
	const config = buildConfig(undefined);
	let result: ExtractionResult | null = null;
	try {
		// Sync file extraction - WASM uses extractBytes with pre-read bytes
		result = await extractBytes(
			documentBytes,
			"application/vnd.openxmlformats-officedocument.wordprocessingml.document",
			config,
		);
	} catch (error) {
		if (shouldSkipFixture(error, "office_docx_tables", [], undefined)) {
			return;
		}
		throw error;
	}
	if (result === null) {
		return;
	}
	assertions.assertExpectedMime(result, ["application/vnd.openxmlformats-officedocument.wordprocessingml.document"]);
	assertions.assertMinContentLength(result, 50);
	assertions.assertContentContainsAll(result, ["Simple uniform table", "Nested Table", "merged cells", "Header Col"]);
	assertions.assertTableCount(result, 1, null);
});

Deno.test("office_fb2_basic", { permissions: { read: true } }, async () => {
	const documentBytes = await resolveDocument("fictionbook/basic.fb2");
	const config = buildConfig(undefined);
	let result: ExtractionResult | null = null;
	try {
		// Sync file extraction - WASM uses extractBytes with pre-read bytes
		result = await extractBytes(documentBytes, "application/x-fictionbook+xml", config);
	} catch (error) {
		if (shouldSkipFixture(error, "office_fb2_basic", [], undefined)) {
			return;
		}
		throw error;
	}
	if (result === null) {
		return;
	}
	assertions.assertExpectedMime(result, ["application/x-fictionbook+xml"]);
	assertions.assertMinContentLength(result, 10);
});

Deno.test("office_markdown_basic", { permissions: { read: true } }, async () => {
	const documentBytes = await resolveDocument("markdown/comprehensive.md");
	const config = buildConfig(undefined);
	let result: ExtractionResult | null = null;
	try {
		// Sync file extraction - WASM uses extractBytes with pre-read bytes
		result = await extractBytes(documentBytes, "text/markdown", config);
	} catch (error) {
		if (shouldSkipFixture(error, "office_markdown_basic", [], undefined)) {
			return;
		}
		throw error;
	}
	if (result === null) {
		return;
	}
	assertions.assertExpectedMime(result, ["text/markdown"]);
	assertions.assertMinContentLength(result, 10);
});

Deno.test("office_org_basic", { permissions: { read: true } }, async () => {
	const documentBytes = await resolveDocument("org/comprehensive.org");
	const config = buildConfig(undefined);
	let result: ExtractionResult | null = null;
	try {
		// Sync file extraction - WASM uses extractBytes with pre-read bytes
		result = await extractBytes(documentBytes, "text/x-org", config);
	} catch (error) {
		if (shouldSkipFixture(error, "office_org_basic", [], undefined)) {
			return;
		}
		throw error;
	}
	if (result === null) {
		return;
	}
	assertions.assertExpectedMime(result, ["text/x-org"]);
	assertions.assertMinContentLength(result, 10);
});

Deno.test("office_ppsx_slideshow", { permissions: { read: true } }, async () => {
	const documentBytes = await resolveDocument("pptx/sample.ppsx");
	const config = buildConfig(undefined);
	let result: ExtractionResult | null = null;
	try {
		// Sync file extraction - WASM uses extractBytes with pre-read bytes
		result = await extractBytes(
			documentBytes,
			"application/vnd.openxmlformats-officedocument.presentationml.slideshow",
			config,
		);
	} catch (error) {
		if (shouldSkipFixture(error, "office_ppsx_slideshow", [], undefined)) {
			return;
		}
		throw error;
	}
	if (result === null) {
		return;
	}
	assertions.assertExpectedMime(result, ["application/vnd.openxmlformats-officedocument.presentationml.slideshow"]);
	assertions.assertMinContentLength(result, 10);
});

Deno.test("office_ppt_legacy", { permissions: { read: true } }, async () => {
	const documentBytes = await resolveDocument("ppt/simple.ppt");
	const config = buildConfig(undefined);
	let result: ExtractionResult | null = null;
	try {
		// Sync file extraction - WASM uses extractBytes with pre-read bytes
		result = await extractBytes(documentBytes, "application/vnd.ms-powerpoint", config);
	} catch (error) {
		if (
			shouldSkipFixture(error, "office_ppt_legacy", ["libreoffice"], "Skip if LibreOffice conversion is unavailable.")
		) {
			return;
		}
		throw error;
	}
	if (result === null) {
		return;
	}
	assertions.assertExpectedMime(result, ["application/vnd.ms-powerpoint"]);
	assertions.assertMinContentLength(result, 10);
});

Deno.test("office_pptx_basic", { permissions: { read: true } }, async () => {
	const documentBytes = await resolveDocument("pptx/simple.pptx");
	const config = buildConfig(undefined);
	let result: ExtractionResult | null = null;
	try {
		// Sync file extraction - WASM uses extractBytes with pre-read bytes
		result = await extractBytes(
			documentBytes,
			"application/vnd.openxmlformats-officedocument.presentationml.presentation",
			config,
		);
	} catch (error) {
		if (shouldSkipFixture(error, "office_pptx_basic", [], undefined)) {
			return;
		}
		throw error;
	}
	if (result === null) {
		return;
	}
	assertions.assertExpectedMime(result, ["application/vnd.openxmlformats-officedocument.presentationml.presentation"]);
	assertions.assertMinContentLength(result, 50);
});

Deno.test("office_pptx_images", { permissions: { read: true } }, async () => {
	const documentBytes = await resolveDocument("pptx/powerpoint_with_image.pptx");
	const config = buildConfig(undefined);
	let result: ExtractionResult | null = null;
	try {
		// Sync file extraction - WASM uses extractBytes with pre-read bytes
		result = await extractBytes(
			documentBytes,
			"application/vnd.openxmlformats-officedocument.presentationml.presentation",
			config,
		);
	} catch (error) {
		if (shouldSkipFixture(error, "office_pptx_images", [], undefined)) {
			return;
		}
		throw error;
	}
	if (result === null) {
		return;
	}
	assertions.assertExpectedMime(result, ["application/vnd.openxmlformats-officedocument.presentationml.presentation"]);
	assertions.assertMinContentLength(result, 20);
});

Deno.test("office_pptx_pitch_deck", { permissions: { read: true } }, async () => {
	const documentBytes = await resolveDocument("pptx/pitch_deck_presentation.pptx");
	const config = buildConfig(undefined);
	let result: ExtractionResult | null = null;
	try {
		// Sync file extraction - WASM uses extractBytes with pre-read bytes
		result = await extractBytes(
			documentBytes,
			"application/vnd.openxmlformats-officedocument.presentationml.presentation",
			config,
		);
	} catch (error) {
		if (shouldSkipFixture(error, "office_pptx_pitch_deck", [], undefined)) {
			return;
		}
		throw error;
	}
	if (result === null) {
		return;
	}
	assertions.assertExpectedMime(result, ["application/vnd.openxmlformats-officedocument.presentationml.presentation"]);
	assertions.assertMinContentLength(result, 100);
});

Deno.test("office_rst_basic", { permissions: { read: true } }, async () => {
	const documentBytes = await resolveDocument("rst/restructured_text.rst");
	const config = buildConfig(undefined);
	let result: ExtractionResult | null = null;
	try {
		// Sync file extraction - WASM uses extractBytes with pre-read bytes
		result = await extractBytes(documentBytes, "text/x-rst", config);
	} catch (error) {
		if (shouldSkipFixture(error, "office_rst_basic", [], undefined)) {
			return;
		}
		throw error;
	}
	if (result === null) {
		return;
	}
	assertions.assertExpectedMime(result, ["text/x-rst"]);
	assertions.assertMinContentLength(result, 10);
});

Deno.test("office_rtf_basic", { permissions: { read: true } }, async () => {
	const documentBytes = await resolveDocument("rtf/lorem_ipsum.rtf");
	const config = buildConfig(undefined);
	let result: ExtractionResult | null = null;
	try {
		// Sync file extraction - WASM uses extractBytes with pre-read bytes
		result = await extractBytes(documentBytes, "application/rtf", config);
	} catch (error) {
		if (shouldSkipFixture(error, "office_rtf_basic", [], undefined)) {
			return;
		}
		throw error;
	}
	if (result === null) {
		return;
	}
	assertions.assertExpectedMime(result, ["application/rtf"]);
	assertions.assertMinContentLength(result, 10);
	assertions.assertContentContainsAny(result, ["lorem", "ipsum"]);
});

Deno.test("office_typst_basic", { permissions: { read: true } }, async () => {
	const documentBytes = await resolveDocument("typst/simple.typ");
	const config = buildConfig(undefined);
	let result: ExtractionResult | null = null;
	try {
		// Sync file extraction - WASM uses extractBytes with pre-read bytes
		result = await extractBytes(documentBytes, "application/x-typst", config);
	} catch (error) {
		if (shouldSkipFixture(error, "office_typst_basic", [], undefined)) {
			return;
		}
		throw error;
	}
	if (result === null) {
		return;
	}
	assertions.assertExpectedMime(result, ["application/x-typst"]);
	assertions.assertMinContentLength(result, 10);
});

Deno.test("office_xls_legacy", { permissions: { read: true } }, async () => {
	const documentBytes = await resolveDocument("xls/test_excel.xls");
	const config = buildConfig(undefined);
	let result: ExtractionResult | null = null;
	try {
		// Sync file extraction - WASM uses extractBytes with pre-read bytes
		result = await extractBytes(documentBytes, "application/vnd.ms-excel", config);
	} catch (error) {
		if (shouldSkipFixture(error, "office_xls_legacy", [], undefined)) {
			return;
		}
		throw error;
	}
	if (result === null) {
		return;
	}
	assertions.assertExpectedMime(result, ["application/vnd.ms-excel"]);
	assertions.assertMinContentLength(result, 10);
});

Deno.test("office_xlsx_basic", { permissions: { read: true } }, async () => {
	const documentBytes = await resolveDocument("xlsx/stanley_cups.xlsx");
	const config = buildConfig(undefined);
	let result: ExtractionResult | null = null;
	try {
		// Sync file extraction - WASM uses extractBytes with pre-read bytes
		result = await extractBytes(
			documentBytes,
			"application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
			config,
		);
	} catch (error) {
		if (shouldSkipFixture(error, "office_xlsx_basic", [], undefined)) {
			return;
		}
		throw error;
	}
	if (result === null) {
		return;
	}
	assertions.assertExpectedMime(result, ["application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"]);
	assertions.assertMinContentLength(result, 100);
	assertions.assertContentContainsAll(result, ["Team", "Location", "Stanley Cups"]);
	assertions.assertTableCount(result, 1, null);
	assertions.assertMetadataExpectation(result, "sheet_count", { gte: 2 });
	assertions.assertMetadataExpectation(result, "sheet_names", { contains: ["Stanley Cups"] });
});

Deno.test("office_xlsx_multi_sheet", { permissions: { read: true } }, async () => {
	const documentBytes = await resolveDocument("xlsx/excel_multi_sheet.xlsx");
	const config = buildConfig(undefined);
	let result: ExtractionResult | null = null;
	try {
		// Sync file extraction - WASM uses extractBytes with pre-read bytes
		result = await extractBytes(
			documentBytes,
			"application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
			config,
		);
	} catch (error) {
		if (shouldSkipFixture(error, "office_xlsx_multi_sheet", [], undefined)) {
			return;
		}
		throw error;
	}
	if (result === null) {
		return;
	}
	assertions.assertExpectedMime(result, ["application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"]);
	assertions.assertMinContentLength(result, 20);
	assertions.assertMetadataExpectation(result, "sheet_count", { gte: 2 });
});

Deno.test("office_xlsx_office_example", { permissions: { read: true } }, async () => {
	const documentBytes = await resolveDocument("xlsx/test_01.xlsx");
	const config = buildConfig(undefined);
	let result: ExtractionResult | null = null;
	try {
		// Sync file extraction - WASM uses extractBytes with pre-read bytes
		result = await extractBytes(
			documentBytes,
			"application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
			config,
		);
	} catch (error) {
		if (shouldSkipFixture(error, "office_xlsx_office_example", [], undefined)) {
			return;
		}
		throw error;
	}
	if (result === null) {
		return;
	}
	assertions.assertExpectedMime(result, ["application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"]);
	assertions.assertMinContentLength(result, 10);
});
