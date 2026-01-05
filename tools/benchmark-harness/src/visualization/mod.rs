//! Visualization system for benchmark results
//!
//! This module provides data preparation and template rendering for generating
//! HTML visualizations of benchmark results.

use crate::aggregate::NewConsolidatedResults;
use std::path::Path;

pub mod data_prep;
pub mod html_gen;
pub mod templates;

pub use data_prep::{
    ColdStartMetrics, ColdStartRow, DiskSizeRow, ErrorInfo, FileTypeComparison, FrameworkMetrics, MetricValues,
    PercentileValues, VisualizationData, VisualizationMetadata, convert_percentile_metrics, prepare_visualization_data,
};
pub use html_gen::render_html;

/// Generate HTML visualization report from aggregated benchmark results
///
/// This is the main entry point for generating HTML visualizations. It:
/// 1. Transforms aggregated data into visualization-friendly structures
/// 2. Renders HTML using minijinja templates
/// 3. Writes the HTML file to the specified output path
///
/// # Arguments
/// * `aggregated` - The aggregated benchmark results
/// * `output_path` - Path where the HTML file should be written (e.g., "docs/benchmarks/charts/index.html")
///
/// # Returns
/// * `Result<()>` - Success or error message
///
/// # Example
/// ```no_run
/// use benchmark_harness::visualization::generate_html_report;
/// use benchmark_harness::aggregate::NewConsolidatedResults;
/// use std::path::Path;
///
/// # fn example(aggregated: &NewConsolidatedResults) -> Result<(), Box<dyn std::error::Error>> {
/// generate_html_report(aggregated, Path::new("output/index.html"))?;
/// # Ok(())
/// # }
/// ```
pub fn generate_html_report(
    aggregated: &NewConsolidatedResults,
    output_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Prepare visualization data from aggregated results
    let vis_data = data_prep::prepare_visualization_data(aggregated);

    // Step 2: Render HTML from visualization data
    let html = html_gen::render_html(&vis_data)?;

    // Step 3: Ensure parent directory exists
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Step 4: Write HTML to file
    std::fs::write(output_path, html)?;

    Ok(())
}
