use std::fs;
use std::path::PathBuf;
use std::process;

use anyhow::Result;
use clap::Parser;
use xtask::symlink_root;
use xtask::Bin;
use xtask::DevServer;
use xtask::Watch;

#[derive(Parser)]
struct Options {
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

  #[clap(flatten)]
  base: xtask::Dist,
}

#[derive(Parser)]
enum Command {
  Dist(Build),
  Watch(Watch),
  Start(DevServer),
  Bin(Bin),
  Echo,
  Setup,
}

fn main() -> Result<()> {
  let root = fs::canonicalize(PathBuf::from("./"))?;

  let options: Options = Parser::parse();

  env_logger::builder()
    .filter(Some("xtask"), options.log_level)
    .init();

  match options.cmd {
    Command::Dist(dist) => {
      log::info!("Generating package...");

      let dist_result = dist
        .base
        .static_dir_path("apps/website/static")
        .app_name("kickjump.com")
        .run("website")?;

      if dist.optimize {
        xtask::WasmOpt::level(1)
          .shrink(2)
          .optimize(dist_result.wasm)?;
      }
    }

    Command::Watch(watch) => {
      log::info!("Watching for changes and check...");

      let mut command = process::Command::new("cargo");
      command.arg("check");

      watch.run(command)?;
    }

    Command::Start(arg) => {
      log::info!("Starting the development server...");

      arg.arg("dist").start(xtask::default_dist_dir(false))?;
    }

    Command::Echo => {
      std::process::Command::new("echo")
        .arg("Hello, world!")
        .spawn()?
        .wait()?;
    }

    Command::Setup => {
      symlink_root(root.join("xtask"))?;
    }

    Command::Bin(bin) => {
      bin.run();
    }
  }

  Ok(())
}
