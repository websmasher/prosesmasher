//! Application core — check trait, runner, and all checks.

pub mod check;
pub mod document_policy;
pub mod quality;
pub mod runner;

#[cfg(test)]
pub(crate) mod test_helpers;

use prosesmasher_ports_outbound_traits as _;
