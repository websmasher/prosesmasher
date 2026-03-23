//! Internal CLI binary entry point.

use clap as _;
use low_expectations as _;
use prosesmasher_adapters_outbound_fs as _;
use prosesmasher_adapters_outbound_parser as _;
use prosesmasher_app_core as _;
use prosesmasher_domain_types as _;
use prosesmasher_ports_outbound_traits as _;
use serde as _;
use serde_json as _;
use walkdir as _;

fn main() -> std::process::ExitCode {
    prosesmasher_adapters_inbound_cli::main_entry()
}
