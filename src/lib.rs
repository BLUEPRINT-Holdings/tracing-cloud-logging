#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(test), deny(unused_crate_dependencies))]
#![deny(missing_docs, unreachable_pub)]
#![allow(clippy::needless_doctest_main)]
#![doc = include_str!("../README.md")]

// The `valuable` feature pulls in `http`, `url`, `valuable`, and `valuable_serde`, but their
// usage is additionally gated behind the `tracing_unstable` cfg. When the feature is enabled
// without that cfg, these crates are otherwise unused and trip `unused_crate_dependencies`.
#[cfg(all(feature = "valuable", not(tracing_unstable)))]
use {http as _, url as _, valuable as _, valuable_serde as _};

/// Event formatter for Stackdriver
pub mod event_formatter;
mod google;
mod layer;
mod serializers;
mod visitor;
mod writer;

pub use self::google::*;
pub use self::layer::*;
