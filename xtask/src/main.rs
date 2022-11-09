use std::fs;
use std::path::PathBuf;
use std::process;

use anyhow::Result;
use clap::Parser;
use xtask::codegen;
use xtask::format_files;
use xtask::symlink_root;
use xtask::Bin;

#[derive(Parser)]
struct Options {
  /// Set the log level to use
  #[clap(long = "log", default_value = "Info")]
  log_level: log::LevelFilter,
  #[clap(subcommand)]
  cmd: Command,
}

#[derive(Parser)]
struct Build {
  /// Optimize the generated package using `wasm-opt`.
  #[clap(long)]
  optimize: bool,
}

#[derive(Parser)]
enum Command {
  Bin(Bin),
  /// Generate code for the project
  Codegen,
  /// Symlink the project and install all binaries.
  Setup,
  /// Run cargo commands serially in the workspace
  Serial(Serial),
}

#[non_exhaustive]
#[derive(Debug, Parser)]
pub struct Serial {
  #[arg(short, long, default_value_t = false)]
  continue_on_error: bool,
  /// The name and the rest of the args.
  rest: Vec<String>,
}

fn main() -> Result<()> {
  let root = fs::canonicalize(PathBuf::from("./"))?;

  let options: Options = Parser::parse();

  env_logger::builder()
    .filter(Some("xtask"), options.log_level)
    .init();

  match options.cmd {
    Command::Codegen => {
      let files = codegen::generate_ast()?;
      format_files(files)?;
    }

    Command::Setup => {
      symlink_root(root.join("xtask"))?;
    }

    Command::Bin(bin) => {
      bin.run();
    }

    Command::Serial(serial) => {
      let mut exit_code = 0;

      for command in serial.rest {
        match process::Command::new("cargo")
          .stdout(process::Stdio::inherit())
          .stderr(process::Stdio::inherit())
          .stdin(process::Stdio::inherit())
          .arg(command)
          .output()
        {
          Ok(_) => {}
          Err(_error) => {
            exit_code = 1;
            if !serial.continue_on_error {
              break;
            }
          }
        };
      }

      process::exit(exit_code);
    }
  }

  Ok(())
}
