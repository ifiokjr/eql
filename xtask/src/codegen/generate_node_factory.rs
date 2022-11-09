use anyhow::Result;
use heck::ToShoutySnakeCase;
use heck::ToSnakeCase;
use quote::format_ident;
use quote::quote;

use super::kinds::AstSrc;
use super::kinds::Field;
use super::SharedQuotes;

pub fn generate_node_factory(ast: &AstSrc) -> Result<String> {
  let shared_quotes = SharedQuotes {};
  let syntax_crate = shared_quotes.syntax_crate();
  let syntax_kind = shared_quotes.syntax_kind();
  let syntax_token = shared_quotes.syntax_token();
  let syntax_node = shared_quotes.syntax_node();
  let syntax_element = shared_quotes.syntax_element();

  let nodes = ast.nodes.iter().map(|node| {
    let type_name = format_ident!("{}", node.name);
    let kind = format_ident!("{}", &node.name.to_shouty_snake_case());
    let factory_name = format_ident!("{}", &node.name.to_snake_case());

    let (optional, required): (Vec<_>, Vec<_>) =
      node.fields.iter().partition(|field| field.is_optional());

    if optional.is_empty() {
      let (args, slots): (Vec<_>, Vec<_>) = required
        .into_iter()
        .map(|field| {
          let name = field.method_name();
          let type_name = field.ty();

          let arg = quote! { #name: #type_name };

          let slot = match field {
            Field::Token { .. } => {
              quote! { Some(SyntaxElement::Token(#name)) }
            }
            Field::Node { .. } => {
              quote! { Some(SyntaxElement::Node(#name.into_syntax())) }
            }
          };

          (arg, slot)
        })
        .unzip();

      return quote! {
          pub fn #factory_name( #( #args ),* ) -> #type_name {
              #type_name::unwrap_cast(SyntaxNode::new_detached(
                  #syntax_kind::#kind,
                  [#( #slots ),*],
              ))
          }
      };
    }

    let builder_name = format_ident!("{}Builder", node.name);

    let (required_args, required_fields): (Vec<_>, Vec<_>) = required
      .into_iter()
      .map(|field| {
        let name = field.method_name();
        let type_name = field.ty();

        let arg = quote! { #name: #type_name };
        let field = quote! { #name };

        (arg, field)
      })
      .unzip();

    let (optional_builder, optional_methods): (Vec<_>, Vec<_>) = optional
      .into_iter()
      .map(|field| {
        let name = field.method_name();
        let method_name = format_ident!("with_{}", name);
        let type_name = field.ty();

        let field_type = quote! { #name: Option<#type_name> };
        let field_init = quote! { #name: None };

        let method = quote! {
            pub fn #method_name(mut self, #name: #type_name) -> Self {
                self.#name = Some(#name);
                self
            }
        };

        ((field_type, field_init), method)
      })
      .unzip();

    let (optional_fields, optional_inits): (Vec<_>, Vec<_>) = optional_builder.into_iter().unzip();

    let slots: Vec<_> = node
      .fields
      .iter()
      .map(|field| {
        let name = field.method_name();
        match field {
          Field::Token { optional, .. } => {
            if *optional {
              quote! { self.#name.map(|token| SyntaxElement::Token(token)) }
            } else {
              quote! { Some(SyntaxElement::Token(self.#name)) }
            }
          }
          Field::Node { optional, .. } => {
            if *optional {
              quote! { self.#name.map(|token| SyntaxElement::Node(token.into_syntax())) }
            } else {
              quote! { Some(SyntaxElement::Node(self.#name.into_syntax())) }
            }
          }
        }
      })
      .collect();

    quote! {
        pub fn #factory_name( #( #required_args ),* ) -> #builder_name {
            #builder_name {
                #( #required_fields, )*
                #( #optional_inits, )*
            }
        }

        pub struct #builder_name {
            #( #required_args, )*
            #( #optional_fields, )*
        }

        impl #builder_name {
            #( #optional_methods )*
            pub fn build(self) -> #type_name {
                #type_name::unwrap_cast(SyntaxNode::new_detached(
                    #syntax_kind::#kind,
                    [#( #slots ),*],
                ))
            }
        }
    }
  });

  let lists = ast.lists().map(|(name, list)| {
    let list_name = format_ident!("{}", name);
    let kind = format_ident!("{}", name.to_shouty_snake_case());
    let factory_name = format_ident!("{}", name.to_snake_case());
    let item = format_ident!("{}", list.element_name);

    if list.separator.is_some() {
      quote! {
          pub fn #factory_name<I, S>(items: I, separators: S) -> #list_name
          where
              I: IntoIterator<Item = #item>,
              I::IntoIter: ExactSizeIterator,
              S: IntoIterator<Item = #syntax_token>,
              S::IntoIter: ExactSizeIterator,
          {
              let mut items = items.into_iter();
              let mut separators = separators.into_iter();
              let length = items.len() + separators.len();
              #list_name::unwrap_cast(SyntaxNode::new_detached(
                  #syntax_kind::#kind,
                  (0..length).map(|index| {
                      if index % 2 == 0 {
                          Some(items.next()?.into_syntax().into())
                      } else {
                          Some(separators.next()?.into())
                      }
                  }),
              ))
          }
      }
    } else {
      quote! {
          pub fn #factory_name<I>(items: I) -> #list_name
          where
              I: IntoIterator<Item = #item>,
              I::IntoIter: ExactSizeIterator,
          {
              #list_name::unwrap_cast(SyntaxNode::new_detached(
                  #syntax_kind::#kind,
                  items
                      .into_iter()
                      .map(|item| Some(item.into_syntax().into())),
              ))
          }
      }
    }
  });

  let unknowns = ast.unknowns.iter().map(|name| {
    let unknown_name = format_ident!("{}", name);
    let kind = format_ident!("{}", name.to_shouty_snake_case());
    let factory_name = format_ident!("{}", name.to_snake_case());

    quote! {
        pub fn #factory_name<I>(slots: I) -> #unknown_name
        where
            I: IntoIterator<Item = Option<SyntaxElement>>,
            I::IntoIter: ExactSizeIterator,
        {
            #unknown_name::unwrap_cast(SyntaxNode::new_detached(
                #syntax_kind::#kind,
                slots
            ))
        }
    }
  });

  let output = quote! {
      #![allow(clippy::redundant_closure)]
      #![allow(clippy::too_many_arguments)]
      use #syntax_crate::{*, #syntax_token as SyntaxToken, #syntax_node as SyntaxNode, #syntax_element as SyntaxElement};
      use rome_rowan::AstNode;

      #(#nodes)*
      #(#lists)*
      #(#unknowns)*
  };

  Ok(output.to_string())
}
