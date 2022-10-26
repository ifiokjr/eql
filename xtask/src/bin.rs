use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path;
use std::path::Path;
use std::path::PathBuf;
use std::process;

use anyhow::anyhow;
use anyhow::Result;
use cargo_metadata::MetadataCommand;
use cargo_toml::Dependency;
use cargo_toml::Manifest;
use clap::Parser;
use fstrings::f;
use fstrings::format_args_f;

#[derive(Debug)]
struct PkgVersion {
  name: String,
  version: String,
}

struct RootDeps {
  deps: BTreeMap<String, Dependency>,
  path: PathBuf,
  valid: bool,
}

fn get_root_deps<P: AsRef<Path>>(current_dir: P) -> RootDeps {
  let mut current_dir = current_dir.as_ref().to_path_buf();
  println!("current_dir: {:?}", current_dir);

  loop {
    let file_path = current_dir.join("xtask/Cargo.toml");
    println!("file_path: {:?}", &file_path);
    if file_path.exists() {
      let toml = Manifest::from_path(&file_path).unwrap();
      let mut deps = toml.dependencies;
      let mut dev_dependencies = toml.dev_dependencies;
      deps.append(&mut dev_dependencies);

      return RootDeps {
        deps,
        path: file_path,
        valid: true,
      };
    }

    if !current_dir.pop() {
      return RootDeps {
        deps: BTreeMap::new(),
        path: current_dir,
        valid: false,
      };
    }
  }
}

fn get_version_from_workspace(dep_details: &Dependency) -> Option<String> {
  match dep_details {
    Dependency::Detailed(dep) => dep.version.to_owned(),
    Dependency::Simple(dep) => Some(dep.to_owned()),
    Dependency::Inherited(_) => None,
  }
}

pub fn get_binaries() -> Result<Vec<String>> {
  let home_dir = home::cargo_home()?;
  let cache_folder = fs::read_dir(home_dir.join("registry/src"))?
    .next()
    .unwrap()
    .unwrap()
    .path();

  let mut binaries: Vec<String> = vec![];
  let RootDeps { deps, .. } = get_root_deps(env::current_dir()?);

  for (dep_name, dep_details) in deps.iter() {
    let version = match get_version_from_workspace(dep_details) {
      Some(version) => version,
      None => continue,
    };

    let crate_folder =
      path::Path::new(cache_folder.to_str().unwrap()).join(f!("{dep_name}-{version}"));

    let dep_manifest =
      Manifest::from_path(crate_folder.clone().join("Cargo.toml").to_str().unwrap())?;

    if !dep_manifest.bin.is_empty() {
      for bin in dep_manifest.bin {
        binaries.push(bin.name.unwrap());
      }
    } else if crate_folder.clone().join("src/main.rs").exists() {
      binaries.push(dep_name.to_owned());
    }
  }

  Ok(binaries)
}

fn get_pkg_version(bin_name: &str) -> Result<PkgVersion> {
  let RootDeps { deps, path, .. } = get_root_deps(env::current_dir()?);

  for (key, details) in deps.iter() {
    if key != bin_name {
      continue;
    }

    let version = get_version_from_workspace(details).unwrap();

    return Ok(PkgVersion {
      name: key.to_owned(),
      version,
    });
  }

  let metadata = MetadataCommand::new()
    .manifest_path(&path) // TODO Delete this my later, and find a way to autodiscover.
    .exec()?;

  let pkg = metadata
    .packages
    .iter()
    .find(|e| {
      return e.targets.iter().any(|t| {
        // println!("{:?}", &t.name);
        t.name == bin_name
      });
    })
    .ok_or_else(|| anyhow!(f!("Package for binary {bin_name} not found")))?;

  Ok(PkgVersion {
    name: pkg.name.to_owned(),
    version: pkg.version.to_string(),
  })
}

fn run_binary(args: &Vec<String>) -> Result<()> {
  let mut args = args.to_owned();

  let mut rust_version = "unknown".to_string();
  if let Some(res) = version_check::triple() {
    if res.1.is_nightly() {
      rust_version = "nightly".to_string();
    } else {
      rust_version = res.0.to_string();
    }
  }

  let bin_name = args[0].to_owned();
  let pkg_version = get_pkg_version(&bin_name)?;
  let cache_path = f!("./.bin/rust-{rust_version}/{pkg_version.name}/{pkg_version.version}");
  let mut cache_bin_path = f!("{cache_path}/bin/{bin_name}");

  let mut env_path = match env::var("PATH") {
    Ok(val) => val,
    Err(_) => "".to_owned(), // TODO throw err;
  };

  if !path::Path::new(&cache_bin_path).exists() {
    println!("Creating directory {cache_path} for {bin_name}");
    fs::create_dir_all(&cache_path)?;
    process::Command::new("cargo")
      .stdout(process::Stdio::inherit())
      .stderr(process::Stdio::inherit())
      .stdin(process::Stdio::inherit())
      .arg("install")
      .arg("--root")
      .arg(&cache_path)
      .arg("--target-dir")
      .arg("./target") // TODO fix target dir alongside cargo.toml later
      .arg("--version")
      .arg(pkg_version.version)
      .arg(pkg_version.name)
      .output()?;
  } else {
    println!("Binary found!");
  }

  args.drain(0..1);
  println!("Running binary {cache_bin_path} with args {args:?}");

  if bin_name.starts_with("cargo-") {
    cache_bin_path = "cargo".to_owned();
    env_path = f!("{cache_path}/bin:{env_path}");

    let mut new_args = vec![bin_name.replace("cargo-", "")];
    new_args.append(&mut args);
    args = new_args;
  }

  let spawn = process::Command::new(cache_bin_path)
    .stdout(process::Stdio::inherit())
    .stderr(process::Stdio::inherit())
    .stdin(process::Stdio::inherit())
    .env("PATH", env_path)
    .args(&args)
    .spawn();

  if let Ok(mut spawn) = spawn {
    let status = spawn
      .wait()?
      .code()
      .ok_or_else(|| anyhow!("Failed to get spawn exit code"))?;
    process::exit(status);
  }

  Err(anyhow!(f!("Process {bin_name} failed to start")))
}

#[non_exhaustive]
#[derive(Debug, Parser)]
#[command(
  about = "Run a local binary",
  long_about = "Taken from `cargo-run-bin`"
)]
pub struct Bin {
  /// Whether to list all available binaries
  #[arg(short, long, default_value_t = false)]
  list: bool,

  /// The name and the rest of the args.
  rest: Vec<String>,
}

impl Bin {
  pub fn run(&self) {
    if self.list {
      let result = get_binaries();

      match result {
        Ok(binaries) => {
          print!("Binaries:\n{}", binaries.join("\n"));
        }
        Err(error) => {
          println!("{}", f!("run-bin failed: {error}"));
        }
      }
    } else {
      let result = run_binary(&self.rest);

      if let Err(error) = result {
        println!("{}", f!("run-bin failed: {error}"));
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use expectest::prelude::*;

  use super::*;

  mod get_pkg_version {
    use super::*;

    #[test]
    fn return_a_name_and_version_number() {
      let res = get_pkg_version("cargo-llvm-cov").unwrap();
      expect!(res.name).to(be_equal_to("cargo-llvm-cov"));
      expect!(res.version).to(be_equal_to("0.1.3"));
    }

    #[test]
    fn return_an_error_when_a_package_is_not_found() {
      match get_pkg_version("does-not-exist") {
        Ok(_res) => panic!("Function should of not succeeded"),
        Err(err) => {
          expect!(err.to_string()).to(be_equal_to("Package for binary does-not-exist not found"));
        }
      }
    }
  }

  mod get_binaries {
    use super::*;

    #[test]
    fn should_execute_successfully() {
      let res = get_binaries().unwrap();
      expect!(res).to(be_equal_to(vec!["cargo-cmd", "cargo-llvm-cov", "petname"]));
    }
  }

  mod run_binary {
    use super::*;

    #[test]
    fn cargo_binary_execute_successfuly() {
      let args: Vec<String> = vec![
        "cargo".to_owned(),
        "bin".to_owned(),
        "cargo-llvm-cov".to_owned(),
        "--help".to_owned(),
      ];
      let bin = Bin::parse_from(args);
      bin.run();
    }

    #[test]
    fn regular_binary_execute_successfuly() {
      let args: Vec<String> = vec![
        "cargo".to_owned(),
        "bin".to_owned(),
        "petname".to_owned(),
        "--help".to_owned(),
      ];
      let bin = Bin::parse_from(args);
      bin.run();
    }
  }
}
