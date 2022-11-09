use anyhow::Result;
use quote::format_ident;
use quote::quote;

use super::kinds::AstSrc;
use super::kinds::Field;
use super::SharedQuotes;

pub fn generate_nodes_mut(ast: &AstSrc) -> Result<String> {
  let node_boilerplate_impls: Vec<_> = ast
    .nodes
    .iter()
    .map(|node| {
      let name = format_ident!("{}", node.name);

      let methods: Vec<_> = node
        .fields
        .iter()
        .enumerate()
        .map(|(index, field)| {
          let method_name = format_ident!("with_{}", field.method_name());
          let type_name = field.ty();

          let element = match field {
            Field::Token { .. } => {
              quote! { element }
            }
            Field::Node { .. } => {
              quote! { element.into_syntax() }
            }
          };

          let element = quote! { #element.into() };

          let (arg_type, element) = if field.is_optional() {
            (
              quote! { Option<#type_name> },
              quote! { element.map(|element| #element) },
            )
          } else {
            (quote! { #type_name }, quote! { Some(#element) })
          };

          quote! {
              pub fn #method_name(self, element: #arg_type) -> Self {
                  Self::unwrap_cast(self.syntax.splice_slots(#index..=#index, once(#element)))
              }
          }
        })
        .collect();

      quote! {
          impl #name {
              #(#methods)*
          }
      }
    })
    .collect();

  let shared_quotes = SharedQuotes {};
  let syntax_token = shared_quotes.syntax_token();

  let ast = quote! {
      use std::iter::once;
      use rome_rowan::AstNode;
      use crate::{generated::nodes::*, #syntax_token as SyntaxToken};

      #(#node_boilerplate_impls)*
  };

  let ast = ast
    .to_string()
    .replace("T ! [ ", "T![")
    .replace(" ] )", "])");

  Ok(ast)
}
