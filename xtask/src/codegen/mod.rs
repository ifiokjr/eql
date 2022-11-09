use std::path::Path;

use anyhow::Result;
pub use generate_ast::generate_ast;
use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::fs2;

const SYNTAX_KINDS: &str = "crates/eql_syntax/src/generated/kind.rs";
const AST_NODES: &str = "crates/eql_syntax/src/generated/nodes.rs";
const AST_NODES_MUT: &str = "crates/eql_syntax/src/generated/nodes_mut.rs";
const SYNTAX_FACTORY: &str = "crates/eql_factory/src/generated/syntax_factory.rs";
const NODE_FACTORY: &str = "crates/eql_factory/src/generated/node_factory.rs";
const AST_MACROS: &str = "crates/eql_syntax/src/generated/macros.rs";

pub(crate) struct SharedQuotes;

impl SharedQuotes {
  #[inline]
  pub(crate) fn syntax_crate(&self) -> TokenStream {
    quote! { eql_syntax }
  }

  #[inline]
  pub(crate) fn syntax_kind(&self) -> TokenStream {
    quote! { EqlSyntaxKind }
  }

  #[inline]
  pub(crate) fn syntax_node(&self) -> TokenStream {
    quote! { EqlSyntaxNode }
  }

  #[inline]
  pub(crate) fn syntax_element(&self) -> TokenStream {
    quote! { EqlSyntaxElement }
  }

  #[inline]
  pub(crate) fn syntax_token(&self) -> TokenStream {
    quote! { EqlSyntaxToken }
  }

  #[inline]
  pub(crate) fn syntax_element_children(&self) -> TokenStream {
    quote! { EqlSyntaxElementChildren }
  }

  #[inline]
  pub(crate) fn syntax_list(&self) -> TokenStream {
    quote! { EqlSyntaxList }
  }

  #[inline]
  pub(crate) fn language(&self) -> TokenStream {
    quote! { EqlLanguage }
  }

  #[inline]
  pub(crate) fn factory_kind(&self) -> TokenStream {
    quote! { EqlSyntaxFactory }
  }
}

/// A helper to update file on disk if it has changed.
/// With verify = false,
pub fn update(path: &Path, contents: &str, mode: &Mode) -> Result<UpdateResult> {
  match fs2::read_to_string(path) {
    Ok(old_contents) if old_contents == contents => {
      return Ok(UpdateResult::NotUpdated);
    }
    _ => (),
  }

  if *mode == Mode::Verify {
    anyhow::bail!("`{}` is not up-to-date", path.display());
  }

  eprintln!("updating {}", path.display());
  fs2::write(path, contents)?;
  Ok(UpdateResult::Updated)
}

pub enum UpdateResult {
  NotUpdated,
  Updated,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Mode {
  Overwrite,
  Verify,
}

mod generate_ast;
mod generate_macros;
mod generate_node_factory;
mod generate_nodes;
mod generate_nodes_mut;
mod generate_syntax_factory;
mod generate_syntax_kinds;
mod kinds;
mod macros;
mod nodes;
