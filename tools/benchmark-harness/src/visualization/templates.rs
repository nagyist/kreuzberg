//! Minijinja templates for HTML visualization
//!
//! This module contains all template constants used for rendering the benchmark
//! visualization HTML report. Templates use minijinja syntax and are embedded as
//! string constants for self-contained deployment.

/// Main HTML template with header, file-type sections, disk sizes, and cold starts
/// This template includes all tables inline for compatibility with minijinja
pub const BASE_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Benchmark Results - {{ metadata.aggregation_timestamp }}</title>
    <style>{{ css }}</style>
</head>
<body data-theme="light">
    <div class="container">
        <header>
            <h1>Framework Benchmark Results</h1>
            <p class="timestamp">Generated: {{ metadata.aggregation_timestamp }}</p>
            <p class="summary">
                {{ metadata.framework_count }} frameworks |
                {{ metadata.file_type_count }} file types |
                {{ metadata.total_results }} results
            </p>
        </header>

        {% for file_type_data in file_type_comparisons %}
        <section class="file-type-section">
            <h2>{{ file_type_data.file_type | upper }} Files</h2>
            <table class="benchmark-table">
                <thead>
                    <tr>
                        <th>Framework</th>
                        <th>Mode</th>
                        <th>OCR</th>
                        <th>Throughput (p50/p95/p99) MB/s</th>
                        <th>Memory (p50/p95/p99) MB</th>
                        <th>Duration (p50/p95/p99) ms</th>
                        <th>Success Rate</th>
                    </tr>
                </thead>
                <tbody>
                    {% for framework in file_type_data.frameworks %}
                    {% if framework.no_ocr %}
                    <tr>
                        <td>{{ framework.framework }}</td>
                        <td><span class="mode-badge mode-{{ framework.mode }}">{{ framework.mode }}</span></td>
                        <td><span class="ocr-badge ocr-no">No OCR</span></td>
                        <td class="metric-cell">
                            <span class="p50">{{ framework.no_ocr.throughput.p50 | round(2) }}</span> /
                            <span class="p95">{{ framework.no_ocr.throughput.p95 | round(2) }}</span> /
                            <span class="p99">{{ framework.no_ocr.throughput.p99 | round(2) }}</span>
                        </td>
                        <td class="metric-cell">
                            <span class="p50">{{ framework.no_ocr.memory.p50 | round(1) }}</span> /
                            <span class="p95">{{ framework.no_ocr.memory.p95 | round(1) }}</span> /
                            <span class="p99">{{ framework.no_ocr.memory.p99 | round(1) }}</span>
                        </td>
                        <td class="metric-cell">
                            <span class="p50">{{ framework.no_ocr.duration.p50 | round(1) }}</span> /
                            <span class="p95">{{ framework.no_ocr.duration.p95 | round(1) }}</span> /
                            <span class="p99">{{ framework.no_ocr.duration.p99 | round(1) }}</span>
                        </td>
                        <td class="success-rate {% if framework.no_ocr.success_rate_percent < 100.0 %}warning{% endif %}">
                            {{ framework.no_ocr.success_rate_percent | round(1) }}%
                        </td>
                    </tr>
                    {% endif %}
                    {% if framework.with_ocr %}
                    <tr>
                        <td>{{ framework.framework }}</td>
                        <td><span class="mode-badge mode-{{ framework.mode }}">{{ framework.mode }}</span></td>
                        <td><span class="ocr-badge ocr-yes">With OCR</span></td>
                        <td class="metric-cell">
                            <span class="p50">{{ framework.with_ocr.throughput.p50 | round(2) }}</span> /
                            <span class="p95">{{ framework.with_ocr.throughput.p95 | round(2) }}</span> /
                            <span class="p99">{{ framework.with_ocr.throughput.p99 | round(2) }}</span>
                        </td>
                        <td class="metric-cell">
                            <span class="p50">{{ framework.with_ocr.memory.p50 | round(1) }}</span> /
                            <span class="p95">{{ framework.with_ocr.memory.p95 | round(1) }}</span> /
                            <span class="p99">{{ framework.with_ocr.memory.p99 | round(1) }}</span>
                        </td>
                        <td class="metric-cell">
                            <span class="p50">{{ framework.with_ocr.duration.p50 | round(1) }}</span> /
                            <span class="p95">{{ framework.with_ocr.duration.p95 | round(1) }}</span> /
                            <span class="p99">{{ framework.with_ocr.duration.p99 | round(1) }}</span>
                        </td>
                        <td class="success-rate {% if framework.with_ocr.success_rate_percent < 100.0 %}warning{% endif %}">
                            {{ framework.with_ocr.success_rate_percent | round(1) }}%
                        </td>
                    </tr>
                    {% endif %}
                    {% endfor %}
                </tbody>
            </table>
        </section>
        {% endfor %}

        <section class="disk-sizes">
            <h2>Installation Sizes</h2>
            <table class="benchmark-table">
                <thead>
                    <tr>
                        <th>Framework</th>
                        <th>Installation Size (MB)</th>
                    </tr>
                </thead>
                <tbody>
                    {% for framework, size_mb in disk_sizes | dictsort %}
                    <tr>
                        <td>{{ framework }}</td>
                        <td class="metric-cell">{{ size_mb | round(1) }}</td>
                    </tr>
                    {% endfor %}
                </tbody>
            </table>
        </section>

        <section class="cold-starts">
            <h2>Cold Start Times</h2>
            <table class="benchmark-table">
                <thead>
                    <tr>
                        <th>Framework</th>
                        <th>Mode</th>
                        <th>Cold Start Duration (p50/p95/p99) ms</th>
                    </tr>
                </thead>
                <tbody>
                    {% for framework in framework_metrics %}
                    {% if framework.cold_start %}
                    <tr>
                        <td>{{ framework.framework }}</td>
                        <td><span class="mode-badge mode-{{ framework.mode }}">{{ framework.mode }}</span></td>
                        <td class="metric-cell">
                            <span class="p50">{{ framework.cold_start.duration.p50 | round(1) }}</span> /
                            <span class="p95">{{ framework.cold_start.duration.p95 | round(1) }}</span> /
                            <span class="p99">{{ framework.cold_start.duration.p99 | round(1) }}</span>
                        </td>
                    </tr>
                    {% endif %}
                    {% endfor %}
                </tbody>
            </table>
        </section>
    </div>
</body>
</html>
"#;
