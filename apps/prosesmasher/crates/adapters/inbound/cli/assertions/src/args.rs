use clap::Parser;
use prosesmasher_adapters_inbound_cli_runtime::args::Args;

pub fn parse_ok<I, T>(argv: I) -> Args
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    match Args::try_parse_from(argv) {
        Ok(args) => args,
        Err(err) => panic!("parse failed: {err}"),
    }
}

pub fn assert_parse_err<I, T>(argv: I, message: &str)
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    assert!(Args::try_parse_from(argv).is_err(), "{message}");
}

pub fn assert_parse_ok<I, T>(argv: I, message: &str)
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    assert!(Args::try_parse_from(argv).is_ok(), "{message}");
}
