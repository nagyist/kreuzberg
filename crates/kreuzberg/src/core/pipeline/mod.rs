//! Post-processing pipeline orchestration.
//!
//! This module orchestrates the post-processing pipeline, executing validators,
//! quality processing, chunking, and custom hooks in the correct order.

mod cache;
mod format;

pub use cache::clear_processor_cache;
pub use format::apply_output_format;

use crate::core::config::ExtractionConfig;
use crate::plugins::ProcessingStage;
use crate::types::ExtractionResult;
use crate::{KreuzbergError, Result};

use cache::{ProcessorCache, PROCESSOR_CACHE};

/// Run the post-processing pipeline on an extraction result.
///
/// Executes post-processing in the following order:
/// 1. Post-Processors - Execute by stage (Early, Middle, Late) to modify/enhance the result
/// 2. Quality Processing - Text cleaning and quality scoring
/// 3. Chunking - Text splitting if enabled
/// 4. Validators - Run validation hooks on the processed result (can fail fast)
///
/// # Arguments
///
/// * `result` - The extraction result to process
/// * `config` - Extraction configuration
///
/// # Returns
///
/// The processed extraction result.
///
/// # Errors
///
/// - Validator errors bubble up immediately
/// - Post-processor errors are caught and recorded in metadata
/// - System errors (IO, RuntimeError equivalents) always bubble up
#[cfg_attr(feature = "otel", tracing::instrument(
    skip(result, config),
    fields(
        pipeline.stage = "post_processing",
        content.length = result.content.len(),
    )
))]
pub async fn run_pipeline(mut result: ExtractionResult, config: &ExtractionConfig) -> Result<ExtractionResult> {
    let pp_config = config.postprocessor.as_ref();
    let postprocessing_enabled = pp_config.is_none_or(|c| c.enabled);

    if postprocessing_enabled {
        initialize_features();
        initialize_processor_cache()?;

        let (early_processors, middle_processors, late_processors) =
            get_processors_from_cache()?;

        execute_processors(
            &mut result,
            config,
            &pp_config,
            early_processors,
            middle_processors,
            late_processors,
        )
        .await?;
    }

    execute_chunking(&mut result, config)?;
    execute_language_detection(&mut result, config)?;
    execute_validators(&result, config).await?;

    // Transform to element-based output if requested
    if config.result_format == crate::types::OutputFormat::ElementBased {
        result.elements = Some(crate::extraction::transform::transform_extraction_result_to_elements(
            &result,
        ));
    }

    // Apply output format conversion as the final step
    apply_output_format(&mut result, config.output_format);

    Ok(result)
}

/// Run the post-processing pipeline synchronously (WASM-compatible version).
///
/// This is a synchronous implementation for WASM and non-async contexts.
/// It performs a subset of the full async pipeline, excluding async post-processors
/// and validators.
///
/// # Arguments
///
/// * `result` - The extraction result to process
/// * `config` - Extraction configuration
///
/// # Returns
///
/// The processed extraction result.
///
/// # Notes
///
/// This function is only available when the `tokio-runtime` feature is disabled.
/// It handles:
/// - Quality processing (if enabled)
/// - Chunking (if enabled)
/// - Language detection (if enabled)
///
/// It does NOT handle:
/// - Async post-processors
/// - Async validators
#[cfg(not(feature = "tokio-runtime"))]
pub fn run_pipeline_sync(mut result: ExtractionResult, config: &ExtractionConfig) -> Result<ExtractionResult> {
    execute_chunking(&mut result, config)?;
    execute_language_detection(&mut result, config)?;

    // Transform to element-based output if requested
    if config.result_format == crate::types::OutputFormat::ElementBased {
        result.elements = Some(crate::extraction::transform::transform_extraction_result_to_elements(
            &result,
        ));
    }

    // Apply output format conversion as the final step
    apply_output_format(&mut result, config.output_format);

    Ok(result)
}

/// Initialize feature-specific systems that may be needed during pipeline execution.
fn initialize_features() {
    #[cfg(any(feature = "keywords-yake", feature = "keywords-rake"))]
    {
        let _ = crate::keywords::ensure_initialized();
    }

    #[cfg(feature = "language-detection")]
    {
        let _ = crate::language_detection::ensure_initialized();
    }

    #[cfg(feature = "chunking")]
    {
        let _ = crate::chunking::ensure_initialized();
    }

    #[cfg(feature = "quality")]
    {
        let registry = crate::plugins::registry::get_post_processor_registry();
        if let Ok(mut reg) = registry.write() {
            let _ = reg.register(std::sync::Arc::new(crate::text::QualityProcessor), 30);
        }
    }
}

/// Initialize the processor cache if not already initialized.
fn initialize_processor_cache() -> Result<()> {
    let mut cache_lock = PROCESSOR_CACHE
        .write()
        .map_err(|e| crate::KreuzbergError::Other(format!("Processor cache lock poisoned: {}", e)))?;
    if cache_lock.is_none() {
        *cache_lock = Some(ProcessorCache::new()?);
    }
    Ok(())
}

/// Get processors from the cache, organized by stage.
fn get_processors_from_cache(
) -> Result<(std::sync::Arc<Vec<std::sync::Arc<dyn crate::plugins::PostProcessor>>>, std::sync::Arc<Vec<std::sync::Arc<dyn crate::plugins::PostProcessor>>>, std::sync::Arc<Vec<std::sync::Arc<dyn crate::plugins::PostProcessor>>>)> {
    let cache_lock = PROCESSOR_CACHE
        .read()
        .map_err(|e| crate::KreuzbergError::Other(format!("Processor cache lock poisoned: {}", e)))?;
    let cache = cache_lock
        .as_ref()
        .ok_or_else(|| crate::KreuzbergError::Other("Processor cache not initialized".to_string()))?;
    Ok((
        std::sync::Arc::clone(&cache.early),
        std::sync::Arc::clone(&cache.middle),
        std::sync::Arc::clone(&cache.late),
    ))
}

/// Execute all registered post-processors by stage.
async fn execute_processors(
    result: &mut ExtractionResult,
    config: &ExtractionConfig,
    pp_config: &Option<&crate::core::config::PostProcessorConfig>,
    early_processors: std::sync::Arc<Vec<std::sync::Arc<dyn crate::plugins::PostProcessor>>>,
    middle_processors: std::sync::Arc<Vec<std::sync::Arc<dyn crate::plugins::PostProcessor>>>,
    late_processors: std::sync::Arc<Vec<std::sync::Arc<dyn crate::plugins::PostProcessor>>>,
) -> Result<()> {
    for (_stage, processors_arc) in [
        (ProcessingStage::Early, early_processors),
        (ProcessingStage::Middle, middle_processors),
        (ProcessingStage::Late, late_processors),
    ] {
        for processor in processors_arc.iter() {
            let processor_name = processor.name();

            let should_run = should_processor_run(pp_config, processor_name);

            if should_run && processor.should_process(result, config) {
                match processor.process(result, config).await {
                    Ok(_) => {}
                    Err(err @ KreuzbergError::Io(_))
                    | Err(err @ KreuzbergError::LockPoisoned(_))
                    | Err(err @ KreuzbergError::Plugin { .. }) => {
                        return Err(err);
                    }
                    Err(err) => {
                        result.metadata.additional.insert(
                            format!("processing_error_{processor_name}"),
                            serde_json::Value::String(err.to_string()),
                        );
                    }
                }
            }
        }
    }
    Ok(())
}

/// Determine if a processor should run based on configuration.
fn should_processor_run(
    pp_config: &Option<&crate::core::config::PostProcessorConfig>,
    processor_name: &str,
) -> bool {
    if let Some(config) = pp_config {
        if let Some(ref enabled_set) = config.enabled_set {
            enabled_set.contains(processor_name)
        } else if let Some(ref disabled_set) = config.disabled_set {
            !disabled_set.contains(processor_name)
        } else if let Some(ref enabled) = config.enabled_processors {
            enabled.iter().any(|name| name == processor_name)
        } else if let Some(ref disabled) = config.disabled_processors {
            !disabled.iter().any(|name| name == processor_name)
        } else {
            true
        }
    } else {
        true
    }
}

/// Execute chunking if configured.
fn execute_chunking(result: &mut ExtractionResult, config: &ExtractionConfig) -> Result<()> {
    #[cfg(feature = "chunking")]
    if let Some(ref chunking_config) = config.chunking {
        let chunk_config = crate::chunking::ChunkingConfig {
            max_characters: chunking_config.max_chars,
            overlap: chunking_config.max_overlap,
            trim: true,
            chunker_type: crate::chunking::ChunkerType::Text,
        };

        let page_boundaries = result.metadata.pages.as_ref().and_then(|ps| ps.boundaries.as_deref());

        match crate::chunking::chunk_text(&result.content, &chunk_config, page_boundaries) {
            Ok(chunking_result) => {
                result.chunks = Some(chunking_result.chunks);

                if let Some(ref chunks) = result.chunks {
                    result.metadata.additional.insert(
                        "chunk_count".to_string(),
                        serde_json::Value::Number(serde_json::Number::from(chunks.len())),
                    );
                }

                #[cfg(feature = "embeddings")]
                if let Some(ref embedding_config) = chunking_config.embedding
                    && let Some(ref mut chunks) = result.chunks
                {
                    match crate::embeddings::generate_embeddings_for_chunks(chunks, embedding_config) {
                        Ok(()) => {
                            result
                                .metadata
                                .additional
                                .insert("embeddings_generated".to_string(), serde_json::Value::Bool(true));
                        }
                        Err(e) => {
                            result
                                .metadata
                                .additional
                                .insert("embedding_error".to_string(), serde_json::Value::String(e.to_string()));
                        }
                    }
                }

                #[cfg(not(feature = "embeddings"))]
                if chunking_config.embedding.is_some() {
                    result.metadata.additional.insert(
                        "embedding_error".to_string(),
                        serde_json::Value::String("Embeddings feature not enabled".to_string()),
                    );
                }
            }
            Err(e) => {
                result
                    .metadata
                    .additional
                    .insert("chunking_error".to_string(), serde_json::Value::String(e.to_string()));
            }
        }
    }

    #[cfg(not(feature = "chunking"))]
    if config.chunking.is_some() {
        result.metadata.additional.insert(
            "chunking_error".to_string(),
            serde_json::Value::String("Chunking feature not enabled".to_string()),
        );
    }

    Ok(())
}

/// Execute language detection if configured.
fn execute_language_detection(result: &mut ExtractionResult, config: &ExtractionConfig) -> Result<()> {
    #[cfg(feature = "language-detection")]
    if let Some(ref lang_config) = config.language_detection {
        match crate::language_detection::detect_languages(&result.content, lang_config) {
            Ok(detected) => {
                result.detected_languages = detected;
            }
            Err(e) => {
                result.metadata.additional.insert(
                    "language_detection_error".to_string(),
                    serde_json::Value::String(e.to_string()),
                );
            }
        }
    }

    #[cfg(not(feature = "language-detection"))]
    if config.language_detection.is_some() {
        result.metadata.additional.insert(
            "language_detection_error".to_string(),
            serde_json::Value::String("Language detection feature not enabled".to_string()),
        );
    }

    Ok(())
}

/// Execute all registered validators.
async fn execute_validators(result: &ExtractionResult, config: &ExtractionConfig) -> Result<()> {
    let validator_registry = crate::plugins::registry::get_validator_registry();
    let validators = {
        let registry = validator_registry
            .read()
            .map_err(|e| crate::KreuzbergError::Other(format!("Validator registry lock poisoned: {}", e)))?;
        registry.get_all()
    };

    if !validators.is_empty() {
        for validator in validators {
            if validator.should_validate(result, config) {
                validator.validate(result, config).await?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::OutputFormat;
    use crate::types::Metadata;
    use lazy_static::lazy_static;

    const VALIDATION_MARKER_KEY: &str = "registry_validation_marker";
    #[cfg(feature = "quality")]
    const QUALITY_VALIDATION_MARKER: &str = "quality_validation_test";
    const POSTPROCESSOR_VALIDATION_MARKER: &str = "postprocessor_validation_test";
    const ORDER_VALIDATION_MARKER: &str = "order_validation_test";

    lazy_static! {
        static ref REGISTRY_TEST_GUARD: std::sync::Mutex<()> = std::sync::Mutex::new(());
    }

    #[tokio::test]
    async fn test_run_pipeline_basic() {
        let mut result = ExtractionResult {
            content: "test".to_string(),
            mime_type: "text/plain".to_string(),
            metadata: Metadata::default(),
            tables: vec![],
            detected_languages: None,
            chunks: None,
            images: None,
            djot_content: None,
            pages: None,
            elements: None,
        };
        result.metadata.additional.insert(
            VALIDATION_MARKER_KEY.to_string(),
            serde_json::json!(ORDER_VALIDATION_MARKER),
        );
        let config = ExtractionConfig::default();

        let processed = run_pipeline(result, &config).await.unwrap();
        assert_eq!(processed.content, "test");
    }

    #[tokio::test]
    #[cfg(feature = "quality")]
    async fn test_pipeline_with_quality_processing() {
        let result = ExtractionResult {
            content: "This is a test document with some meaningful content.".to_string(),
            mime_type: "text/plain".to_string(),
            metadata: Metadata::default(),
            tables: vec![],
            detected_languages: None,
            chunks: None,
            images: None,
            djot_content: None,
            pages: None,
            elements: None,
        };
        let config = ExtractionConfig {
            enable_quality_processing: true,
            ..Default::default()
        };

        let processed = run_pipeline(result, &config).await.unwrap();
        assert!(processed.metadata.additional.contains_key("quality_score"));
    }

    #[tokio::test]
    async fn test_pipeline_without_quality_processing() {
        let result = ExtractionResult {
            content: "test".to_string(),
            mime_type: "text/plain".to_string(),
            metadata: Metadata::default(),
            tables: vec![],
            detected_languages: None,
            chunks: None,
            images: None,
            djot_content: None,
            pages: None,
            elements: None,
        };
        let config = ExtractionConfig {
            enable_quality_processing: false,
            ..Default::default()
        };

        let processed = run_pipeline(result, &config).await.unwrap();
        assert!(!processed.metadata.additional.contains_key("quality_score"));
    }

    #[tokio::test]
    #[cfg(feature = "chunking")]
    async fn test_pipeline_with_chunking() {
        let result = ExtractionResult {
            content: "This is a long text that should be chunked. ".repeat(100),
            mime_type: "text/plain".to_string(),
            metadata: Metadata::default(),
            tables: vec![],
            detected_languages: None,
            chunks: None,
            images: None,
            djot_content: None,
            pages: None,
            elements: None,
        };
        let config = ExtractionConfig {
            chunking: Some(crate::ChunkingConfig {
                max_chars: 500,
                max_overlap: 50,
                embedding: None,
                preset: None,
            }),
            ..Default::default()
        };

        let processed = run_pipeline(result, &config).await.unwrap();
        assert!(processed.metadata.additional.contains_key("chunk_count"));
        let chunk_count = processed.metadata.additional.get("chunk_count").unwrap();
        assert!(chunk_count.as_u64().unwrap() > 1);
    }

    #[tokio::test]
    async fn test_pipeline_without_chunking() {
        let result = ExtractionResult {
            content: "test".to_string(),
            mime_type: "text/plain".to_string(),
            metadata: Metadata::default(),
            tables: vec![],
            detected_languages: None,
            chunks: None,
            images: None,
            djot_content: None,
            pages: None,
            elements: None,
        };
        let config = ExtractionConfig {
            chunking: None,
            ..Default::default()
        };

        let processed = run_pipeline(result, &config).await.unwrap();
        assert!(!processed.metadata.additional.contains_key("chunk_count"));
    }

    #[tokio::test]
    async fn test_pipeline_preserves_metadata() {
        use std::collections::HashMap;
        let mut additional = HashMap::new();
        additional.insert("source".to_string(), serde_json::json!("test"));
        additional.insert("page".to_string(), serde_json::json!(1));

        let result = ExtractionResult {
            content: "test".to_string(),
            mime_type: "text/plain".to_string(),
            metadata: Metadata {
                additional,
                ..Default::default()
            },
            pages: None,
            tables: vec![],
            detected_languages: None,
            chunks: None,
            images: None,
            djot_content: None,
            elements: None,
        };
        let config = ExtractionConfig::default();

        let processed = run_pipeline(result, &config).await.unwrap();
        assert_eq!(
            processed.metadata.additional.get("source").unwrap(),
            &serde_json::json!("test")
        );
        assert_eq!(
            processed.metadata.additional.get("page").unwrap(),
            &serde_json::json!(1)
        );
    }

    #[tokio::test]
    async fn test_pipeline_preserves_tables() {
        use crate::types::Table;

        let table = Table {
            cells: vec![vec!["A".to_string(), "B".to_string()]],
            markdown: "| A | B |".to_string(),
            page_number: 0,
        };

        let result = ExtractionResult {
            content: "test".to_string(),
            mime_type: "text/plain".to_string(),
            metadata: Metadata::default(),
            tables: vec![table],
            detected_languages: None,
            chunks: None,
            images: None,
            djot_content: None,
            pages: None,
            elements: None,
        };
        let config = ExtractionConfig::default();

        let processed = run_pipeline(result, &config).await.unwrap();
        assert_eq!(processed.tables.len(), 1);
        assert_eq!(processed.tables[0].cells.len(), 1);
    }

    #[tokio::test]
    async fn test_pipeline_empty_content() {
        let _guard = REGISTRY_TEST_GUARD.lock().unwrap();

        {
            let registry = crate::plugins::registry::get_post_processor_registry();
            registry.write().unwrap().shutdown_all().unwrap();
        }
        {
            let registry = crate::plugins::registry::get_validator_registry();
            registry.write().unwrap().shutdown_all().unwrap();
        }

        let result = ExtractionResult {
            content: String::new(),
            mime_type: "text/plain".to_string(),
            metadata: Metadata::default(),
            tables: vec![],
            detected_languages: None,
            chunks: None,
            images: None,
            djot_content: None,
            pages: None,
            elements: None,
        };
        let config = ExtractionConfig::default();

        drop(_guard);

        let processed = run_pipeline(result, &config).await.unwrap();
        assert_eq!(processed.content, "");
    }

    #[tokio::test]
    #[cfg(feature = "chunking")]
    async fn test_pipeline_with_all_features() {
        let result = ExtractionResult {
            content: "This is a comprehensive test document. ".repeat(50),
            mime_type: "text/plain".to_string(),
            metadata: Metadata::default(),
            tables: vec![],
            detected_languages: None,
            chunks: None,
            images: None,
            djot_content: None,
            pages: None,
            elements: None,
        };
        let config = ExtractionConfig {
            enable_quality_processing: true,
            chunking: Some(crate::ChunkingConfig {
                max_chars: 500,
                max_overlap: 50,
                embedding: None,
                preset: None,
            }),
            ..Default::default()
        };

        let processed = run_pipeline(result, &config).await.unwrap();
        assert!(processed.metadata.additional.contains_key("quality_score"));
        assert!(processed.metadata.additional.contains_key("chunk_count"));
    }

    #[tokio::test]
    #[cfg(any(feature = "keywords-yake", feature = "keywords-rake"))]
    async fn test_pipeline_with_keyword_extraction() {
        {
            let _guard = REGISTRY_TEST_GUARD.lock().unwrap();
            crate::plugins::registry::get_validator_registry()
                .write()
                .unwrap()
                .shutdown_all()
                .unwrap();
            crate::plugins::registry::get_post_processor_registry()
                .write()
                .unwrap()
                .shutdown_all()
                .unwrap();

            let _ = crate::keywords::register_keyword_processor();
        }

        let result = ExtractionResult {
            content: r#"
Machine learning is a branch of artificial intelligence that focuses on
building systems that can learn from data. Deep learning is a subset of
machine learning that uses neural networks with multiple layers.
Natural language processing enables computers to understand human language.
            "#
            .to_string(),
            mime_type: "text/plain".to_string(),
            metadata: Metadata::default(),
            tables: vec![],
            detected_languages: None,
            chunks: None,
            images: None,
            djot_content: None,
            pages: None,
            elements: None,
        };

        #[cfg(feature = "keywords-yake")]
        let keyword_config = crate::keywords::KeywordConfig::yake();

        #[cfg(all(feature = "keywords-rake", not(feature = "keywords-yake")))]
        let keyword_config = crate::keywords::KeywordConfig::rake();

        let config = ExtractionConfig {
            keywords: Some(keyword_config),
            ..Default::default()
        };

        let processed = run_pipeline(result, &config).await.unwrap();

        assert!(processed.metadata.additional.contains_key("keywords"));

        let keywords_value = processed.metadata.additional.get("keywords").unwrap();
        assert!(keywords_value.is_array());

        let keywords = keywords_value.as_array().unwrap();
        assert!(!keywords.is_empty(), "Should have extracted keywords");

        let first_keyword = &keywords[0];
        assert!(first_keyword.is_object());
        assert!(first_keyword.get("text").is_some());
        assert!(first_keyword.get("score").is_some());
        assert!(first_keyword.get("algorithm").is_some());
    }

    #[tokio::test]
    #[cfg(any(feature = "keywords-yake", feature = "keywords-rake"))]
    async fn test_pipeline_without_keyword_config() {
        {
            let _guard = REGISTRY_TEST_GUARD.lock().unwrap();
        }
        let result = ExtractionResult {
            content: "Machine learning and artificial intelligence.".to_string(),
            mime_type: "text/plain".to_string(),
            metadata: Metadata::default(),
            tables: vec![],
            detected_languages: None,
            chunks: None,
            images: None,
            djot_content: None,
            pages: None,
            elements: None,
        };

        let config = ExtractionConfig {
            keywords: None,
            ..Default::default()
        };

        let processed = run_pipeline(result, &config).await.unwrap();

        assert!(!processed.metadata.additional.contains_key("keywords"));
    }

    #[tokio::test]
    #[cfg(any(feature = "keywords-yake", feature = "keywords-rake"))]
    async fn test_pipeline_keyword_extraction_short_content() {
        let _guard = REGISTRY_TEST_GUARD.lock().unwrap();
        crate::plugins::registry::get_validator_registry()
            .write()
            .unwrap()
            .shutdown_all()
            .unwrap();
        crate::plugins::registry::get_post_processor_registry()
            .write()
            .unwrap()
            .shutdown_all()
            .unwrap();

        let result = ExtractionResult {
            content: "Short text".to_string(),
            mime_type: "text/plain".to_string(),
            metadata: Metadata::default(),
            tables: vec![],
            detected_languages: None,
            chunks: None,
            images: None,
            djot_content: None,
            pages: None,
            elements: None,
        };

        #[cfg(feature = "keywords-yake")]
        let keyword_config = crate::keywords::KeywordConfig::yake();

        #[cfg(all(feature = "keywords-rake", not(feature = "keywords-yake")))]
        let keyword_config = crate::keywords::KeywordConfig::rake();

        let config = ExtractionConfig {
            keywords: Some(keyword_config),
            ..Default::default()
        };

        drop(_guard);

        let processed = run_pipeline(result, &config).await.unwrap();

        assert!(!processed.metadata.additional.contains_key("keywords"));
    }

    #[tokio::test]
    async fn test_postprocessor_runs_before_validator() {
        use crate::plugins::{Plugin, PostProcessor, ProcessingStage, Validator};
        use async_trait::async_trait;
        use std::sync::Arc;

        struct TestPostProcessor;
        impl Plugin for TestPostProcessor {
            fn name(&self) -> &str {
                "test-processor"
            }
            fn version(&self) -> String {
                "1.0.0".to_string()
            }
            fn initialize(&self) -> Result<()> {
                Ok(())
            }
            fn shutdown(&self) -> Result<()> {
                Ok(())
            }
        }

        #[async_trait]
        impl PostProcessor for TestPostProcessor {
            async fn process(&self, result: &mut ExtractionResult, _config: &ExtractionConfig) -> Result<()> {
                result
                    .metadata
                    .additional
                    .insert("processed".to_string(), serde_json::json!(true));
                Ok(())
            }

            fn processing_stage(&self) -> ProcessingStage {
                ProcessingStage::Middle
            }
        }

        struct TestValidator;
        impl Plugin for TestValidator {
            fn name(&self) -> &str {
                "test-validator"
            }
            fn version(&self) -> String {
                "1.0.0".to_string()
            }
            fn initialize(&self) -> Result<()> {
                Ok(())
            }
            fn shutdown(&self) -> Result<()> {
                Ok(())
            }
        }

        #[async_trait]
        impl Validator for TestValidator {
            async fn validate(&self, result: &ExtractionResult, _config: &ExtractionConfig) -> Result<()> {
                let should_validate = result
                    .metadata
                    .additional
                    .get(VALIDATION_MARKER_KEY)
                    .and_then(|v| v.as_str())
                    == Some(POSTPROCESSOR_VALIDATION_MARKER);

                if !should_validate {
                    return Ok(());
                }

                let processed = result
                    .metadata
                    .additional
                    .get("processed")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);

                if !processed {
                    return Err(crate::KreuzbergError::Validation {
                        message: "Post-processor did not run before validator".to_string(),
                        source: None,
                    });
                }
                Ok(())
            }
        }

        let pp_registry = crate::plugins::registry::get_post_processor_registry();
        let val_registry = crate::plugins::registry::get_validator_registry();

        let _guard = REGISTRY_TEST_GUARD.lock().unwrap();
        clear_processor_cache().unwrap();
        pp_registry.write().unwrap().shutdown_all().unwrap();
        val_registry.write().unwrap().shutdown_all().unwrap();
        clear_processor_cache().unwrap();

        {
            let mut registry = pp_registry.write().unwrap();
            registry.register(Arc::new(TestPostProcessor), 0).unwrap();
        }

        {
            let mut registry = val_registry.write().unwrap();
            registry.register(Arc::new(TestValidator)).unwrap();
        }

        // Clear the cache after registering new processors so it rebuilds with the test processors
        clear_processor_cache().unwrap();

        let mut result = ExtractionResult {
            content: "test".to_string(),
            mime_type: "text/plain".to_string(),
            metadata: Metadata::default(),
            tables: vec![],
            detected_languages: None,
            chunks: None,
            images: None,
            djot_content: None,
            pages: None,
            elements: None,
        };
        result.metadata.additional.insert(
            VALIDATION_MARKER_KEY.to_string(),
            serde_json::json!(POSTPROCESSOR_VALIDATION_MARKER),
        );

        let config = ExtractionConfig {
            postprocessor: Some(crate::core::config::PostProcessorConfig {
                enabled: true,
                enabled_set: None,
                disabled_set: None,
                enabled_processors: None,
                disabled_processors: None,
            }),
            ..Default::default()
        };
        drop(_guard);

        let processed = run_pipeline(result, &config).await;

        pp_registry.write().unwrap().shutdown_all().unwrap();
        val_registry.write().unwrap().shutdown_all().unwrap();

        assert!(processed.is_ok(), "Validator should have seen post-processor metadata");
        let processed = processed.unwrap();
        assert_eq!(
            processed.metadata.additional.get("processed"),
            Some(&serde_json::json!(true)),
            "Post-processor metadata should be present"
        );
    }

    #[tokio::test]
    #[cfg(feature = "quality")]
    async fn test_quality_processing_runs_before_validator() {
        let _guard = REGISTRY_TEST_GUARD.lock().unwrap();
        use crate::plugins::{Plugin, Validator};
        use async_trait::async_trait;
        use std::sync::Arc;

        struct QualityValidator;
        impl Plugin for QualityValidator {
            fn name(&self) -> &str {
                "quality-validator"
            }
            fn version(&self) -> String {
                "1.0.0".to_string()
            }
            fn initialize(&self) -> Result<()> {
                Ok(())
            }
            fn shutdown(&self) -> Result<()> {
                Ok(())
            }
        }

        #[async_trait]
        impl Validator for QualityValidator {
            async fn validate(&self, result: &ExtractionResult, _config: &ExtractionConfig) -> Result<()> {
                let should_validate = result
                    .metadata
                    .additional
                    .get(VALIDATION_MARKER_KEY)
                    .and_then(|v| v.as_str())
                    == Some(QUALITY_VALIDATION_MARKER);

                if !should_validate {
                    return Ok(());
                }

                if !result.metadata.additional.contains_key("quality_score") {
                    return Err(crate::KreuzbergError::Validation {
                        message: "Quality processing did not run before validator".to_string(),
                        source: None,
                    });
                }
                Ok(())
            }
        }

        let val_registry = crate::plugins::registry::get_validator_registry();
        {
            let mut registry = val_registry.write().unwrap();
            registry.register(Arc::new(QualityValidator)).unwrap();
        }

        let mut result = ExtractionResult {
            content: "This is meaningful test content for quality scoring.".to_string(),
            mime_type: "text/plain".to_string(),
            metadata: Metadata::default(),
            tables: vec![],
            detected_languages: None,
            chunks: None,
            images: None,
            djot_content: None,
            pages: None,
            elements: None,
        };
        result.metadata.additional.insert(
            VALIDATION_MARKER_KEY.to_string(),
            serde_json::json!(QUALITY_VALIDATION_MARKER),
        );

        let config = ExtractionConfig {
            enable_quality_processing: true,
            ..Default::default()
        };

        drop(_guard);

        let processed = run_pipeline(result, &config).await;

        {
            let mut registry = val_registry.write().unwrap();
            registry.remove("quality-validator").unwrap();
        }

        assert!(processed.is_ok(), "Validator should have seen quality_score");
    }

    #[tokio::test]
    async fn test_multiple_postprocessors_run_before_validator() {
        use crate::plugins::{Plugin, PostProcessor, ProcessingStage, Validator};
        use async_trait::async_trait;
        use std::sync::Arc;

        struct EarlyProcessor;
        impl Plugin for EarlyProcessor {
            fn name(&self) -> &str {
                "early-proc"
            }
            fn version(&self) -> String {
                "1.0.0".to_string()
            }
            fn initialize(&self) -> Result<()> {
                Ok(())
            }
            fn shutdown(&self) -> Result<()> {
                Ok(())
            }
        }

        #[async_trait]
        impl PostProcessor for EarlyProcessor {
            async fn process(&self, result: &mut ExtractionResult, _config: &ExtractionConfig) -> Result<()> {
                let mut order = result
                    .metadata
                    .additional
                    .get("execution_order")
                    .and_then(|v| v.as_array())
                    .cloned()
                    .unwrap_or_default();
                order.push(serde_json::json!("early"));
                result
                    .metadata
                    .additional
                    .insert("execution_order".to_string(), serde_json::json!(order));
                Ok(())
            }

            fn processing_stage(&self) -> ProcessingStage {
                ProcessingStage::Early
            }
        }

        struct LateProcessor;
        impl Plugin for LateProcessor {
            fn name(&self) -> &str {
                "late-proc"
            }
            fn version(&self) -> String {
                "1.0.0".to_string()
            }
            fn initialize(&self) -> Result<()> {
                Ok(())
            }
            fn shutdown(&self) -> Result<()> {
                Ok(())
            }
        }

        #[async_trait]
        impl PostProcessor for LateProcessor {
            async fn process(&self, result: &mut ExtractionResult, _config: &ExtractionConfig) -> Result<()> {
                let mut order = result
                    .metadata
                    .additional
                    .get("execution_order")
                    .and_then(|v| v.as_array())
                    .cloned()
                    .unwrap_or_default();
                order.push(serde_json::json!("late"));
                result
                    .metadata
                    .additional
                    .insert("execution_order".to_string(), serde_json::json!(order));
                Ok(())
            }

            fn processing_stage(&self) -> ProcessingStage {
                ProcessingStage::Late
            }
        }

        struct OrderValidator;
        impl Plugin for OrderValidator {
            fn name(&self) -> &str {
                "order-validator"
            }
            fn version(&self) -> String {
                "1.0.0".to_string()
            }
            fn initialize(&self) -> Result<()> {
                Ok(())
            }
            fn shutdown(&self) -> Result<()> {
                Ok(())
            }
        }

        #[async_trait]
        impl Validator for OrderValidator {
            async fn validate(&self, result: &ExtractionResult, _config: &ExtractionConfig) -> Result<()> {
                let should_validate = result
                    .metadata
                    .additional
                    .get(VALIDATION_MARKER_KEY)
                    .and_then(|v| v.as_str())
                    == Some(ORDER_VALIDATION_MARKER);

                if !should_validate {
                    return Ok(());
                }

                let order = result
                    .metadata
                    .additional
                    .get("execution_order")
                    .and_then(|v| v.as_array())
                    .ok_or_else(|| crate::KreuzbergError::Validation {
                        message: "No execution order found".to_string(),
                        source: None,
                    })?;

                if order.len() != 2 {
                    return Err(crate::KreuzbergError::Validation {
                        message: format!("Expected 2 processors to run, got {}", order.len()),
                        source: None,
                    });
                }

                if order[0] != "early" || order[1] != "late" {
                    return Err(crate::KreuzbergError::Validation {
                        message: format!("Wrong execution order: {:?}", order),
                        source: None,
                    });
                }

                Ok(())
            }
        }

        let pp_registry = crate::plugins::registry::get_post_processor_registry();
        let val_registry = crate::plugins::registry::get_validator_registry();
        let _guard = REGISTRY_TEST_GUARD.lock().unwrap();

        pp_registry.write().unwrap().shutdown_all().unwrap();
        val_registry.write().unwrap().shutdown_all().unwrap();
        clear_processor_cache().unwrap();

        {
            let mut registry = pp_registry.write().unwrap();
            registry.register(Arc::new(EarlyProcessor), 0).unwrap();
            registry.register(Arc::new(LateProcessor), 0).unwrap();
        }

        {
            let mut registry = val_registry.write().unwrap();
            registry.register(Arc::new(OrderValidator)).unwrap();
        }

        // Clear the cache after registering new processors so it rebuilds with the test processors
        clear_processor_cache().unwrap();

        let result = ExtractionResult {
            content: "test".to_string(),
            mime_type: "text/plain".to_string(),
            metadata: Metadata::default(),
            tables: vec![],
            detected_languages: None,
            chunks: None,
            images: None,
            djot_content: None,
            pages: None,
            elements: None,
        };

        let config = ExtractionConfig::default();
        drop(_guard);

        let processed = run_pipeline(result, &config).await;

        pp_registry.write().unwrap().shutdown_all().unwrap();
        val_registry.write().unwrap().shutdown_all().unwrap();
        clear_processor_cache().unwrap();

        assert!(processed.is_ok(), "All processors should run before validator");
    }

    #[tokio::test]
    async fn test_run_pipeline_with_output_format_plain() {
        let result = ExtractionResult {
            content: "test content".to_string(),
            mime_type: "text/plain".to_string(),
            metadata: Metadata::default(),
            tables: vec![],
            detected_languages: None,
            chunks: None,
            images: None,
            pages: None,
            djot_content: None,
            elements: None,
        };

        let config = crate::core::config::ExtractionConfig {
            output_format: OutputFormat::Plain,
            ..Default::default()
        };

        let processed = run_pipeline(result, &config).await.unwrap();
        assert_eq!(processed.content, "test content");
    }

    #[tokio::test]
    async fn test_run_pipeline_with_output_format_djot() {
        use crate::types::{BlockType, DjotContent, FormattedBlock, InlineElement, InlineType};

        let result = ExtractionResult {
            content: "test content".to_string(),
            mime_type: "text/djot".to_string(),
            metadata: Metadata::default(),
            tables: vec![],
            detected_languages: None,
            chunks: None,
            images: None,
            pages: None,
            elements: None,
            djot_content: Some(DjotContent {
                plain_text: "test content".to_string(),
                blocks: vec![FormattedBlock {
                    block_type: BlockType::Paragraph,
                    level: None,
                    inline_content: vec![InlineElement {
                        element_type: InlineType::Text,
                        content: "test content".to_string(),
                        attributes: None,
                        metadata: None,
                    }],
                    attributes: None,
                    language: None,
                    code: None,
                    children: vec![],
                }],
                metadata: Metadata::default(),
                tables: vec![],
                images: vec![],
                links: vec![],
                footnotes: vec![],
                attributes: std::collections::HashMap::new(),
            }),
        };

        let config = crate::core::config::ExtractionConfig {
            output_format: OutputFormat::Djot,
            ..Default::default()
        };

        let processed = run_pipeline(result, &config).await.unwrap();
        // The content should still be present
        assert!(!processed.content.is_empty());
    }

    #[tokio::test]
    async fn test_run_pipeline_with_output_format_html() {
        let result = ExtractionResult {
            content: "test content".to_string(),
            mime_type: "text/plain".to_string(),
            metadata: Metadata::default(),
            tables: vec![],
            detected_languages: None,
            chunks: None,
            images: None,
            pages: None,
            djot_content: None,
            elements: None,
        };

        let config = crate::core::config::ExtractionConfig {
            output_format: OutputFormat::Html,
            ..Default::default()
        };

        let processed = run_pipeline(result, &config).await.unwrap();
        // For non-djot documents, HTML wraps content in <pre> tags
        assert!(processed.content.contains("<pre>"));
        assert!(processed.content.contains("test content"));
        assert!(processed.content.contains("</pre>"));
    }

    #[tokio::test]
    async fn test_run_pipeline_applies_output_format_last() {
        // This test verifies that output format is applied after all other processing
        use crate::types::DjotContent;

        let result = ExtractionResult {
            content: "test".to_string(),
            mime_type: "text/plain".to_string(),
            metadata: Metadata::default(),
            tables: vec![],
            detected_languages: None,
            chunks: None,
            images: None,
            pages: None,
            elements: None,
            djot_content: Some(DjotContent {
                plain_text: "test".to_string(),
                blocks: vec![],
                metadata: Metadata::default(),
                tables: vec![],
                images: vec![],
                links: vec![],
                footnotes: vec![],
                attributes: std::collections::HashMap::new(),
            }),
        };

        let config = crate::core::config::ExtractionConfig {
            output_format: OutputFormat::Djot,
            // Disable other processing to ensure pipeline runs cleanly
            enable_quality_processing: false,
            ..Default::default()
        };

        let processed = run_pipeline(result, &config).await.unwrap();
        // The result should have gone through the pipeline successfully
        assert!(processed.djot_content.is_some());
    }
}
