//! Filesystem adapter — `FileReader` and `ConfigLoader` implementations.

mod config_dto;
pub mod config_loader;
pub mod file_reader;

pub use config_loader::FsConfigLoader;
pub use file_reader::FsFileReader;
