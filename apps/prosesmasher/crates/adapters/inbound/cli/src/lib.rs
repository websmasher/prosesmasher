//! CLI adapter — composition root and argument handling.

pub mod args;
pub mod checks;
pub mod output;

// Used by the binary target (main.rs), not the library.
use low_expectations as _;
use prosesmasher_adapters_outbound_fs as _;
use prosesmasher_adapters_outbound_parser as _;
use prosesmasher_domain_types as _;
use prosesmasher_ports_outbound_traits as _;
use walkdir as _;
