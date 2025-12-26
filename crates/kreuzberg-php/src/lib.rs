//! Kreuzberg PHP Bindings
//!
//! This module exposes the Rust core extraction API to PHP using ext-php-rs.
//!
//! # Architecture
//!
//! - All extraction logic is in the Rust core (crates/kreuzberg)
//! - PHP is a thin wrapper that adds language-specific features
//! - Zero duplication of core functionality
//! - Modern ext-php-rs patterns throughout

#![cfg_attr(windows, feature(abi_vectorcall))]

use ext_php_rs::prelude::*;

mod config;
mod error;
mod extraction;
mod types;

use config::*;
use extraction::*;
use types::*;

/// Get the Kreuzberg library version.
///
/// # Returns
///
/// Version string in semver format (e.g., "4.0.0-rc.20")
///
/// # Example
///
/// ```php
/// $version = kreuzberg_version();
/// echo "Kreuzberg version: $version\n";
/// ```
#[php_function]
pub fn kreuzberg_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Kreuzberg PHP extension module.
///
/// Exports all extraction functions, configuration types, and error handling.
#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
