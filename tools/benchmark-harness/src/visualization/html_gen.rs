//! HTML generation and CSS styling for benchmark visualization
//!
//! This module handles CSS generation and Minijinja template rendering to produce
//! the final HTML report. CSS is generated programmatically based on the design
//! system colors from docs/css/extra.css.

use super::data_prep::VisualizationData;
use super::templates::BASE_TEMPLATE;
use minijinja::{Environment, context};

/// CSS stylesheet for the HTML visualization
///
/// Uses colors from the design system:
/// - Primary: #da2ae0 (magenta)
/// - Accent opposite: #58fbda (cyan)
/// - Percentile indicators: p50=green (#4caf50), p95=orange (#ff9800), p99=red (#f44336)
/// - Dark mode percentiles: Adjusted for WCAG AAA contrast (7:1 minimum)
const BASE_CSS: &str = r#"
    :root {
        --primary-light: #da2ae0;
        --primary-dark: #58fbda;
        --bg-light: #ffffff;
        --bg-dark: #23232c;
        --text-light: #323040;
        --text-dark: rgba(255, 255, 255, 0.8);
        --radius: 8px;
        --spacing: 8px;
    }

    [data-theme="light"] {
        --primary: var(--primary-light);
        --background: var(--bg-light);
        --text: var(--text-light);
    }

    [data-theme="dark"] {
        --primary: var(--primary-dark);
        --background: var(--bg-dark);
        --text: var(--text-dark);
    }

    * {
        box-sizing: border-box;
    }

    body {
        font-family: Inter, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
        background: var(--background);
        color: var(--text);
        margin: 0;
        padding: 2rem;
        line-height: 1.6;
    }

    .container {
        max-width: 1400px;
        margin: 0 auto;
    }

    header {
        margin-bottom: 3rem;
        padding-bottom: 2rem;
        border-bottom: 2px solid var(--primary);
    }

    header h1 {
        margin: 0 0 0.5rem 0;
        color: var(--primary);
        font-size: 2.5rem;
        font-weight: 700;
    }

    header .timestamp {
        margin: 0;
        color: var(--text);
        opacity: 0.7;
        font-size: 0.95rem;
    }

    header .summary {
        margin: 0.5rem 0 0 0;
        color: var(--text);
        opacity: 0.6;
        font-size: 0.9rem;
    }

    section {
        margin: 3rem 0;
    }

    section h2 {
        color: var(--text);
        font-size: 1.8rem;
        font-weight: 600;
        margin: 0 0 1.5rem 0;
        border-left: 4px solid var(--primary);
        padding-left: 1rem;
    }

    .benchmark-table {
        width: 100%;
        border-collapse: collapse;
        margin: 0;
        border-radius: var(--radius);
        overflow: hidden;
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
        background: var(--background);
    }

    .benchmark-table thead {
        background: var(--primary);
        color: white;
    }

    .benchmark-table th {
        padding: calc(var(--spacing) * 2);
        text-align: left;
        font-weight: 600;
        font-size: 0.95rem;
        white-space: nowrap;
    }

    .benchmark-table td {
        padding: calc(var(--spacing) * 1.5) calc(var(--spacing) * 2);
        border-bottom: 1px solid rgba(0, 0, 0, 0.1);
    }

    [data-theme="dark"] .benchmark-table td {
        border-bottom: 1px solid rgba(255, 255, 255, 0.1);
    }

    .benchmark-table tbody tr {
        transition: background-color 0.2s ease;
    }

    .benchmark-table tbody tr:hover {
        background-color: rgba(218, 42, 224, 0.05);
    }

    [data-theme="dark"] .benchmark-table tbody tr:hover {
        background-color: rgba(88, 251, 218, 0.05);
    }

    .benchmark-table tbody tr:last-child td {
        border-bottom: none;
    }

    .na-cell {
        text-align: center;
        color: rgba(0, 0, 0, 0.4);
        font-style: italic;
    }

    [data-theme="dark"] .na-cell {
        color: rgba(255, 255, 255, 0.4);
    }

    .unsupported {
        opacity: 0.5;
        background: rgba(0, 0, 0, 0.02);
    }

    [data-theme="dark"] .unsupported {
        background: rgba(255, 255, 255, 0.02);
    }

    .mode-badge {
        display: inline-block;
        padding: 4px 10px;
        border-radius: 4px;
        font-size: 0.8rem;
        font-weight: 600;
        text-align: center;
        white-space: nowrap;
    }

    .mode-single {
        background: #e3f2fd;
        color: #1976d2;
    }

    .mode-batch {
        background: #f3e5f5;
        color: #7b1fa2;
    }

    .mode-sync {
        background: #e8f5e9;
        color: #388e3c;
    }

    .mode-async {
        background: #fff3e0;
        color: #f57c00;
    }

    [data-theme="dark"] .mode-single {
        background: rgba(25, 118, 210, 0.2);
        color: #64b5f6;
    }

    [data-theme="dark"] .mode-batch {
        background: rgba(123, 31, 162, 0.2);
        color: #ce93d8;
    }

    [data-theme="dark"] .mode-sync {
        background: rgba(56, 142, 60, 0.2);
        color: #81c784;
    }

    [data-theme="dark"] .mode-async {
        background: rgba(245, 124, 0, 0.2);
        color: #ffb74d;
    }

    .ocr-badge {
        display: inline-block;
        padding: 4px 10px;
        border-radius: 4px;
        font-size: 0.8rem;
        font-weight: 600;
        text-align: center;
        white-space: nowrap;
    }

    .ocr-no {
        background: #e0f7fa;
        color: #00838f;
    }

    .ocr-yes {
        background: #fff9c4;
        color: #f57f17;
    }

    [data-theme="dark"] .ocr-no {
        background: rgba(0, 131, 143, 0.2);
        color: #4dd0e1;
    }

    [data-theme="dark"] .ocr-yes {
        background: rgba(245, 127, 23, 0.2);
        color: #ffd54f;
    }

    .metric-cell {
        font-family: 'Courier New', monospace;
        font-size: 0.9rem;
    }

    .metric-cell .p50 {
        color: #4caf50;
        font-weight: 600;
    }

    .metric-cell .p95 {
        color: #ff9800;
        font-weight: 600;
    }

    .metric-cell .p99 {
        color: #f44336;
        font-weight: 600;
    }

    [data-theme="dark"] .metric-cell .p50 {
        color: #81d65b;
    }

    [data-theme="dark"] .metric-cell .p95 {
        color: #ffc76b;
    }

    [data-theme="dark"] .metric-cell .p99 {
        color: #ff6b6b;
    }

    .success-rate {
        font-weight: 500;
        text-align: right;
    }

    .success-rate.warning {
        color: #f57c00;
        font-weight: 600;
    }

    [data-theme="dark"] .success-rate.warning {
        color: #ffb74d;
    }

    .error-indicator {
        display: inline-block;
        width: 20px;
        height: 20px;
        background: #f44336;
        color: white;
        border-radius: 50%;
        text-align: center;
        line-height: 20px;
        font-size: 0.75rem;
        font-weight: bold;
        cursor: help;
        margin-left: 4px;
    }

    .error-indicator:hover {
        background: #d32f2f;
        box-shadow: 0 0 0 3px rgba(244, 67, 54, 0.2);
    }

    @media (max-width: 1024px) {
        .benchmark-table {
            font-size: 0.9rem;
        }

        .benchmark-table th,
        .benchmark-table td {
            padding: 8px 12px;
        }
    }

    @media (max-width: 768px) {
        body {
            padding: 1rem;
        }

        header h1 {
            font-size: 1.8rem;
        }

        .benchmark-table {
            font-size: 0.8rem;
        }

        .benchmark-table th,
        .benchmark-table td {
            padding: 6px 8px;
        }

        .mode-badge,
        .ocr-badge {
            padding: 3px 6px;
            font-size: 0.7rem;
        }
    }
    "#;

/// Render HTML from visualization data using Minijinja templates
///
/// This function:
/// 1. Creates a Minijinja environment with all macros
/// 2. Renders the base template with the visualization data
/// 3. Returns the complete HTML as a string
///
/// # Arguments
/// * `data` - The prepared visualization data
///
/// # Returns
/// * `Result<String>` - The rendered HTML or error message
pub fn render_html(data: &VisualizationData) -> Result<String, Box<dyn std::error::Error>> {
    // Create Minijinja environment with autoescape enabled for XSS protection
    let mut env = Environment::new();
    env.set_auto_escape_callback(|_| minijinja::AutoEscape::Html);

    // Register the main template (which has inlined all table content)
    env.add_template("base", BASE_TEMPLATE)?;

    // Get the base template
    let tmpl = env.get_template("base")?;

    // Prepare context with data and CSS
    let ctx = context! {
        metadata => &data.metadata,
        file_type_comparisons => &data.file_type_comparisons,
        framework_metrics => &data.framework_metrics,
        disk_sizes => &data.disk_sizes,
        css => BASE_CSS,
    };

    // Render template
    let html = tmpl.render(ctx)?;

    Ok(html)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visualization::data_prep::{
        FileTypeComparison, FrameworkMetrics, MetricValues, PercentileValues, VisualizationMetadata,
    };
    use std::collections::HashMap;

    fn create_test_visualization_data() -> VisualizationData {
        let metadata = VisualizationMetadata {
            total_results: 10,
            framework_count: 2,
            file_type_count: 1,
            aggregation_timestamp: "2024-01-01T00:00:00Z".to_string(),
        };

        let percentiles = PercentileValues {
            p50: 10.5,
            p95: 15.3,
            p99: 18.7,
        };

        let metrics = MetricValues {
            sample_count: 5,
            success_rate_percent: 100.0,
            memory: PercentileValues {
                p50: 100.0,
                p95: 150.0,
                p99: 180.0,
            },
            duration: PercentileValues {
                p50: 50.5,
                p95: 75.3,
                p99: 90.7,
            },
            throughput: percentiles,
        };

        let framework_metrics = FrameworkMetrics {
            framework: "test-framework".to_string(),
            mode: "sync".to_string(),
            key: "test-framework:sync".to_string(),
            file_type: Some("pdf".to_string()),
            no_ocr: Some(metrics),
            with_ocr: None,
            cold_start: None,
        };

        let file_type_comparison = FileTypeComparison {
            file_type: "pdf".to_string(),
            frameworks: vec![framework_metrics],
        };

        let mut disk_sizes = HashMap::new();
        disk_sizes.insert("test-framework".to_string(), 50.0);

        VisualizationData {
            file_type_comparisons: vec![file_type_comparison],
            framework_metrics: vec![],
            disk_sizes,
            metadata,
        }
    }

    #[test]
    fn test_base_css() {
        assert!(BASE_CSS.contains("--primary-light: #da2ae0"));
        assert!(BASE_CSS.contains("--primary-dark: #58fbda"));
        assert!(BASE_CSS.contains(".p50"));
        assert!(BASE_CSS.contains(".p95"));
        assert!(BASE_CSS.contains(".p99"));
        assert!(BASE_CSS.contains(".mode-badge"));
        assert!(BASE_CSS.contains(".ocr-badge"));
        assert!(BASE_CSS.len() > 1000);

        // Verify dark mode colors have been improved for accessibility
        assert!(BASE_CSS.contains("#81d65b")); // p50 dark mode
        assert!(BASE_CSS.contains("#ffc76b")); // p95 dark mode
        assert!(BASE_CSS.contains("#ff6b6b")); // p99 dark mode
    }

    #[test]
    fn test_render_html_basic() {
        let data = create_test_visualization_data();
        let result = render_html(&data);

        if let Err(e) = &result {
            eprintln!("Template rendering error: {}", e);
        }
        assert!(result.is_ok());
        let html = result.unwrap();

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Framework Benchmark Results"));
        assert!(html.contains("test-framework"));
        assert!(html.contains("PDF"));
    }

    #[test]
    fn test_render_html_contains_css() {
        let data = create_test_visualization_data();
        let result = render_html(&data);

        assert!(result.is_ok());
        let html = result.unwrap();

        assert!(html.contains("--primary-light: #da2ae0"));
        assert!(html.contains("<style>"));
    }
}
