use std::fmt::Display;

use eql_syntax::EqlSyntaxKind;
use eql_syntax::EqlSyntaxToken;
use rome_rowan::TriviaPiece;
use rome_rowan::TriviaPieceKind;

pub use crate::generated::node_factory::*;

/// Create a new identifier token with no attached trivia
pub fn ident(text: &str) -> EqlSyntaxToken {
  EqlSyntaxToken::new_detached(EqlSyntaxKind::IDENT, text, [], [])
}

/// Create a new identifier token with no attached trivia
pub fn jsx_ident(text: &str) -> EqlSyntaxToken {
  EqlSyntaxToken::new_detached(EqlSyntaxKind::JSX_IDENT, text, [], [])
}

/// Create a new string literal token with no attached trivia
pub fn js_string_literal(text: &str) -> EqlSyntaxToken {
  EqlSyntaxToken::new_detached(
    EqlSyntaxKind::JS_STRING_LITERAL,
    &format!("\"{text}\""),
    [],
    [],
  )
}

/// Create a new string literal token with no attached trivia
pub fn js_number_literal<N>(text: N) -> EqlSyntaxToken
where
  N: Display + Copy,
{
  EqlSyntaxToken::new_detached(EqlSyntaxKind::JS_NUMBER_LITERAL, &text.to_string(), [], [])
}

/// Create a new token with the specified syntax kind and no attached trivia
pub fn token(kind: EqlSyntaxKind) -> EqlSyntaxToken {
  if let Some(text) = kind.to_string() {
    EqlSyntaxToken::new_detached(kind, text, [], [])
  } else {
    panic!("token kind {kind:?} cannot be transformed to text")
  }
}

/// Create a new token with the specified syntax kind, and a whitespace trivia
/// piece on both the leading and trailing positions
pub fn token_decorated_with_space(kind: EqlSyntaxKind) -> EqlSyntaxToken {
  if let Some(text) = kind.to_string() {
    EqlSyntaxToken::new_detached(
      kind,
      &format!(" {text} "),
      [TriviaPiece::new(TriviaPieceKind::Whitespace, 1)],
      [TriviaPiece::new(TriviaPieceKind::Whitespace, 1)],
    )
  } else {
    panic!("token kind {kind:?} cannot be transformed to text")
  }
}

pub fn eof() -> EqlSyntaxToken {
  EqlSyntaxToken::new_detached(EqlSyntaxKind::EOF, "", [], [])
}
