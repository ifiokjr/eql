use anyhow::Result;
use heck::ToShoutySnakeCase;
use quote::format_ident;
use quote::quote;

use super::generate_nodes::token_kind_to_code;
use super::kinds::AstSrc;
use super::kinds::Field;
use super::kinds::TokenKind;
use super::SharedQuotes;

pub fn generate_syntax_factory(ast: &AstSrc) -> Result<String> {
  let shared_quotes = SharedQuotes {};
  let syntax_crate = shared_quotes.syntax_crate();
  let syntax_kind = shared_quotes.syntax_kind();
  let factory_kind = shared_quotes.factory_kind();

  let normal_node_arms = ast.nodes.iter().map(|node| {
    let kind = format_ident!("{}", &node.name.to_shouty_snake_case());
    let expected_len = node.fields.len();

    let fields = node.fields.iter().map(|field| {
      let field_predicate = match field {
        Field::Node { ty, .. } => {
          let ast_type_name = format_ident!("{}", ty);

          quote! {
              #ast_type_name::can_cast(element.kind())
          }
        }
        Field::Token { kind, .. } => {
          match kind {
            TokenKind::Single(expected) => {
              let expected_kind = token_kind_to_code(expected);
              quote! { element.kind() == #expected_kind}
            }
            TokenKind::Many(expected) => {
              let expected_kinds = expected.iter().map(|kind| token_kind_to_code(kind));
              quote! {
                  matches!(element.kind(), #(#expected_kinds)|*)
              }
            }
          }
        }
      };

      quote! {
          if let Some(element) = &current_element {
              if #field_predicate {
                  slots.mark_present();
                  current_element = elements.next();
              }
          }
          slots.next_slot();
      }
    });

    quote! {
        #kind => {
            let mut elements = (&children).into_iter();
            let mut slots: RawNodeSlots<#expected_len> = RawNodeSlots::default();
            let mut current_element = elements.next();

            #(#fields)*

            // Additional unexpected elements
            if current_element.is_some() {
                return RawSyntaxNode::new(
                    #kind.to_unknown(),
                    children.into_iter().map(Some),
                );
            }

            slots.into_node(#kind, children)
        }
    }
  });

  let lists = ast.lists().map(|(name, data)| {
        let element_type = format_ident!("{}", data.element_name);
        let kind = format_ident!("{}", name.to_shouty_snake_case());
        if let Some(separator) = &data.separator {
            let allow_trailing = separator.allow_trailing;
            let separator_kind = token_kind_to_code(&separator.separator_token, );
            quote! {
                #kind => Self::make_separated_list_syntax(kind, children, #element_type::can_cast, #separator_kind, #allow_trailing)
            }
        } else {
            quote! {
                #kind => Self::make_node_list_syntax(kind, children, #element_type::can_cast)
            }
        }
    });

  let unknown_kinds = ast
    .unknowns
    .iter()
    .map(|node| format_ident!("{}", node.to_shouty_snake_case()));

  let output = quote! {
      use #syntax_crate::{*, #syntax_kind, #syntax_kind::*, T};
      use rome_rowan::{AstNode, ParsedChildren, RawNodeSlots, RawSyntaxNode, SyntaxFactory, SyntaxKind};

      #[derive(Debug)]
      pub struct #factory_kind;

      impl SyntaxFactory for #factory_kind {
          type Kind = #syntax_kind;

          #[allow(unused_mut)]
          fn make_syntax(
              kind: Self::Kind,
              children: ParsedChildren<Self::Kind>,
          ) -> RawSyntaxNode<Self::Kind>
          {
              match kind {
                  #(#unknown_kinds)|* => {
                      RawSyntaxNode::new(kind, children.into_iter().map(Some))
                  },
                  #(#normal_node_arms),*,
                  #(#lists),*,
                  _ => unreachable!("Is {:?} a token?", kind),
              }
          }
      }
  };

  let output = output
    .to_string()
    .replace("T ! [ ", "T![")
    .replace(" ] )", "])");

  Ok(output)
}
