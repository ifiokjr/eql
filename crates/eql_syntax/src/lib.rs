#[macro_use]
mod generated;
mod source_type;
mod syntax_node;

pub use generated::*;
pub use rome_rowan::SyntaxNodeText;
pub use rome_rowan::TextLen;
pub use rome_rowan::TextRange;
pub use rome_rowan::TextSize;
pub use rome_rowan::TokenAtOffset;
pub use rome_rowan::TriviaPieceKind;
pub use rome_rowan::WalkEvent;
pub use syntax_node::*;

impl rome_rowan::SyntaxKind for EqlSyntaxKind {}

#[allow(dead_code)]
#[derive(Debug, Eq, Ord, PartialOrd, PartialEq, Copy, Clone, Hash)]
pub enum OperatorPrecedence {
  /// `union`
  Union,
  /// `if..else`
  Conditional,
  /// `or`
  Or,
  /// `and`
  And,
  /// `not`
  Not,
  /// `=`, `!=`, `?=`, `?!=`
  Equality,
  /// `<`, `>`, `<=`, `>=`
  Compare,
  /// `like`, `ilike`
  Like,
  /// `in`, `not in`
  In,
  /// `is`, `is not`
  Is,
  /// `+<plus>`, `-<minus>`, `++<strplus>`
  Additive,
  /// `*`, `/`, `//`, `%`
  Multiplicative,
  /// `??`
  Coalesce,
  /// `distinct`, unary `-<uminus>`
  Distinct,
  /// `^<pow>`
  Exponential,
  /// `type cast <cast>`
  Cast,
  /// `array[] <arrayidx>`, `str[] <stridx>`, `json[] <jsonidx>`, `bytes[]
  /// <bytesidx>`
  Index,
  /// `detached`
  Detached,
  /// `(<something>)`
  Group,
}

impl OperatorPrecedence {
  /// Returns the operator with the lowest precedence
  pub fn lowest() -> Self {
    OperatorPrecedence::Union
  }

  /// Returns the operator with the highest precedence
  #[allow(dead_code)]
  pub fn highest() -> Self {
    OperatorPrecedence::Detached
  }

  /// Returns `true` if this operator has right to left associativity
  pub fn is_right_to_left(&self) -> bool {
    matches!(
      self,
      OperatorPrecedence::Yield
        | OperatorPrecedence::Assignment
        | OperatorPrecedence::Conditional
        | OperatorPrecedence::Exponential
        | OperatorPrecedence::Update
    )
  }

  /// Returns the precedence for a binary operator token or [None] if the token
  /// isn't a binary operator
  pub fn try_from_binary_operator(kind: EqlSyntaxKind) -> Option<OperatorPrecedence> {
    Some(match kind {
      T![??] => OperatorPrecedence::Coalesce,
      T![or] => OperatorPrecedence::Or,
      T![and] => OperatorPrecedence::And,
      T![=] | T![!=] | T![?!=] | T![?=] => OperatorPrecedence::Equality,
      T![<] | T![>] | T![<=] | T![>=] => OperatorPrecedence::Compare,

      T![in] => OperatorPrecedence::In,
      T![is] => OperatorPrecedence::Is,
      T![+] | T![-] => OperatorPrecedence::Additive,
      T![*] | T![/] | T![%] | T!["//"] => OperatorPrecedence::Multiplicative,
      T![^] => OperatorPrecedence::Exponential,
      _ => return None,
    })
  }

  pub const fn is_bitwise(&self) -> bool {
    matches!(
      self,
      OperatorPrecedence::BitwiseAnd
        | OperatorPrecedence::BitwiseOr
        | OperatorPrecedence::BitwiseXor
    )
  }

  pub const fn is_additive(&self) -> bool {
    matches!(self, OperatorPrecedence::Additive)
  }

  pub const fn is_equality(&self) -> bool {
    matches!(self, OperatorPrecedence::Equality)
  }

  pub const fn is_multiplicative(&self) -> bool {
    matches!(self, OperatorPrecedence::Multiplicative)
  }

  pub const fn is_exponential(&self) -> bool {
    matches!(self, OperatorPrecedence::Exponential)
  }
}
