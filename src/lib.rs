//! Normalize Vietnamese road-type prefixes without changing the proper name.
//!
//! The crate ships one versioned rule set and deliberately does not accept
//! runtime regular expressions or configuration.

mod normalizer;
mod rules;

pub use normalizer::normalize;

/// Selects how a recognized road-type prefix is transformed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mode {
    /// Replace the prefix with its approved abbreviation.
    Abbreviate,
    /// Apply the rule-specific removal policy.
    Remove,
}
