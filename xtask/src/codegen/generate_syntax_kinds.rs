use anyhow::Result;
use heck::ToShoutySnakeCase;
use proc_macro2::Literal;
use proc_macro2::Punct;
use proc_macro2::Spacing;
use quote::format_ident;
use quote::quote;

use super::kinds::AstKinds;
use super::SharedQuotes;

pub fn generate_syntax_kinds(grammar: AstKinds) -> Result<String> {
  let shared_quotes = SharedQuotes {};
  let syntax_kind = shared_quotes.syntax_kind();
  let punctuation_values = grammar
    .punctuation
    .iter()
    .map(|(token, _name)| {
      // These tokens, when parsed to proc_macro2::TokenStream, generates a stream of
      // bytes that can't be recognized by [quote].
      // Hence, they need to be thread differently
      if "{}[]()`".contains(token) {
        let c = token.chars().next().unwrap();
        quote! { #c }
      } else if ["$=", "//"].contains(token) {
        let token = Literal::string(token);
        quote! { #token }
      } else {
        let cs = token.chars().map(|c| Punct::new(c, Spacing::Joint));
        quote! { #(#cs)* }
      }
    })
    .collect::<Vec<_>>();
  let punctuation_strings = grammar.punctuation.iter().map(|(token, _name)| token);

  let punctuation = grammar
    .punctuation
    .iter()
    .map(|(_token, name)| format_ident!("{}", name))
    .collect::<Vec<_>>();

  let full_keywords_values = &grammar.keywords;
  let full_keywords = full_keywords_values
    .iter()
    .map(|kw| format_ident!("{}_KW", kw.to_shouty_snake_case()))
    .collect::<Vec<_>>();
  let full_reserved_keywords_values = &grammar.reserved_keywords;
  let full_reserved_keywords = full_reserved_keywords_values
    .iter()
    .map(|kw| format_ident!("RESERVED_{}_KW", kw.to_shouty_snake_case()))
    .collect::<Vec<_>>();
  let full_edgeql_keywords_values = &grammar.edgeql_keywords;
  let full_edgeql_keywords = full_edgeql_keywords_values
    .iter()
    .map(|kw| format_ident!("EDGE_{}_KW", kw.to_shouty_snake_case()))
    .collect::<Vec<_>>();
  let full_sdl_keywords_values = &grammar.sdl_keywords;
  let full_sdl_keywords = full_sdl_keywords_values
    .iter()
    .map(|kw| format_ident!("SDL_{}_KW", kw.to_shouty_snake_case()))
    .collect::<Vec<_>>();
  let full_ddl_keywords_values = &grammar.ddl_keywords;
  let full_ddl_keywords = full_ddl_keywords_values
    .iter()
    .map(|kw| format_ident!("DDL_{}_KW", kw.to_shouty_snake_case()))
    .collect::<Vec<_>>();

  let all_keywords_values = grammar.keywords.to_vec();
  let all_keywords_idents = all_keywords_values
    .iter()
    .map(|kw| {
      println!("kw: {}", kw);
      format_ident!("{}", kw)
    })
    .collect::<Vec<_>>();
  let all_keywords = all_keywords_values
    .iter()
    .map(|name| format_ident!("{}_KW", name.to_shouty_snake_case()))
    .collect::<Vec<_>>();
  let all_keyword_strings = all_keywords_values.iter().map(|name| name.to_string());

  let all_reserved_keywords_values = grammar.reserved_keywords.to_vec();
  let all_reserved_keywords_idents = all_reserved_keywords_values
    .iter()
    .map(|kw| format_ident!("{}", kw))
    .collect::<Vec<_>>();
  let all_reserved_keywords = all_reserved_keywords_values
    .iter()
    .map(|name| format_ident!("RESERVED_{}_KW", name.to_shouty_snake_case()))
    .collect::<Vec<_>>();
  let all_reserved_keyword_strings = all_reserved_keywords_values
    .iter()
    .map(|name| name.to_string());

  let all_edgeql_keywords_values = grammar.edgeql_keywords.to_vec();
  let all_edgeql_keywords_idents = all_edgeql_keywords_values
    .iter()
    .map(|kw| format_ident!("{}", kw))
    .collect::<Vec<_>>();
  let all_edgeql_keywords = all_edgeql_keywords_values
    .iter()
    .map(|name| format_ident!("EDGE_{}_KW", name.to_shouty_snake_case()))
    .collect::<Vec<_>>();
  let all_edgeql_keyword_strings = all_edgeql_keywords_values
    .iter()
    .map(|name| name.to_string());

  let all_sdl_keywords_values = grammar.sdl_keywords.to_vec();
  let all_sdl_keywords_idents = all_sdl_keywords_values
    .iter()
    .map(|kw| format_ident!("{}", kw))
    .collect::<Vec<_>>();
  let all_sdl_keywords = all_sdl_keywords_values
    .iter()
    .map(|name| format_ident!("SDL_{}_KW", name.to_shouty_snake_case()))
    .collect::<Vec<_>>();
  let all_sdl_keyword_strings = all_sdl_keywords_values.iter().map(|name| name.to_string());

  let all_ddl_keywords_values = grammar.ddl_keywords.to_vec();
  let all_ddl_keywords_idents = all_ddl_keywords_values
    .iter()
    .map(|kw| format_ident!("{}", kw))
    .collect::<Vec<_>>();
  let all_ddl_keywords = all_ddl_keywords_values
    .iter()
    .map(|name| format_ident!("DDL_{}_KW", name.to_shouty_snake_case()))
    .collect::<Vec<_>>();
  let all_ddl_keyword_strings = all_ddl_keywords_values.iter().map(|name| name.to_string());

  let literals = grammar
    .literals
    .iter()
    .map(|name| format_ident!("{}", name))
    .collect::<Vec<_>>();

  let tokens = grammar
    .tokens
    .iter()
    .map(|name| format_ident!("{}", name))
    .collect::<Vec<_>>();

  let nodes = grammar
    .nodes
    .iter()
    .map(|name| format_ident!("{}", name))
    .collect::<Vec<_>>();

  let lists = grammar
    .nodes
    .iter()
    .filter_map(|name| {
      if name.ends_with("_LIST") {
        Some(format_ident!("{}", name))
      } else {
        None
      }
    })
    .collect::<Vec<_>>();

  let syntax_kind_impl = quote! {
    pub const fn to_string(&self) -> Option<&'static str> {
      let tok = match self {
        #(#punctuation => #punctuation_strings,)*
        #(#all_keywords => #all_keyword_strings,)*
        #(#all_reserved_keywords => #all_reserved_keyword_strings,)*
        #(#all_edgeql_keywords => #all_edgeql_keyword_strings,)*
        #(#all_sdl_keywords => #all_sdl_keyword_strings,)*
        #(#all_ddl_keywords => #all_ddl_keyword_strings,)*
        STRING_LITERAL => "string literal",
        _ => return None,
      };

      Some(tok)
    }
  };

  let is_list_ast = if lists.is_empty() {
    quote! {}
  } else {
    quote! { #(#lists)|* => true, }
  };
  let ast = quote! {
      #![allow(clippy::all)]
      #![allow(bad_style, missing_docs, unreachable_pub)]
      /// The kind of syntax node, e.g. `IDENT`, `FUNCTION_KW`, or `FOR_STMT`.
      #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
      #[repr(u16)]
      pub enum #syntax_kind {
        // Technical SyntaxKinds: they appear temporally during parsing,
        // but never end up in the final tree
        #[doc(hidden)]
        TOMBSTONE,
        /// Marks the end of the file. May have trivia attached.
        EOF,
        #(#punctuation,)*
        #(#all_keywords,)*
        #(#all_reserved_keywords,)*
        #(#all_edgeql_keywords,)*
        #(#all_sdl_keywords,)*
        #(#all_ddl_keywords,)*
        #(#literals,)*
        #(#tokens,)*
        #(#nodes,)*

        // Technical kind so that we can cast from u16 safely
        #[doc(hidden)]
        __LAST,
      }
      use self::#syntax_kind::*;

      impl #syntax_kind {
        pub const fn is_punct(self) -> bool {
          match self {
            #(#punctuation)|* => true,
            _ => false,
          }
        }

        pub const fn is_literal(self) -> bool {
          match self {
            #(#literals)|* => true,
            _ => false,
          }
        }

        pub const fn is_list(self) -> bool {
          match self {
            #is_list_ast
            _ => false,
          }
        }

        pub fn from_keyword(ident: &str) -> Option<#syntax_kind> {
          let kw = match ident {
            #(#full_keywords_values => #full_keywords,)*
            #(#full_reserved_keywords_values => #full_reserved_keywords,)*
            #(#full_edgeql_keywords_values => #full_edgeql_keywords,)*
            #(#full_sdl_keywords_values => #full_sdl_keywords,)*
            #(#full_ddl_keywords_values => #full_ddl_keywords,)*
            _ => return None,
          };
          Some(kw)
        }

        #syntax_kind_impl

      }

      /// Utility macro for creating a SyntaxKind through simple macro syntax
      #[macro_export]
      macro_rules! T {
          #([#punctuation_values] => { $crate::#syntax_kind::#punctuation };)*
          #([#all_keywords_idents] => { $crate::#syntax_kind::#all_keywords };)*
          #([#all_reserved_keywords_idents] => { $crate::#syntax_kind::#all_reserved_keywords };)*
          #([#all_edgeql_keywords_idents] => { $crate::#syntax_kind::#all_edgeql_keywords };)*
          #([#all_sdl_keywords_idents] => { $crate::#syntax_kind::#all_sdl_keywords };)*
          #([#all_ddl_keywords_idents] => { $crate::#syntax_kind::#all_ddl_keywords };)*
          [ident] => { $crate::#syntax_kind::IDENT };
          [EOF] => { $crate::#syntax_kind::EOF };
          [#] => { $crate::#syntax_kind::HASH };
      }
  };

  Ok(ast.to_string())
}
