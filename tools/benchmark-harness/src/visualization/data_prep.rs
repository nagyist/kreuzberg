//! Data preparation for visualization
//!
//! This module provides functions to transform aggregated benchmark results
//! into visualization-ready formats using Jinja2 templates.

use crate::aggregate::{DurationPercentiles, NewConsolidatedResults, PerformancePercentiles};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Complete visualization data ready for template rendering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationData {
    /// Framework comparisons organized by file type
    pub file_type_comparisons: Vec<FileTypeComparison>,
    /// Framework metrics for each framework:mode combination
    pub framework_metrics: Vec<FrameworkMetrics>,
    /// Disk size information for each framework
    pub disk_sizes: HashMap<String, f64>,
    /// Metadata about the data
    pub metadata: VisualizationMetadata,
}

/// Comparison data for a specific file type across frameworks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTypeComparison {
    /// File type extension (e.g., "pdf", "docx")
    pub file_type: String,
    /// Metrics for this file type grouped by framework:mode
    pub frameworks: Vec<FrameworkMetrics>,
}

/// Metrics for a specific framework:mode combination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameworkMetrics {
    /// Framework name
    pub framework: String,
    /// Processing mode (single, batch, sync, async)
    pub mode: String,
    /// Combined key: "framework:mode"
    pub key: String,
    /// File type being measured (None if aggregate across types)
    pub file_type: Option<String>,
    /// Performance metrics without OCR
    pub no_ocr: Option<MetricValues>,
    /// Performance metrics with OCR
    pub with_ocr: Option<MetricValues>,
    /// Cold start duration metrics (if available)
    pub cold_start: Option<ColdStartMetrics>,
}

/// Performance metrics for a specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricValues {
    /// Number of samples used for calculations
    pub sample_count: usize,
    /// Success rate as a percentage (0-100)
    pub success_rate_percent: f64,
    /// Memory usage metrics (MB)
    pub memory: PercentileValues,
    /// Duration metrics (milliseconds)
    pub duration: PercentileValues,
    /// Throughput metrics (MB/s)
    pub throughput: PercentileValues,
}

/// Percentile values for a metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PercentileValues {
    /// 50th percentile (median)
    pub p50: f64,
    /// 95th percentile
    pub p95: f64,
    /// 99th percentile
    pub p99: f64,
}

/// Cold start duration metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColdStartMetrics {
    /// Number of cold start samples
    pub sample_count: usize,
    /// Duration percentiles (milliseconds)
    pub duration: PercentileValues,
}

/// Error information for failed benchmarks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorInfo {
    /// Framework that failed
    pub framework: String,
    /// File type being tested
    pub file_type: String,
    /// Error message
    pub message: String,
}

/// Disk size information for display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskSizeRow {
    /// Framework name
    pub framework: String,
    /// Installation size in MB
    pub size_mb: f64,
}

/// Cold start duration information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColdStartRow {
    /// Framework:mode key
    pub framework_mode: String,
    /// Cold start duration (ms)
    pub duration_ms: f64,
}

/// Metadata about the visualization data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationMetadata {
    /// Total number of benchmark results
    pub total_results: usize,
    /// Number of unique frameworks
    pub framework_count: usize,
    /// Number of unique file types
    pub file_type_count: usize,
    /// Timestamp when data was aggregated
    pub aggregation_timestamp: String,
}

/// Prepare visualization data from aggregated benchmark results
///
/// This function transforms the aggregated results into a structure optimized
/// for template rendering, extracting both no_ocr and with_ocr metrics separately
/// for each framework and file type combination.
pub fn prepare_visualization_data(aggregated: &NewConsolidatedResults) -> VisualizationData {
    let mut file_type_comparisons: HashMap<String, Vec<FrameworkMetrics>> = HashMap::new();
    let mut all_frameworks: Vec<FrameworkMetrics> = Vec::new();

    // Process each framework:mode combination
    for (key, framework_mode_data) in &aggregated.by_framework_mode {
        // Extract framework and mode from the key
        let (framework, mode) = parse_framework_mode_key(key);

        // Process each file type
        for (file_type, file_type_agg) in &framework_mode_data.by_file_type {
            let metrics = FrameworkMetrics {
                framework: framework.to_string(),
                mode: mode.to_string(),
                key: key.clone(),
                file_type: Some(file_type.clone()),
                no_ocr: file_type_agg.no_ocr.as_ref().map(convert_percentile_metrics),
                with_ocr: file_type_agg.with_ocr.as_ref().map(convert_percentile_metrics),
                cold_start: framework_mode_data.cold_start.as_ref().map(convert_cold_start_metrics),
            };

            file_type_comparisons
                .entry(file_type.clone())
                .or_default()
                .push(metrics.clone());

            all_frameworks.push(metrics);
        }
    }

    // Convert file_type_comparisons to the final structure
    let file_type_comparisons: Vec<FileTypeComparison> = file_type_comparisons
        .into_iter()
        .map(|(file_type, frameworks)| FileTypeComparison { file_type, frameworks })
        .collect();

    // Convert disk sizes from bytes to MB
    let disk_sizes: HashMap<String, f64> = aggregated
        .disk_sizes
        .iter()
        .map(|(framework, disk_info)| {
            let size_mb = disk_info.size_bytes as f64 / 1_000_000.0;
            (framework.clone(), size_mb)
        })
        .collect();

    let metadata = VisualizationMetadata {
        total_results: aggregated.metadata.total_results,
        framework_count: aggregated.metadata.framework_count,
        file_type_count: aggregated.metadata.file_type_count,
        aggregation_timestamp: aggregated.metadata.timestamp.clone(),
    };

    VisualizationData {
        file_type_comparisons,
        framework_metrics: all_frameworks,
        disk_sizes,
        metadata,
    }
}

/// Convert percentile metrics to visualization format
///
/// Converts units:
/// - Memory: bytes to MB (divide by 1,000,000)
/// - Duration: already in ms (from aggregate.rs)
/// - Throughput: already in MB/s (from aggregate.rs)
pub fn convert_percentile_metrics(perf: &PerformancePercentiles) -> MetricValues {
    MetricValues {
        sample_count: perf.sample_count,
        success_rate_percent: perf.success_rate * 100.0,
        memory: PercentileValues {
            p50: perf.memory.p50,
            p95: perf.memory.p95,
            p99: perf.memory.p99,
        },
        duration: PercentileValues {
            p50: perf.duration.p50,
            p95: perf.duration.p95,
            p99: perf.duration.p99,
        },
        throughput: PercentileValues {
            p50: perf.throughput.p50,
            p95: perf.throughput.p95,
            p99: perf.throughput.p99,
        },
    }
}

/// Convert cold start metrics to visualization format
fn convert_cold_start_metrics(cold_start: &DurationPercentiles) -> ColdStartMetrics {
    ColdStartMetrics {
        sample_count: cold_start.sample_count,
        duration: PercentileValues {
            p50: cold_start.p50_ms,
            p95: cold_start.p95_ms,
            p99: cold_start.p99_ms,
        },
    }
}

/// Parse framework:mode key into components
fn parse_framework_mode_key(key: &str) -> (&str, &str) {
    match key.split_once(':') {
        Some((framework, mode)) => (framework, mode),
        None => (key, "single"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aggregate::{
        ConsolidationMetadata, FileTypeAggregation, FrameworkModeAggregation, Percentiles, PerformancePercentiles,
    };
    use crate::types::DiskSizeInfo;
    use std::collections::HashMap;

    fn create_test_performance_percentiles(sample_count: usize, success_rate: f64) -> PerformancePercentiles {
        PerformancePercentiles {
            sample_count,
            throughput: Percentiles {
                p50: 10.0,
                p95: 15.0,
                p99: 18.0,
            },
            memory: Percentiles {
                p50: 100.0,
                p95: 150.0,
                p99: 180.0,
            },
            duration: Percentiles {
                p50: 50.0,
                p95: 75.0,
                p99: 90.0,
            },
            success_rate,
        }
    }

    fn create_test_disk_size_info(size_bytes: u64) -> DiskSizeInfo {
        DiskSizeInfo {
            size_bytes,
            method: "test".to_string(),
            description: "Test disk size".to_string(),
        }
    }

    #[test]
    fn test_convert_percentile_metrics() {
        let perf = create_test_performance_percentiles(10, 0.95);
        let metrics = convert_percentile_metrics(&perf);

        assert_eq!(metrics.sample_count, 10);
        assert_eq!(metrics.success_rate_percent, 95.0);
        assert_eq!(metrics.memory.p50, 100.0);
        assert_eq!(metrics.memory.p95, 150.0);
        assert_eq!(metrics.memory.p99, 180.0);
        assert_eq!(metrics.duration.p50, 50.0);
        assert_eq!(metrics.throughput.p50, 10.0);
    }

    #[test]
    fn test_prepare_visualization_data_basic() {
        let mut by_framework_mode = HashMap::new();
        let mut by_file_type = HashMap::new();

        let file_type_agg = FileTypeAggregation {
            file_type: "pdf".to_string(),
            no_ocr: Some(create_test_performance_percentiles(5, 1.0)),
            with_ocr: Some(create_test_performance_percentiles(3, 0.9)),
        };

        by_file_type.insert("pdf".to_string(), file_type_agg);

        let framework_mode_agg = FrameworkModeAggregation {
            framework: "kreuzberg".to_string(),
            mode: "sync".to_string(),
            cold_start: None,
            by_file_type,
        };

        by_framework_mode.insert("kreuzberg:sync".to_string(), framework_mode_agg);

        let mut disk_sizes = HashMap::new();
        disk_sizes.insert("kreuzberg".to_string(), create_test_disk_size_info(50_000_000));

        let aggregated = NewConsolidatedResults {
            by_framework_mode,
            disk_sizes,
            metadata: ConsolidationMetadata {
                total_results: 8,
                framework_count: 1,
                file_type_count: 1,
                timestamp: "2024-01-01T00:00:00Z".to_string(),
            },
        };

        let viz_data = prepare_visualization_data(&aggregated);

        assert_eq!(viz_data.file_type_comparisons.len(), 1);
        assert_eq!(viz_data.framework_metrics.len(), 1);
        assert_eq!(viz_data.disk_sizes.len(), 1);

        let file_type_comp = &viz_data.file_type_comparisons[0];
        assert_eq!(file_type_comp.file_type, "pdf");
        assert_eq!(file_type_comp.frameworks.len(), 1);

        let framework_metrics = &file_type_comp.frameworks[0];
        assert_eq!(framework_metrics.framework, "kreuzberg");
        assert_eq!(framework_metrics.mode, "sync");
        assert_eq!(framework_metrics.key, "kreuzberg:sync");
        assert_eq!(framework_metrics.file_type, Some("pdf".to_string()));

        assert!(framework_metrics.no_ocr.is_some());
        assert!(framework_metrics.with_ocr.is_some());

        let no_ocr = framework_metrics.no_ocr.as_ref().unwrap();
        assert_eq!(no_ocr.sample_count, 5);
        assert_eq!(no_ocr.success_rate_percent, 100.0);

        let with_ocr = framework_metrics.with_ocr.as_ref().unwrap();
        assert_eq!(with_ocr.sample_count, 3);
        assert_eq!(with_ocr.success_rate_percent, 90.0);

        // Check disk size conversion (50M bytes to MB)
        let kreuzberg_size = viz_data.disk_sizes.get("kreuzberg").unwrap();
        assert_eq!(*kreuzberg_size, 50.0);

        // Check metadata
        assert_eq!(viz_data.metadata.total_results, 8);
        assert_eq!(viz_data.metadata.framework_count, 1);
        assert_eq!(viz_data.metadata.file_type_count, 1);
    }

    #[test]
    fn test_prepare_visualization_data_multiple_frameworks() {
        let mut by_framework_mode = HashMap::new();

        // First framework
        let mut by_file_type_1 = HashMap::new();
        by_file_type_1.insert(
            "pdf".to_string(),
            FileTypeAggregation {
                file_type: "pdf".to_string(),
                no_ocr: Some(create_test_performance_percentiles(5, 1.0)),
                with_ocr: None,
            },
        );

        let framework_mode_agg_1 = FrameworkModeAggregation {
            framework: "kreuzberg".to_string(),
            mode: "sync".to_string(),
            cold_start: None,
            by_file_type: by_file_type_1,
        };

        // Second framework
        let mut by_file_type_2 = HashMap::new();
        by_file_type_2.insert(
            "pdf".to_string(),
            FileTypeAggregation {
                file_type: "pdf".to_string(),
                no_ocr: Some(create_test_performance_percentiles(4, 0.95)),
                with_ocr: None,
            },
        );

        let framework_mode_agg_2 = FrameworkModeAggregation {
            framework: "pdfplumber".to_string(),
            mode: "single".to_string(),
            cold_start: None,
            by_file_type: by_file_type_2,
        };

        by_framework_mode.insert("kreuzberg:sync".to_string(), framework_mode_agg_1);
        by_framework_mode.insert("pdfplumber:single".to_string(), framework_mode_agg_2);

        let disk_sizes = HashMap::new();

        let aggregated = NewConsolidatedResults {
            by_framework_mode,
            disk_sizes,
            metadata: ConsolidationMetadata {
                total_results: 9,
                framework_count: 2,
                file_type_count: 1,
                timestamp: "2024-01-01T00:00:00Z".to_string(),
            },
        };

        let viz_data = prepare_visualization_data(&aggregated);

        assert_eq!(viz_data.file_type_comparisons.len(), 1);
        assert_eq!(viz_data.framework_metrics.len(), 2);

        let file_type_comp = &viz_data.file_type_comparisons[0];
        assert_eq!(file_type_comp.frameworks.len(), 2);

        // Verify frameworks are in the comparison
        assert!(file_type_comp.frameworks.iter().any(|f| f.framework == "kreuzberg"));
        assert!(file_type_comp.frameworks.iter().any(|f| f.framework == "pdfplumber"));
    }

    #[test]
    fn test_parse_framework_mode_key() {
        assert_eq!(parse_framework_mode_key("kreuzberg:sync"), ("kreuzberg", "sync"));
        assert_eq!(parse_framework_mode_key("pdfplumber:batch"), ("pdfplumber", "batch"));
        assert_eq!(parse_framework_mode_key("framework:async"), ("framework", "async"));
        assert_eq!(parse_framework_mode_key("framework"), ("framework", "single"));
    }

    #[test]
    fn test_empty_aggregation() {
        let aggregated = NewConsolidatedResults {
            by_framework_mode: HashMap::new(),
            disk_sizes: HashMap::new(),
            metadata: ConsolidationMetadata {
                total_results: 0,
                framework_count: 0,
                file_type_count: 0,
                timestamp: "2024-01-01T00:00:00Z".to_string(),
            },
        };

        let viz_data = prepare_visualization_data(&aggregated);

        assert_eq!(viz_data.file_type_comparisons.len(), 0);
        assert_eq!(viz_data.framework_metrics.len(), 0);
        assert_eq!(viz_data.disk_sizes.len(), 0);
        assert_eq!(viz_data.metadata.total_results, 0);
    }
}
