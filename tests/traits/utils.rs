//! Tests for utility traits

use cmark_writer::{error::WriteError, traits::utils::*};
use ecow::EcoString;

// Mock error factory for testing
#[derive(Debug)]
struct MockErrorFactory;

impl ErrorFactory<WriteError> for MockErrorFactory {
    fn create_error(&self) -> WriteError {
        WriteError::custom("mock error")
    }

    fn create_error_with_context<S: Into<EcoString>>(&self, context: S) -> WriteError {
        WriteError::custom(format!("mock error with context: {}", context.into()))
    }
}

// Mock configurable processor
#[derive(Debug)]
struct MockConfig {
    enabled: bool,
    priority: u32,
}

#[derive(Debug)]
struct MockConfigurableProcessor {
    config: MockConfig,
}

impl MockConfigurableProcessor {
    fn new() -> Self {
        Self {
            config: MockConfig {
                enabled: true,
                priority: 0,
            },
        }
    }
}

impl ConfigurableProcessor for MockConfigurableProcessor {
    type Config = MockConfig;

    fn configure(&mut self, config: Self::Config) {
        self.config = config;
    }

    fn config(&self) -> &Self::Config {
        &self.config
    }
}

#[test]
fn test_error_context_with_context() {
    let result: Result<(), WriteError> = Err(WriteError::custom("original error"));

    let result_with_context = result.with_context("additional context");

    assert!(result_with_context.is_err());
    let error = result_with_context.unwrap_err();
    let error_msg = error.to_string();
    assert!(error_msg.contains("additional context"));
    assert!(error_msg.contains("original error"));
}

#[test]
fn test_error_context_with_context_fn() {
    let result: Result<(), WriteError> = Err(WriteError::custom("original error"));

    let result_with_context = result.with_context_fn(|| "dynamic context");

    assert!(result_with_context.is_err());
    let error = result_with_context.unwrap_err();
    let error_msg = error.to_string();
    assert!(error_msg.contains("dynamic context"));
    assert!(error_msg.contains("original error"));
}

#[test]
fn test_error_context_success_case() {
    let result: Result<String, WriteError> = Ok("success".to_string());

    let result_with_context = result.with_context("this context won't be used");

    assert!(result_with_context.is_ok());
    assert_eq!(result_with_context.unwrap(), "success");
}

#[test]
fn test_error_context_fn_success_case() {
    let result: Result<String, WriteError> = Ok("success".to_string());

    let result_with_context = result.with_context_fn(|| "this context won't be used");

    assert!(result_with_context.is_ok());
    assert_eq!(result_with_context.unwrap(), "success");
}

#[test]
fn test_error_factory_create_error() {
    let factory = MockErrorFactory;

    let error = factory.create_error();
    assert_eq!(error.to_string(), "Custom error: mock error");
}

#[test]
fn test_error_factory_create_error_with_context() {
    let factory = MockErrorFactory;

    let error = factory.create_error_with_context("test context");
    let error_msg = error.to_string();
    assert!(error_msg.contains("mock error with context: test context"));
}

#[test]
fn test_configurable_processor_configure() {
    let mut processor = MockConfigurableProcessor::new();

    // Test initial config
    assert!(processor.config().enabled);
    assert_eq!(processor.config().priority, 0);

    // Test configuration change
    let new_config = MockConfig {
        enabled: false,
        priority: 100,
    };
    processor.configure(new_config);

    assert!(!processor.config().enabled);
    assert_eq!(processor.config().priority, 100);
}

#[test]
fn test_configurable_processor_config_immutable_access() {
    let processor = MockConfigurableProcessor::new();

    let config_ref = processor.config();
    assert!(config_ref.enabled);
    assert_eq!(config_ref.priority, 0);
}

#[test]
fn test_error_context_chaining() {
    let result: Result<(), WriteError> = Err(WriteError::custom("base error"));

    let result_with_multiple_contexts = result
        .with_context("first context")
        .with_context("second context");

    assert!(result_with_multiple_contexts.is_err());
    let error = result_with_multiple_contexts.unwrap_err();
    let error_msg = error.to_string();
    assert!(error_msg.contains("second context"));
}

#[test]
fn test_error_context_eco_string() {
    let result: Result<(), WriteError> = Err(WriteError::custom("original error"));
    let context: EcoString = "eco string context".into();

    let result_with_context = result.with_context(context);

    assert!(result_with_context.is_err());
    let error = result_with_context.unwrap_err();
    let error_msg = error.to_string();
    assert!(error_msg.contains("eco string context"));
}
