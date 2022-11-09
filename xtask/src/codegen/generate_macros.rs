use anyhow::Result;
use heck::ToShoutySnakeCase;
use quote::format_ident;
use quote::quote;

use super::kinds::AstSrc;
use super::SharedQuotes;

pub fn generate_macros(ast: &AstSrc) -> Result<String> {
  let shared_quotes = SharedQuotes {};
  let syntax_kind = shared_quotes.syntax_kind();
  let syntax_node = shared_quotes.syntax_node();

  let match_arms: Vec<_> = ast
    .nodes
    .iter()
    .map(|node| {
      let name = format_ident!("{}", node.name);
      let node_kind = format_ident!("{}", &node.name.to_shouty_snake_case());
      (name, node_kind)
    })
    .chain(ast.unknowns.iter().map(|node_name| {
      let name = format_ident!("{}", node_name);
      let node_kind = format_ident!("{}", node_name.to_shouty_snake_case());
      (name, node_kind)
    }))
    .chain(ast.lists().map(|(node_name, _)| {
      let name = format_ident!("{}", node_name);
      let node_kind = format_ident!("{}", node_name.to_shouty_snake_case());
      (name, node_kind)
    }))
    .map(|(name, node_kind)| {
      quote! {
        $crate::#syntax_kind::#node_kind => {
          // SAFETY: The call to new_unchecked is guarded by matching on node.kind()
          let $pattern = unsafe { $crate::#name::new_unchecked(node) };
          $body
        }
      }
    })
    .collect();

  let ast = quote! {
    /// Reconstruct an AstNode from a SyntaxNode
    ///
    /// This macros performs a match over the [kind](rome_rowan::SyntaxNode::kind)
    /// of the provided [rome_rowan::SyntaxNode] and constructs the appropriate
    /// AstNode type for it, then execute the provided expression over it.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// map_syntax_node!(syntax_node, node => node.format())
    /// ```
    #[macro_export]
    macro_rules! map_syntax_node {
      ($node:expr, $pattern:pat => $body:expr) => {
        match $node {
          node => match $crate::#syntax_node::kind(&node) {
            #( #match_arms, )*
            _ => unreachable!()
          }
        }
      };
    }

    pub(crate) use map_syntax_node;
  };

  Ok(ast.to_string())
}
