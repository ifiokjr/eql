use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;

use anyhow::Context;
use anyhow::Result;
use heck::ToShoutySnakeCase;
use quote::quote;
use ungrammar::Grammar;

fn main() {
  let grammar = load_grammar();
  let mut unique_names = HashSet::new();

  for node in grammar.iter() {
    unique_names.insert(grammar[node].name.to_shouty_snake_case());
  }

  let mut names = Vec::from_iter(unique_names);
  names.sort_unstable();
  let ident = quote! {
    pub(crate) const NODES: &'_ [&'_ str] = &[
      #(#names),*
    ];
  };

  let dir =
    env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned());
  let path = Path::new(&dir).join("src/codegen/nodes.rs");
  let contents = ident
    .to_string()
    .replace(" : & '_ [& '_ str] = & [", ": &'_ [&'_ str] = &[\n  ")
    .replace(" , ", ",\n  ")
    .replace("\"] ;", "\"\n];");
  update(&path, contents.as_str());
}

fn load_grammar() -> Grammar {
  let grammar_src = include_str!("./edgedb.ungram");
  grammar_src.parse().unwrap()
}

fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String> {
  let path = path.as_ref();
  fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))
}

fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) {
  let path = path.as_ref();
  fs::write(path, contents).unwrap_or_else(|_| panic!("Failed to write {}", path.display()))
}

fn update(path: &Path, contents: &str) {
  match read_to_string(path) {
    Ok(old_contents) if old_contents == contents => {
      println!("cargo:warning=No changes required to nodes.rs");
      return;
    }
    _ => (),
  }

  println!("cargo:warning=Updating path: {:?}", path);
  write(path, contents);
}
