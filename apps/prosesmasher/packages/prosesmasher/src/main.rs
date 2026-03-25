//! User-facing `prosesmasher` binary crate.

#[cfg(test)]
use prosesmasher_assertions as _;

fn main() -> std::process::ExitCode {
    prosesmasher_adapters_inbound_cli::main_entry()
}
