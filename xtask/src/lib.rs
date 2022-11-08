use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

pub use bin::Bin;
pub use symlink_root::symlink_root;

/// Get the default command for the build in the dist process.
///
/// This is `cargo build --target wasm32-unknown-unknown`.
pub fn default_build_command() -> Command {
  let mut command = Command::new("cargo");
  command.args(["build", "--target", "wasm32-unknown-unknown"]);
  command
}

pub fn project_root() -> PathBuf {
  Path::new(
    &env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned()),
  )
  .ancestors()
  .nth(1)
  .unwrap()
  .to_path_buf()
}

mod bin;
pub mod codegen;
mod symlink_root;
pub mod utils;
