// Generated file, do not edit by hand, see `xtask/src/codegen`

#![allow(clippy::enum_variant_names)]
#![allow(clippy::match_like_matches_macro)]
use std::fmt::Debug;
use std::fmt::Formatter;

use rome_rowan::support;
use rome_rowan::AstNode;
use rome_rowan::AstNodeList;
use rome_rowan::AstNodeListIterator;
use rome_rowan::AstSeparatedList;
use rome_rowan::AstSeparatedListNodesIterator;
use rome_rowan::RawSyntaxKind;
use rome_rowan::SyntaxKindSet;
use rome_rowan::SyntaxResult;
#[cfg(feature = "serde")]
use serde::ser::SerializeSeq;
use serde::Serialize;
use serde::Serializer;

use crate::macros::map_syntax_node;
use crate::EqlLanguage as Language;
use crate::EqlSyntaxElement as SyntaxElement;
use crate::EqlSyntaxElementChildren as SyntaxElementChildren;
use crate::EqlSyntaxKind::*;
use crate::EqlSyntaxKind::{self as SyntaxKind};
use crate::EqlSyntaxList as SyntaxList;
use crate::EqlSyntaxNode as SyntaxNode;
use crate::EqlSyntaxToken as SyntaxToken;
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ArrayType {
  pub(crate) syntax: SyntaxNode,
}
impl ArrayType {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> ArrayTypeFields {
    ArrayTypeFields {
      array_token: self.array_token(),
      less_token: self.less_token(),
      member: self.member(),
      greater_token: self.greater_token(),
    }
  }

  pub fn array_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }

  pub fn less_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 1usize)
  }

  pub fn member(&self) -> SyntaxResult<PrimitiveType> {
    support::required_node(&self.syntax, 2usize)
  }

  pub fn greater_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 3usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for ArrayType {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ArrayTypeFields {
  pub array_token: SyntaxResult<SyntaxToken>,
  pub less_token: SyntaxResult<SyntaxToken>,
  pub member: SyntaxResult<PrimitiveType>,
  pub greater_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BareBytesLiteralExpression {
  pub(crate) syntax: SyntaxNode,
}
impl BareBytesLiteralExpression {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> BareBytesLiteralExpressionFields {
    BareBytesLiteralExpressionFields {
      r_token: self.r_token(),
      b_token: self.b_token(),
      bare_string_literal_expression: self.bare_string_literal_expression(),
    }
  }

  pub fn r_token(&self) -> Option<SyntaxToken> {
    support::token(&self.syntax, 0usize)
  }

  pub fn b_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 1usize)
  }

  pub fn bare_string_literal_expression(&self) -> SyntaxResult<BareStringLiteralExpression> {
    support::required_node(&self.syntax, 2usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for BareBytesLiteralExpression {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct BareBytesLiteralExpressionFields {
  pub r_token: Option<SyntaxToken>,
  pub b_token: SyntaxResult<SyntaxToken>,
  pub bare_string_literal_expression: SyntaxResult<BareStringLiteralExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BareStringLiteralExpression {
  pub(crate) syntax: SyntaxNode,
}
impl BareStringLiteralExpression {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> BareStringLiteralExpressionFields {
    BareStringLiteralExpressionFields {
      value_token: self.value_token(),
    }
  }

  pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for BareStringLiteralExpression {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct BareStringLiteralExpressionFields {
  pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BigIntLiteralExpression {
  pub(crate) syntax: SyntaxNode,
}
impl BigIntLiteralExpression {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> BigIntLiteralExpressionFields {
    BigIntLiteralExpressionFields {
      value_token: self.value_token(),
    }
  }

  pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for BigIntLiteralExpression {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct BigIntLiteralExpressionFields {
  pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BigIntType {
  pub(crate) syntax: SyntaxNode,
}
impl BigIntType {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> BigIntTypeFields {
    BigIntTypeFields {
      bigint_token: self.bigint_token(),
    }
  }

  pub fn bigint_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for BigIntType {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct BigIntTypeFields {
  pub bigint_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BooleanLiteralExpression {
  pub(crate) syntax: SyntaxNode,
}
impl BooleanLiteralExpression {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> BooleanLiteralExpressionFields {
    BooleanLiteralExpressionFields {
      value_token: self.value_token(),
    }
  }

  pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for BooleanLiteralExpression {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct BooleanLiteralExpressionFields {
  pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BooleanType {
  pub(crate) syntax: SyntaxNode,
}
impl BooleanType {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> BooleanTypeFields {
    BooleanTypeFields {
      bool_token: self.bool_token(),
    }
  }

  pub fn bool_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for BooleanType {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct BooleanTypeFields {
  pub bool_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BytesType {
  pub(crate) syntax: SyntaxNode,
}
impl BytesType {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> BytesTypeFields {
    BytesTypeFields {
      bytes_token: self.bytes_token(),
    }
  }

  pub fn bytes_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for BytesType {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct BytesTypeFields {
  pub bytes_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct DateTimeType {
  pub(crate) syntax: SyntaxNode,
}
impl DateTimeType {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> DateTimeTypeFields {
    DateTimeTypeFields {
      datetime_token: self.datetime_token(),
    }
  }

  pub fn datetime_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for DateTimeType {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct DateTimeTypeFields {
  pub datetime_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct DecimalLiteralExpression {
  pub(crate) syntax: SyntaxNode,
}
impl DecimalLiteralExpression {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> DecimalLiteralExpressionFields {
    DecimalLiteralExpressionFields {
      value_token: self.value_token(),
    }
  }

  pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for DecimalLiteralExpression {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct DecimalLiteralExpressionFields {
  pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct DecimalType {
  pub(crate) syntax: SyntaxNode,
}
impl DecimalType {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> DecimalTypeFields {
    DecimalTypeFields {
      decimal_token: self.decimal_token(),
    }
  }

  pub fn decimal_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for DecimalType {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct DecimalTypeFields {
  pub decimal_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct DotReferenceName {
  pub(crate) syntax: SyntaxNode,
}
impl DotReferenceName {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> DotReferenceNameFields {
    DotReferenceNameFields {
      name_token: self.name_token(),
      dot_token: self.dot_token(),
      path_token: self.path_token(),
    }
  }

  pub fn name_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }

  pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 1usize)
  }

  pub fn path_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 2usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for DotReferenceName {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct DotReferenceNameFields {
  pub name_token: SyntaxResult<SyntaxToken>,
  pub dot_token: SyntaxResult<SyntaxToken>,
  pub path_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct DurationType {
  pub(crate) syntax: SyntaxNode,
}
impl DurationType {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> DurationTypeFields {
    DurationTypeFields {
      duration_token: self.duration_token(),
    }
  }

  pub fn duration_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for DurationType {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct DurationTypeFields {
  pub duration_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct EmptyStatement {
  pub(crate) syntax: SyntaxNode,
}
impl EmptyStatement {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> EmptyStatementFields {
    EmptyStatementFields {
      semicolon_token: self.semicolon_token(),
    }
  }

  pub fn semicolon_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for EmptyStatement {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct EmptyStatementFields {
  pub semicolon_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Expression {
  pub(crate) syntax: SyntaxNode,
}
impl Expression {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> ExpressionFields {
    ExpressionFields {
      unknown: self.unknown(),
    }
  }

  pub fn unknown(&self) -> SyntaxResult<Unknown> {
    support::required_node(&self.syntax, 0usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for Expression {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ExpressionFields {
  pub unknown: SyntaxResult<Unknown>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FloatLiteralExpression {
  pub(crate) syntax: SyntaxNode,
}
impl FloatLiteralExpression {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> FloatLiteralExpressionFields {
    FloatLiteralExpressionFields {
      value_token: self.value_token(),
    }
  }

  pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for FloatLiteralExpression {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct FloatLiteralExpressionFields {
  pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FloatSixtyFourType {
  pub(crate) syntax: SyntaxNode,
}
impl FloatSixtyFourType {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> FloatSixtyFourTypeFields {
    FloatSixtyFourTypeFields {
      float64_token: self.float64_token(),
    }
  }

  pub fn float64_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for FloatSixtyFourType {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct FloatSixtyFourTypeFields {
  pub float64_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FloatThirtyTwoType {
  pub(crate) syntax: SyntaxNode,
}
impl FloatThirtyTwoType {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> FloatThirtyTwoTypeFields {
    FloatThirtyTwoTypeFields {
      float32_token: self.float32_token(),
    }
  }

  pub fn float32_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for FloatThirtyTwoType {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct FloatThirtyTwoTypeFields {
  pub float32_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct IntLiteralExpression {
  pub(crate) syntax: SyntaxNode,
}
impl IntLiteralExpression {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> IntLiteralExpressionFields {
    IntLiteralExpressionFields {
      value_token: self.value_token(),
    }
  }

  pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for IntLiteralExpression {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct IntLiteralExpressionFields {
  pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct IntSixteenType {
  pub(crate) syntax: SyntaxNode,
}
impl IntSixteenType {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> IntSixteenTypeFields {
    IntSixteenTypeFields {
      int16_token: self.int16_token(),
    }
  }

  pub fn int16_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for IntSixteenType {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct IntSixteenTypeFields {
  pub int16_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct IntSixtyFourType {
  pub(crate) syntax: SyntaxNode,
}
impl IntSixtyFourType {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> IntSixtyFourTypeFields {
    IntSixtyFourTypeFields {
      int64_token: self.int64_token(),
    }
  }

  pub fn int64_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for IntSixtyFourType {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct IntSixtyFourTypeFields {
  pub int64_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct IntThirtyTwoType {
  pub(crate) syntax: SyntaxNode,
}
impl IntThirtyTwoType {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> IntThirtyTwoTypeFields {
    IntThirtyTwoTypeFields {
      int32_token: self.int32_token(),
    }
  }

  pub fn int32_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for IntThirtyTwoType {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct IntThirtyTwoTypeFields {
  pub int32_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsonType {
  pub(crate) syntax: SyntaxNode,
}
impl JsonType {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> JsonTypeFields {
    JsonTypeFields {
      json_token: self.json_token(),
    }
  }

  pub fn json_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for JsonType {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JsonTypeFields {
  pub json_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ParameterName {
  pub(crate) syntax: SyntaxNode,
}
impl ParameterName {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> ParameterNameFields {
    ParameterNameFields {
      dollar_token: self.dollar_token(),
      parameter_name_token: self.parameter_name_token(),
    }
  }

  pub fn dollar_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }

  pub fn parameter_name_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 1usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for ParameterName {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ParameterNameFields {
  pub dollar_token: SyntaxResult<SyntaxToken>,
  pub parameter_name_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct QualifiedName {
  pub(crate) syntax: SyntaxNode,
}
impl QualifiedName {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> QualifiedNameFields {
    QualifiedNameFields {
      name_token: self.name_token(),
      namespace_token: self.namespace_token(),
      path_token: self.path_token(),
    }
  }

  pub fn name_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }

  pub fn namespace_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 1usize)
  }

  pub fn path_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 2usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for QualifiedName {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct QualifiedNameFields {
  pub name_token: SyntaxResult<SyntaxToken>,
  pub namespace_token: SyntaxResult<SyntaxToken>,
  pub path_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RangeType {
  pub(crate) syntax: SyntaxNode,
}
impl RangeType {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> RangeTypeFields {
    RangeTypeFields {
      range_token: self.range_token(),
      less_token: self.less_token(),
      member: self.member(),
      greater_token: self.greater_token(),
    }
  }

  pub fn range_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }

  pub fn less_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 1usize)
  }

  pub fn member(&self) -> SyntaxResult<RangeTypeMember> {
    support::required_node(&self.syntax, 2usize)
  }

  pub fn greater_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 3usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for RangeType {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct RangeTypeFields {
  pub range_token: SyntaxResult<SyntaxToken>,
  pub less_token: SyntaxResult<SyntaxToken>,
  pub member: SyntaxResult<RangeTypeMember>,
  pub greater_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RawBytesLiteralExpression {
  pub(crate) syntax: SyntaxNode,
}
impl RawBytesLiteralExpression {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> RawBytesLiteralExpressionFields {
    RawBytesLiteralExpressionFields {
      b_token: self.b_token(),
      raw_string_literal_expression: self.raw_string_literal_expression(),
    }
  }

  pub fn b_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }

  pub fn raw_string_literal_expression(&self) -> SyntaxResult<RawStringLiteralExpression> {
    support::required_node(&self.syntax, 1usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for RawBytesLiteralExpression {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct RawBytesLiteralExpressionFields {
  pub b_token: SyntaxResult<SyntaxToken>,
  pub raw_string_literal_expression: SyntaxResult<RawStringLiteralExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RawStringLiteralExpression {
  pub(crate) syntax: SyntaxNode,
}
impl RawStringLiteralExpression {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> RawStringLiteralExpressionFields {
    RawStringLiteralExpressionFields {
      r_token: self.r_token(),
      value_token: self.value_token(),
    }
  }

  pub fn r_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }

  pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 1usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for RawStringLiteralExpression {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct RawStringLiteralExpressionFields {
  pub r_token: SyntaxResult<SyntaxToken>,
  pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SdlAnnotation {
  pub(crate) syntax: SyntaxNode,
}
impl SdlAnnotation {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> SdlAnnotationFields {
    SdlAnnotationFields {
      annotation_token: self.annotation_token(),
      name: self.name(),
      assign_token: self.assign_token(),
      value: self.value(),
      empty_statement: self.empty_statement(),
    }
  }

  pub fn annotation_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }

  pub fn name(&self) -> SyntaxResult<UnqualifiedName> {
    support::required_node(&self.syntax, 1usize)
  }

  pub fn assign_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 2usize)
  }

  pub fn value(&self) -> SyntaxResult<StringLiteralExpression> {
    support::required_node(&self.syntax, 3usize)
  }

  pub fn empty_statement(&self) -> SyntaxResult<EmptyStatement> {
    support::required_node(&self.syntax, 4usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlAnnotation {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SdlAnnotationFields {
  pub annotation_token: SyntaxResult<SyntaxToken>,
  pub name: SyntaxResult<UnqualifiedName>,
  pub assign_token: SyntaxResult<SyntaxToken>,
  pub value: SyntaxResult<StringLiteralExpression>,
  pub empty_statement: SyntaxResult<EmptyStatement>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SdlAnnotationSchema {
  pub(crate) syntax: SyntaxNode,
}
impl SdlAnnotationSchema {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> SdlAnnotationSchemaFields {
    SdlAnnotationSchemaFields {
      abstract_token: self.abstract_token(),
      inheritable_token: self.inheritable_token(),
      annotation_token: self.annotation_token(),
      name: self.name(),
      body: self.body(),
    }
  }

  pub fn abstract_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }

  pub fn inheritable_token(&self) -> Option<SyntaxToken> {
    support::token(&self.syntax, 1usize)
  }

  pub fn annotation_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 2usize)
  }

  pub fn name(&self) -> SyntaxResult<Name> {
    support::required_node(&self.syntax, 3usize)
  }

  pub fn body(&self) -> SyntaxResult<SdlAnnotationSchemaBody> {
    support::required_node(&self.syntax, 4usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlAnnotationSchema {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SdlAnnotationSchemaFields {
  pub abstract_token: SyntaxResult<SyntaxToken>,
  pub inheritable_token: Option<SyntaxToken>,
  pub annotation_token: SyntaxResult<SyntaxToken>,
  pub name: SyntaxResult<Name>,
  pub body: SyntaxResult<SdlAnnotationSchemaBody>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SdlAnnotationSchemaBlock {
  pub(crate) syntax: SyntaxNode,
}
impl SdlAnnotationSchemaBlock {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> SdlAnnotationSchemaBlockFields {
    SdlAnnotationSchemaBlockFields {
      open_curly_token: self.open_curly_token(),
      annotations: self.annotations(),
      close_curly_token: self.close_curly_token(),
      semicolon_token: self.semicolon_token(),
    }
  }

  pub fn open_curly_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }

  pub fn annotations(&self) -> SdlAnnotationList {
    support::list(&self.syntax, 1usize)
  }

  pub fn close_curly_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 2usize)
  }

  pub fn semicolon_token(&self) -> Option<SyntaxToken> {
    support::token(&self.syntax, 3usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlAnnotationSchemaBlock {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SdlAnnotationSchemaBlockFields {
  pub open_curly_token: SyntaxResult<SyntaxToken>,
  pub annotations: SdlAnnotationList,
  pub close_curly_token: SyntaxResult<SyntaxToken>,
  pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SdlConstraint {
  pub(crate) syntax: SyntaxNode,
}
impl SdlConstraint {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> SdlConstraintFields {
    SdlConstraintFields {
      delegated_token: self.delegated_token(),
      constraint_token: self.constraint_token(),
      name: self.name(),
      args: self.args(),
      subject: self.subject(),
      except: self.except(),
      body: self.body(),
    }
  }

  pub fn delegated_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }

  pub fn constraint_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 1usize)
  }

  pub fn name(&self) -> SyntaxResult<Name> {
    support::required_node(&self.syntax, 2usize)
  }

  pub fn args(&self) -> Option<SdlConstraintArgs> {
    support::node(&self.syntax, 3usize)
  }

  pub fn subject(&self) -> Option<SdlConstraintSubjectExpression> {
    support::node(&self.syntax, 4usize)
  }

  pub fn except(&self) -> Option<SdlConstraintExcept> {
    support::node(&self.syntax, 5usize)
  }

  pub fn body(&self) -> SyntaxResult<SdlConstraintBody> {
    support::required_node(&self.syntax, 6usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlConstraint {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SdlConstraintFields {
  pub delegated_token: SyntaxResult<SyntaxToken>,
  pub constraint_token: SyntaxResult<SyntaxToken>,
  pub name: SyntaxResult<Name>,
  pub args: Option<SdlConstraintArgs>,
  pub subject: Option<SdlConstraintSubjectExpression>,
  pub except: Option<SdlConstraintExcept>,
  pub body: SyntaxResult<SdlConstraintBody>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SdlConstraintArg {
  pub(crate) syntax: SyntaxNode,
}
impl SdlConstraintArg {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> SdlConstraintArgFields {
    SdlConstraintArgFields {
      name: self.name(),
      name_token: self.name_token(),
      value: self.value(),
    }
  }

  pub fn name(&self) -> SyntaxResult<UnqualifiedName> {
    support::required_node(&self.syntax, 0usize)
  }

  pub fn name_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 1usize)
  }

  pub fn value(&self) -> SyntaxResult<Expression> {
    support::required_node(&self.syntax, 2usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlConstraintArg {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SdlConstraintArgFields {
  pub name: SyntaxResult<UnqualifiedName>,
  pub name_token: SyntaxResult<SyntaxToken>,
  pub value: SyntaxResult<Expression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SdlConstraintArgs {
  pub(crate) syntax: SyntaxNode,
}
impl SdlConstraintArgs {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> SdlConstraintArgsFields {
    SdlConstraintArgsFields {
      open_paren_token: self.open_paren_token(),
      args: self.args(),
      close_paren_token: self.close_paren_token(),
    }
  }

  pub fn open_paren_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }

  pub fn args(&self) -> SdlConstraintArgList {
    support::list(&self.syntax, 1usize)
  }

  pub fn close_paren_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 2usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlConstraintArgs {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SdlConstraintArgsFields {
  pub open_paren_token: SyntaxResult<SyntaxToken>,
  pub args: SdlConstraintArgList,
  pub close_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SdlConstraintBlock {
  pub(crate) syntax: SyntaxNode,
}
impl SdlConstraintBlock {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> SdlConstraintBlockFields {
    SdlConstraintBlockFields {
      open_curly_token: self.open_curly_token(),
      annotations: self.annotations(),
      error_message: self.error_message(),
      close_curly_token: self.close_curly_token(),
      semicolon_token: self.semicolon_token(),
    }
  }

  pub fn open_curly_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }

  pub fn annotations(&self) -> SdlAnnotationList {
    support::list(&self.syntax, 1usize)
  }

  pub fn error_message(&self) -> Option<SdlConstraintErrorMessage> {
    support::node(&self.syntax, 2usize)
  }

  pub fn close_curly_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 3usize)
  }

  pub fn semicolon_token(&self) -> Option<SyntaxToken> {
    support::token(&self.syntax, 4usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlConstraintBlock {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SdlConstraintBlockFields {
  pub open_curly_token: SyntaxResult<SyntaxToken>,
  pub annotations: SdlAnnotationList,
  pub error_message: Option<SdlConstraintErrorMessage>,
  pub close_curly_token: SyntaxResult<SyntaxToken>,
  pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SdlConstraintErrorMessage {
  pub(crate) syntax: SyntaxNode,
}
impl SdlConstraintErrorMessage {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> SdlConstraintErrorMessageFields {
    SdlConstraintErrorMessageFields {
      errmessage_token: self.errmessage_token(),
      assign_token: self.assign_token(),
      message: self.message(),
      empty_statement: self.empty_statement(),
    }
  }

  pub fn errmessage_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }

  pub fn assign_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 1usize)
  }

  pub fn message(&self) -> SyntaxResult<StringLiteralExpression> {
    support::required_node(&self.syntax, 2usize)
  }

  pub fn empty_statement(&self) -> SyntaxResult<EmptyStatement> {
    support::required_node(&self.syntax, 3usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlConstraintErrorMessage {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SdlConstraintErrorMessageFields {
  pub errmessage_token: SyntaxResult<SyntaxToken>,
  pub assign_token: SyntaxResult<SyntaxToken>,
  pub message: SyntaxResult<StringLiteralExpression>,
  pub empty_statement: SyntaxResult<EmptyStatement>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SdlConstraintExcept {
  pub(crate) syntax: SyntaxNode,
}
impl SdlConstraintExcept {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> SdlConstraintExceptFields {
    SdlConstraintExceptFields {
      except_token: self.except_token(),
      expression: self.expression(),
    }
  }

  pub fn except_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }

  pub fn expression(&self) -> SyntaxResult<Expression> {
    support::required_node(&self.syntax, 1usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlConstraintExcept {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SdlConstraintExceptFields {
  pub except_token: SyntaxResult<SyntaxToken>,
  pub expression: SyntaxResult<Expression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SdlConstraintSubjectExpression {
  pub(crate) syntax: SyntaxNode,
}
impl SdlConstraintSubjectExpression {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> SdlConstraintSubjectExpressionFields {
    SdlConstraintSubjectExpressionFields {
      on_token: self.on_token(),
      open_paren_token: self.open_paren_token(),
      expression: self.expression(),
      close_paren_token: self.close_paren_token(),
    }
  }

  pub fn on_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }

  pub fn open_paren_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 1usize)
  }

  pub fn expression(&self) -> SyntaxResult<Expression> {
    support::required_node(&self.syntax, 2usize)
  }

  pub fn close_paren_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 3usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlConstraintSubjectExpression {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SdlConstraintSubjectExpressionFields {
  pub on_token: SyntaxResult<SyntaxToken>,
  pub open_paren_token: SyntaxResult<SyntaxToken>,
  pub expression: SyntaxResult<Expression>,
  pub close_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SdlEnumDeclaration {
  pub(crate) syntax: SyntaxNode,
}
impl SdlEnumDeclaration {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> SdlEnumDeclarationFields {
    SdlEnumDeclarationFields {
      enum_token: self.enum_token(),
      less_token: self.less_token(),
      members: self.members(),
      greater_token: self.greater_token(),
    }
  }

  pub fn enum_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }

  pub fn less_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 1usize)
  }

  pub fn members(&self) -> SdlEnumDeclarationMembers {
    support::list(&self.syntax, 2usize)
  }

  pub fn greater_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 3usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlEnumDeclaration {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SdlEnumDeclarationFields {
  pub enum_token: SyntaxResult<SyntaxToken>,
  pub less_token: SyntaxResult<SyntaxToken>,
  pub members: SdlEnumDeclarationMembers,
  pub greater_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SdlExtending {
  pub(crate) syntax: SyntaxNode,
}
impl SdlExtending {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> SdlExtendingFields {
    SdlExtendingFields {
      extending_token: self.extending_token(),
      extends: self.extends(),
    }
  }

  pub fn extending_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }

  pub fn extends(&self) -> SdlExtendingNames {
    support::list(&self.syntax, 1usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlExtending {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SdlExtendingFields {
  pub extending_token: SyntaxResult<SyntaxToken>,
  pub extends: SdlExtendingNames,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SdlIndex {
  pub(crate) syntax: SyntaxNode,
}
impl SdlIndex {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> SdlIndexFields {
    SdlIndexFields {
      sdl_unknown: self.sdl_unknown(),
    }
  }

  pub fn sdl_unknown(&self) -> SyntaxResult<SdlUnknown> {
    support::required_node(&self.syntax, 0usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlIndex {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SdlIndexFields {
  pub sdl_unknown: SyntaxResult<SdlUnknown>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SdlLink {
  pub(crate) syntax: SyntaxNode,
}
impl SdlLink {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> SdlLinkFields {
    SdlLinkFields {
      sdl_unknown: self.sdl_unknown(),
    }
  }

  pub fn sdl_unknown(&self) -> SyntaxResult<SdlUnknown> {
    support::required_node(&self.syntax, 0usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlLink {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SdlLinkFields {
  pub sdl_unknown: SyntaxResult<SdlUnknown>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SdlModule {
  pub(crate) syntax: SyntaxNode,
}
impl SdlModule {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> SdlModuleFields {
    SdlModuleFields {
      module_token: self.module_token(),
      unqualified_name: self.unqualified_name(),
      open_curly_token: self.open_curly_token(),
      statements: self.statements(),
      close_curly_token: self.close_curly_token(),
      semicolon_token: self.semicolon_token(),
    }
  }

  pub fn module_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }

  pub fn unqualified_name(&self) -> SyntaxResult<UnqualifiedName> {
    support::required_node(&self.syntax, 1usize)
  }

  pub fn open_curly_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 2usize)
  }

  pub fn statements(&self) -> SdlSchemaStatements {
    support::list(&self.syntax, 3usize)
  }

  pub fn close_curly_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 4usize)
  }

  pub fn semicolon_token(&self) -> Option<SyntaxToken> {
    support::token(&self.syntax, 5usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlModule {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SdlModuleFields {
  pub module_token: SyntaxResult<SyntaxToken>,
  pub unqualified_name: SyntaxResult<UnqualifiedName>,
  pub open_curly_token: SyntaxResult<SyntaxToken>,
  pub statements: SdlSchemaStatements,
  pub close_curly_token: SyntaxResult<SyntaxToken>,
  pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SdlObjectBlock {
  pub(crate) syntax: SyntaxNode,
}
impl SdlObjectBlock {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> SdlObjectBlockFields {
    SdlObjectBlockFields {
      open_curly_token: self.open_curly_token(),
      annotations: self.annotations(),
      properties: self.properties(),
      links: self.links(),
      constraints: self.constraints(),
      indexes: self.indexes(),
      close_curly_token: self.close_curly_token(),
      semicolon_token: self.semicolon_token(),
    }
  }

  pub fn open_curly_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }

  pub fn annotations(&self) -> SdlAnnotationList {
    support::list(&self.syntax, 1usize)
  }

  pub fn properties(&self) -> SdlPropertyList {
    support::list(&self.syntax, 2usize)
  }

  pub fn links(&self) -> SdlLinkList {
    support::list(&self.syntax, 3usize)
  }

  pub fn constraints(&self) -> SdlConstraintList {
    support::list(&self.syntax, 4usize)
  }

  pub fn indexes(&self) -> SdlIndexList {
    support::list(&self.syntax, 5usize)
  }

  pub fn close_curly_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 6usize)
  }

  pub fn semicolon_token(&self) -> Option<SyntaxToken> {
    support::token(&self.syntax, 7usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlObjectBlock {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SdlObjectBlockFields {
  pub open_curly_token: SyntaxResult<SyntaxToken>,
  pub annotations: SdlAnnotationList,
  pub properties: SdlPropertyList,
  pub links: SdlLinkList,
  pub constraints: SdlConstraintList,
  pub indexes: SdlIndexList,
  pub close_curly_token: SyntaxResult<SyntaxToken>,
  pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SdlObjectSchema {
  pub(crate) syntax: SyntaxNode,
}
impl SdlObjectSchema {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> SdlObjectSchemaFields {
    SdlObjectSchemaFields {
      abstract_token: self.abstract_token(),
      type_token: self.type_token(),
      name: self.name(),
      extending: self.extending(),
      body: self.body(),
    }
  }

  pub fn abstract_token(&self) -> Option<SyntaxToken> {
    support::token(&self.syntax, 0usize)
  }

  pub fn type_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 1usize)
  }

  pub fn name(&self) -> SyntaxResult<Name> {
    support::required_node(&self.syntax, 2usize)
  }

  pub fn extending(&self) -> Option<SdlExtending> {
    support::node(&self.syntax, 3usize)
  }

  pub fn body(&self) -> SyntaxResult<SdlObjectBody> {
    support::required_node(&self.syntax, 4usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlObjectSchema {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SdlObjectSchemaFields {
  pub abstract_token: Option<SyntaxToken>,
  pub type_token: SyntaxResult<SyntaxToken>,
  pub name: SyntaxResult<Name>,
  pub extending: Option<SdlExtending>,
  pub body: SyntaxResult<SdlObjectBody>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SdlProperty {
  pub(crate) syntax: SyntaxNode,
}
impl SdlProperty {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> SdlPropertyFields {
    SdlPropertyFields {
      sdl_unknown: self.sdl_unknown(),
    }
  }

  pub fn sdl_unknown(&self) -> SyntaxResult<SdlUnknown> {
    support::required_node(&self.syntax, 0usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlProperty {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SdlPropertyFields {
  pub sdl_unknown: SyntaxResult<SdlUnknown>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SdlScalarBlock {
  pub(crate) syntax: SyntaxNode,
}
impl SdlScalarBlock {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> SdlScalarBlockFields {
    SdlScalarBlockFields {
      open_curly_token: self.open_curly_token(),
      annotations: self.annotations(),
      constraints: self.constraints(),
      close_curly_token: self.close_curly_token(),
      semicolon_token: self.semicolon_token(),
    }
  }

  pub fn open_curly_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }

  pub fn annotations(&self) -> SdlAnnotationList {
    support::list(&self.syntax, 1usize)
  }

  pub fn constraints(&self) -> SdlConstraintList {
    support::list(&self.syntax, 2usize)
  }

  pub fn close_curly_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 3usize)
  }

  pub fn semicolon_token(&self) -> Option<SyntaxToken> {
    support::token(&self.syntax, 4usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlScalarBlock {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SdlScalarBlockFields {
  pub open_curly_token: SyntaxResult<SyntaxToken>,
  pub annotations: SdlAnnotationList,
  pub constraints: SdlConstraintList,
  pub close_curly_token: SyntaxResult<SyntaxToken>,
  pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SdlScalarExtendingEnum {
  pub(crate) syntax: SyntaxNode,
}
impl SdlScalarExtendingEnum {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> SdlScalarExtendingEnumFields {
    SdlScalarExtendingEnumFields {
      extending_token: self.extending_token(),
      extends: self.extends(),
    }
  }

  pub fn extending_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }

  pub fn extends(&self) -> SyntaxResult<SdlEnumDeclaration> {
    support::required_node(&self.syntax, 1usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlScalarExtendingEnum {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SdlScalarExtendingEnumFields {
  pub extending_token: SyntaxResult<SyntaxToken>,
  pub extends: SyntaxResult<SdlEnumDeclaration>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SdlScalarExtendingType {
  pub(crate) syntax: SyntaxNode,
}
impl SdlScalarExtendingType {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> SdlScalarExtendingTypeFields {
    SdlScalarExtendingTypeFields {
      extending_token: self.extending_token(),
      extends: self.extends(),
    }
  }

  pub fn extending_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }

  pub fn extends(&self) -> SdlExtendingNames {
    support::list(&self.syntax, 1usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlScalarExtendingType {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SdlScalarExtendingTypeFields {
  pub extending_token: SyntaxResult<SyntaxToken>,
  pub extends: SdlExtendingNames,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SdlScalarSchema {
  pub(crate) syntax: SyntaxNode,
}
impl SdlScalarSchema {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> SdlScalarSchemaFields {
    SdlScalarSchemaFields {
      abstract_token: self.abstract_token(),
      scalar_token: self.scalar_token(),
      type_token: self.type_token(),
      name: self.name(),
      extending: self.extending(),
      body: self.body(),
    }
  }

  pub fn abstract_token(&self) -> Option<SyntaxToken> {
    support::token(&self.syntax, 0usize)
  }

  pub fn scalar_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 1usize)
  }

  pub fn type_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 2usize)
  }

  pub fn name(&self) -> SyntaxResult<Name> {
    support::required_node(&self.syntax, 3usize)
  }

  pub fn extending(&self) -> Option<SdlScalarExtending> {
    support::node(&self.syntax, 4usize)
  }

  pub fn body(&self) -> SyntaxResult<SdlScalarBody> {
    support::required_node(&self.syntax, 5usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlScalarSchema {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SdlScalarSchemaFields {
  pub abstract_token: Option<SyntaxToken>,
  pub scalar_token: SyntaxResult<SyntaxToken>,
  pub type_token: SyntaxResult<SyntaxToken>,
  pub name: SyntaxResult<Name>,
  pub extending: Option<SdlScalarExtending>,
  pub body: SyntaxResult<SdlScalarBody>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SdlSchemaConstrainParam {
  pub(crate) syntax: SyntaxNode,
}
impl SdlSchemaConstrainParam {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> SdlSchemaConstrainParamFields {
    SdlSchemaConstrainParamFields {
      name: self.name(),
      name_token: self.name_token(),
      param_type: self.param_type(),
    }
  }

  pub fn name(&self) -> SyntaxResult<UnqualifiedName> {
    support::required_node(&self.syntax, 0usize)
  }

  pub fn name_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 1usize)
  }

  pub fn param_type(&self) -> SyntaxResult<TypeExpression> {
    support::required_node(&self.syntax, 2usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlSchemaConstrainParam {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SdlSchemaConstrainParamFields {
  pub name: SyntaxResult<UnqualifiedName>,
  pub name_token: SyntaxResult<SyntaxToken>,
  pub param_type: SyntaxResult<TypeExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SdlSchemaConstraint {
  pub(crate) syntax: SyntaxNode,
}
impl SdlSchemaConstraint {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> SdlSchemaConstraintFields {
    SdlSchemaConstraintFields {
      abstract_token: self.abstract_token(),
      constraint_token: self.constraint_token(),
      name: self.name(),
      params: self.params(),
      subject: self.subject(),
      extending: self.extending(),
      body: self.body(),
    }
  }

  pub fn abstract_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }

  pub fn constraint_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 1usize)
  }

  pub fn name(&self) -> SyntaxResult<Name> {
    support::required_node(&self.syntax, 2usize)
  }

  pub fn params(&self) -> Option<SdlSchemaConstraintArgs> {
    support::node(&self.syntax, 3usize)
  }

  pub fn subject(&self) -> Option<SdlConstraintSubjectExpression> {
    support::node(&self.syntax, 4usize)
  }

  pub fn extending(&self) -> Option<SdlExtending> {
    support::node(&self.syntax, 5usize)
  }

  pub fn body(&self) -> SyntaxResult<SdlSchemaConstraintBody> {
    support::required_node(&self.syntax, 6usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlSchemaConstraint {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SdlSchemaConstraintFields {
  pub abstract_token: SyntaxResult<SyntaxToken>,
  pub constraint_token: SyntaxResult<SyntaxToken>,
  pub name: SyntaxResult<Name>,
  pub params: Option<SdlSchemaConstraintArgs>,
  pub subject: Option<SdlConstraintSubjectExpression>,
  pub extending: Option<SdlExtending>,
  pub body: SyntaxResult<SdlSchemaConstraintBody>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SdlSchemaConstraintArgs {
  pub(crate) syntax: SyntaxNode,
}
impl SdlSchemaConstraintArgs {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> SdlSchemaConstraintArgsFields {
    SdlSchemaConstraintArgsFields {
      open_paren_token: self.open_paren_token(),
      params: self.params(),
      close_paren_token: self.close_paren_token(),
    }
  }

  pub fn open_paren_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }

  pub fn params(&self) -> SdlSchemaConstrainParamList {
    support::list(&self.syntax, 1usize)
  }

  pub fn close_paren_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 2usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlSchemaConstraintArgs {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SdlSchemaConstraintArgsFields {
  pub open_paren_token: SyntaxResult<SyntaxToken>,
  pub params: SdlSchemaConstrainParamList,
  pub close_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SdlSchemaConstraintBlock {
  pub(crate) syntax: SyntaxNode,
}
impl SdlSchemaConstraintBlock {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> SdlSchemaConstraintBlockFields {
    SdlSchemaConstraintBlockFields {
      open_curly_token: self.open_curly_token(),
      using: self.using(),
      annotations: self.annotations(),
      error_message: self.error_message(),
      close_curly_token: self.close_curly_token(),
      semicolon_token: self.semicolon_token(),
    }
  }

  pub fn open_curly_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }

  pub fn using(&self) -> SyntaxResult<Expression> {
    support::required_node(&self.syntax, 1usize)
  }

  pub fn annotations(&self) -> SdlAnnotationList {
    support::list(&self.syntax, 2usize)
  }

  pub fn error_message(&self) -> Option<SdlConstraintErrorMessage> {
    support::node(&self.syntax, 3usize)
  }

  pub fn close_curly_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 4usize)
  }

  pub fn semicolon_token(&self) -> Option<SyntaxToken> {
    support::token(&self.syntax, 5usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlSchemaConstraintBlock {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SdlSchemaConstraintBlockFields {
  pub open_curly_token: SyntaxResult<SyntaxToken>,
  pub using: SyntaxResult<Expression>,
  pub annotations: SdlAnnotationList,
  pub error_message: Option<SdlConstraintErrorMessage>,
  pub close_curly_token: SyntaxResult<SyntaxToken>,
  pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SequenceType {
  pub(crate) syntax: SyntaxNode,
}
impl SequenceType {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> SequenceTypeFields {
    SequenceTypeFields {
      sequence_token: self.sequence_token(),
    }
  }

  pub fn sequence_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for SequenceType {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SequenceTypeFields {
  pub sequence_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct StringType {
  pub(crate) syntax: SyntaxNode,
}
impl StringType {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> StringTypeFields {
    StringTypeFields {
      str_token: self.str_token(),
    }
  }

  pub fn str_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for StringType {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct StringTypeFields {
  pub str_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TupleType {
  pub(crate) syntax: SyntaxNode,
}
impl TupleType {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> TupleTypeFields {
    TupleTypeFields {
      tuple_token: self.tuple_token(),
      less_token: self.less_token(),
      members: self.members(),
      greater_token: self.greater_token(),
    }
  }

  pub fn tuple_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }

  pub fn less_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 1usize)
  }

  pub fn members(&self) -> TupleTypeMembers {
    support::list(&self.syntax, 2usize)
  }

  pub fn greater_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 3usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for TupleType {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct TupleTypeFields {
  pub tuple_token: SyntaxResult<SyntaxToken>,
  pub less_token: SyntaxResult<SyntaxToken>,
  pub members: TupleTypeMembers,
  pub greater_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TypeCastExpression {
  pub(crate) syntax: SyntaxNode,
}
impl TypeCastExpression {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> TypeCastExpressionFields {
    TypeCastExpressionFields {
      less_token: self.less_token(),
      ty: self.ty(),
      greater_token: self.greater_token(),
      target: self.target(),
    }
  }

  pub fn less_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }

  pub fn ty(&self) -> SyntaxResult<TypeExpression> {
    support::required_node(&self.syntax, 1usize)
  }

  pub fn greater_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 2usize)
  }

  pub fn target(&self) -> SyntaxResult<TypeCastTarget> {
    support::required_node(&self.syntax, 3usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for TypeCastExpression {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct TypeCastExpressionFields {
  pub less_token: SyntaxResult<SyntaxToken>,
  pub ty: SyntaxResult<TypeExpression>,
  pub greater_token: SyntaxResult<SyntaxToken>,
  pub target: SyntaxResult<TypeCastTarget>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct UnqualifiedName {
  pub(crate) syntax: SyntaxNode,
}
impl UnqualifiedName {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> UnqualifiedNameFields {
    UnqualifiedNameFields {
      name_token: self.name_token(),
    }
  }

  pub fn name_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for UnqualifiedName {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct UnqualifiedNameFields {
  pub name_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct UuidType {
  pub(crate) syntax: SyntaxNode,
}
impl UuidType {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn as_fields(&self) -> UuidTypeFields {
    UuidTypeFields {
      uuid_token: self.uuid_token(),
    }
  }

  pub fn uuid_token(&self) -> SyntaxResult<SyntaxToken> {
    support::required_token(&self.syntax, 0usize)
  }
}
#[cfg(feature = "serde")]
impl Serialize for UuidType {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self.as_fields().serialize(serializer)
  }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct UuidTypeFields {
  pub uuid_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyLiteralExpression {
  BigIntLiteralExpression(BigIntLiteralExpression),
  BooleanLiteralExpression(BooleanLiteralExpression),
  BytesLiteralExpression(BytesLiteralExpression),
  DecimalLiteralExpression(DecimalLiteralExpression),
  FloatLiteralExpression(FloatLiteralExpression),
  IntLiteralExpression(IntLiteralExpression),
  StringLiteralExpression(StringLiteralExpression),
}
impl AnyLiteralExpression {
  pub fn as_big_int_literal_expression(&self) -> Option<&BigIntLiteralExpression> {
    match &self {
      AnyLiteralExpression::BigIntLiteralExpression(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_boolean_literal_expression(&self) -> Option<&BooleanLiteralExpression> {
    match &self {
      AnyLiteralExpression::BooleanLiteralExpression(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_bytes_literal_expression(&self) -> Option<&BytesLiteralExpression> {
    match &self {
      AnyLiteralExpression::BytesLiteralExpression(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_decimal_literal_expression(&self) -> Option<&DecimalLiteralExpression> {
    match &self {
      AnyLiteralExpression::DecimalLiteralExpression(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_float_literal_expression(&self) -> Option<&FloatLiteralExpression> {
    match &self {
      AnyLiteralExpression::FloatLiteralExpression(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_int_literal_expression(&self) -> Option<&IntLiteralExpression> {
    match &self {
      AnyLiteralExpression::IntLiteralExpression(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_string_literal_expression(&self) -> Option<&StringLiteralExpression> {
    match &self {
      AnyLiteralExpression::StringLiteralExpression(item) => Some(item),
      _ => None,
    }
  }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum BytesLiteralExpression {
  BareBytesLiteralExpression(BareBytesLiteralExpression),
  RawBytesLiteralExpression(RawBytesLiteralExpression),
}
impl BytesLiteralExpression {
  pub fn as_bare_bytes_literal_expression(&self) -> Option<&BareBytesLiteralExpression> {
    match &self {
      BytesLiteralExpression::BareBytesLiteralExpression(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_raw_bytes_literal_expression(&self) -> Option<&RawBytesLiteralExpression> {
    match &self {
      BytesLiteralExpression::RawBytesLiteralExpression(item) => Some(item),
      _ => None,
    }
  }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum Name {
  QualifiedName(QualifiedName),
  UnqualifiedName(UnqualifiedName),
}
impl Name {
  pub fn as_qualified_name(&self) -> Option<&QualifiedName> {
    match &self {
      Name::QualifiedName(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_unqualified_name(&self) -> Option<&UnqualifiedName> {
    match &self {
      Name::UnqualifiedName(item) => Some(item),
      _ => None,
    }
  }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum PrimitiveType {
  BigIntType(BigIntType),
  BooleanType(BooleanType),
  BytesType(BytesType),
  DateTimeType(DateTimeType),
  DecimalType(DecimalType),
  DurationType(DurationType),
  FloatSixtyFourType(FloatSixtyFourType),
  FloatThirtyTwoType(FloatThirtyTwoType),
  IntSixteenType(IntSixteenType),
  IntSixtyFourType(IntSixtyFourType),
  IntThirtyTwoType(IntThirtyTwoType),
  JsonType(JsonType),
  SdlEnumDeclaration(SdlEnumDeclaration),
  SequenceType(SequenceType),
  StringType(StringType),
  UuidType(UuidType),
}
impl PrimitiveType {
  pub fn as_big_int_type(&self) -> Option<&BigIntType> {
    match &self {
      PrimitiveType::BigIntType(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_boolean_type(&self) -> Option<&BooleanType> {
    match &self {
      PrimitiveType::BooleanType(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_bytes_type(&self) -> Option<&BytesType> {
    match &self {
      PrimitiveType::BytesType(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_date_time_type(&self) -> Option<&DateTimeType> {
    match &self {
      PrimitiveType::DateTimeType(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_decimal_type(&self) -> Option<&DecimalType> {
    match &self {
      PrimitiveType::DecimalType(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_duration_type(&self) -> Option<&DurationType> {
    match &self {
      PrimitiveType::DurationType(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_float_sixty_four_type(&self) -> Option<&FloatSixtyFourType> {
    match &self {
      PrimitiveType::FloatSixtyFourType(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_float_thirty_two_type(&self) -> Option<&FloatThirtyTwoType> {
    match &self {
      PrimitiveType::FloatThirtyTwoType(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_int_sixteen_type(&self) -> Option<&IntSixteenType> {
    match &self {
      PrimitiveType::IntSixteenType(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_int_sixty_four_type(&self) -> Option<&IntSixtyFourType> {
    match &self {
      PrimitiveType::IntSixtyFourType(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_int_thirty_two_type(&self) -> Option<&IntThirtyTwoType> {
    match &self {
      PrimitiveType::IntThirtyTwoType(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_json_type(&self) -> Option<&JsonType> {
    match &self {
      PrimitiveType::JsonType(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_sdl_enum_declaration(&self) -> Option<&SdlEnumDeclaration> {
    match &self {
      PrimitiveType::SdlEnumDeclaration(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_sequence_type(&self) -> Option<&SequenceType> {
    match &self {
      PrimitiveType::SequenceType(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_string_type(&self) -> Option<&StringType> {
    match &self {
      PrimitiveType::StringType(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_uuid_type(&self) -> Option<&UuidType> {
    match &self {
      PrimitiveType::UuidType(item) => Some(item),
      _ => None,
    }
  }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum RangeTypeMember {
  DecimalType(DecimalType),
  FloatSixtyFourType(FloatSixtyFourType),
  FloatThirtyTwoType(FloatThirtyTwoType),
  IntSixtyFourType(IntSixtyFourType),
  IntThirtyTwoType(IntThirtyTwoType),
  QualifiedName(QualifiedName),
}
impl RangeTypeMember {
  pub fn as_decimal_type(&self) -> Option<&DecimalType> {
    match &self {
      RangeTypeMember::DecimalType(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_float_sixty_four_type(&self) -> Option<&FloatSixtyFourType> {
    match &self {
      RangeTypeMember::FloatSixtyFourType(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_float_thirty_two_type(&self) -> Option<&FloatThirtyTwoType> {
    match &self {
      RangeTypeMember::FloatThirtyTwoType(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_int_sixty_four_type(&self) -> Option<&IntSixtyFourType> {
    match &self {
      RangeTypeMember::IntSixtyFourType(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_int_thirty_two_type(&self) -> Option<&IntThirtyTwoType> {
    match &self {
      RangeTypeMember::IntThirtyTwoType(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_qualified_name(&self) -> Option<&QualifiedName> {
    match &self {
      RangeTypeMember::QualifiedName(item) => Some(item),
      _ => None,
    }
  }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum SdlAnnotationSchemaBody {
  EmptyStatement(EmptyStatement),
  SdlAnnotationSchemaBlock(SdlAnnotationSchemaBlock),
}
impl SdlAnnotationSchemaBody {
  pub fn as_empty_statement(&self) -> Option<&EmptyStatement> {
    match &self {
      SdlAnnotationSchemaBody::EmptyStatement(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_sdl_annotation_schema_block(&self) -> Option<&SdlAnnotationSchemaBlock> {
    match &self {
      SdlAnnotationSchemaBody::SdlAnnotationSchemaBlock(item) => Some(item),
      _ => None,
    }
  }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum SdlConstraintBody {
  EmptyStatement(EmptyStatement),
  SdlConstraintBlock(SdlConstraintBlock),
}
impl SdlConstraintBody {
  pub fn as_empty_statement(&self) -> Option<&EmptyStatement> {
    match &self {
      SdlConstraintBody::EmptyStatement(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_sdl_constraint_block(&self) -> Option<&SdlConstraintBlock> {
    match &self {
      SdlConstraintBody::SdlConstraintBlock(item) => Some(item),
      _ => None,
    }
  }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum SdlObjectBody {
  EmptyStatement(EmptyStatement),
  SdlObjectBlock(SdlObjectBlock),
}
impl SdlObjectBody {
  pub fn as_empty_statement(&self) -> Option<&EmptyStatement> {
    match &self {
      SdlObjectBody::EmptyStatement(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_sdl_object_block(&self) -> Option<&SdlObjectBlock> {
    match &self {
      SdlObjectBody::SdlObjectBlock(item) => Some(item),
      _ => None,
    }
  }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum SdlScalarBody {
  EmptyStatement(EmptyStatement),
  SdlScalarBlock(SdlScalarBlock),
}
impl SdlScalarBody {
  pub fn as_empty_statement(&self) -> Option<&EmptyStatement> {
    match &self {
      SdlScalarBody::EmptyStatement(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_sdl_scalar_block(&self) -> Option<&SdlScalarBlock> {
    match &self {
      SdlScalarBody::SdlScalarBlock(item) => Some(item),
      _ => None,
    }
  }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum SdlScalarExtending {
  SdlScalarExtendingEnum(SdlScalarExtendingEnum),
  SdlScalarExtendingType(SdlScalarExtendingType),
}
impl SdlScalarExtending {
  pub fn as_sdl_scalar_extending_enum(&self) -> Option<&SdlScalarExtendingEnum> {
    match &self {
      SdlScalarExtending::SdlScalarExtendingEnum(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_sdl_scalar_extending_type(&self) -> Option<&SdlScalarExtendingType> {
    match &self {
      SdlScalarExtending::SdlScalarExtendingType(item) => Some(item),
      _ => None,
    }
  }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum SdlSchema {
  EmptyStatement(EmptyStatement),
  SdlAnnotationSchema(SdlAnnotationSchema),
  SdlObjectSchema(SdlObjectSchema),
}
impl SdlSchema {
  pub fn as_empty_statement(&self) -> Option<&EmptyStatement> {
    match &self {
      SdlSchema::EmptyStatement(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_sdl_annotation_schema(&self) -> Option<&SdlAnnotationSchema> {
    match &self {
      SdlSchema::SdlAnnotationSchema(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_sdl_object_schema(&self) -> Option<&SdlObjectSchema> {
    match &self {
      SdlSchema::SdlObjectSchema(item) => Some(item),
      _ => None,
    }
  }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum SdlSchemaConstraintBody {
  EmptyStatement(EmptyStatement),
  SdlSchemaConstraintBlock(SdlSchemaConstraintBlock),
}
impl SdlSchemaConstraintBody {
  pub fn as_empty_statement(&self) -> Option<&EmptyStatement> {
    match &self {
      SdlSchemaConstraintBody::EmptyStatement(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_sdl_schema_constraint_block(&self) -> Option<&SdlSchemaConstraintBlock> {
    match &self {
      SdlSchemaConstraintBody::SdlSchemaConstraintBlock(item) => Some(item),
      _ => None,
    }
  }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum StringLiteralExpression {
  BareStringLiteralExpression(BareStringLiteralExpression),
  RawStringLiteralExpression(RawStringLiteralExpression),
}
impl StringLiteralExpression {
  pub fn as_bare_string_literal_expression(&self) -> Option<&BareStringLiteralExpression> {
    match &self {
      StringLiteralExpression::BareStringLiteralExpression(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_raw_string_literal_expression(&self) -> Option<&RawStringLiteralExpression> {
    match &self {
      StringLiteralExpression::RawStringLiteralExpression(item) => Some(item),
      _ => None,
    }
  }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum TypeCastTarget {
  AnyLiteralExpression(AnyLiteralExpression),
  ParameterName(ParameterName),
}
impl TypeCastTarget {
  pub fn as_any_literal_expression(&self) -> Option<&AnyLiteralExpression> {
    match &self {
      TypeCastTarget::AnyLiteralExpression(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_parameter_name(&self) -> Option<&ParameterName> {
    match &self {
      TypeCastTarget::ParameterName(item) => Some(item),
      _ => None,
    }
  }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum TypeExpression {
  ArrayType(ArrayType),
  Name(Name),
  PrimitiveType(PrimitiveType),
  RangeType(RangeType),
  TupleType(TupleType),
}
impl TypeExpression {
  pub fn as_array_type(&self) -> Option<&ArrayType> {
    match &self {
      TypeExpression::ArrayType(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_name(&self) -> Option<&Name> {
    match &self {
      TypeExpression::Name(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_primitive_type(&self) -> Option<&PrimitiveType> {
    match &self {
      TypeExpression::PrimitiveType(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_range_type(&self) -> Option<&RangeType> {
    match &self {
      TypeExpression::RangeType(item) => Some(item),
      _ => None,
    }
  }

  pub fn as_tuple_type(&self) -> Option<&TupleType> {
    match &self {
      TypeExpression::TupleType(item) => Some(item),
      _ => None,
    }
  }
}
impl AstNode for ArrayType {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(ARRAY_TYPE as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == ARRAY_TYPE
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for ArrayType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("ArrayType")
      .field(
        "array_token",
        &support::DebugSyntaxResult(self.array_token()),
      )
      .field("less_token", &support::DebugSyntaxResult(self.less_token()))
      .field("member", &support::DebugSyntaxResult(self.member()))
      .field(
        "greater_token",
        &support::DebugSyntaxResult(self.greater_token()),
      )
      .finish()
  }
}
impl From<ArrayType> for SyntaxNode {
  fn from(n: ArrayType) -> SyntaxNode {
    n.syntax
  }
}
impl From<ArrayType> for SyntaxElement {
  fn from(n: ArrayType) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for BareBytesLiteralExpression {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(BARE_BYTES_LITERAL_EXPRESSION as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == BARE_BYTES_LITERAL_EXPRESSION
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for BareBytesLiteralExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("BareBytesLiteralExpression")
      .field("r_token", &support::DebugOptionalElement(self.r_token()))
      .field("b_token", &support::DebugSyntaxResult(self.b_token()))
      .field(
        "bare_string_literal_expression",
        &support::DebugSyntaxResult(self.bare_string_literal_expression()),
      )
      .finish()
  }
}
impl From<BareBytesLiteralExpression> for SyntaxNode {
  fn from(n: BareBytesLiteralExpression) -> SyntaxNode {
    n.syntax
  }
}
impl From<BareBytesLiteralExpression> for SyntaxElement {
  fn from(n: BareBytesLiteralExpression) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for BareStringLiteralExpression {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(BARE_STRING_LITERAL_EXPRESSION as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == BARE_STRING_LITERAL_EXPRESSION
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for BareStringLiteralExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("BareStringLiteralExpression")
      .field(
        "value_token",
        &support::DebugSyntaxResult(self.value_token()),
      )
      .finish()
  }
}
impl From<BareStringLiteralExpression> for SyntaxNode {
  fn from(n: BareStringLiteralExpression) -> SyntaxNode {
    n.syntax
  }
}
impl From<BareStringLiteralExpression> for SyntaxElement {
  fn from(n: BareStringLiteralExpression) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for BigIntLiteralExpression {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(BIG_INT_LITERAL_EXPRESSION as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == BIG_INT_LITERAL_EXPRESSION
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for BigIntLiteralExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("BigIntLiteralExpression")
      .field(
        "value_token",
        &support::DebugSyntaxResult(self.value_token()),
      )
      .finish()
  }
}
impl From<BigIntLiteralExpression> for SyntaxNode {
  fn from(n: BigIntLiteralExpression) -> SyntaxNode {
    n.syntax
  }
}
impl From<BigIntLiteralExpression> for SyntaxElement {
  fn from(n: BigIntLiteralExpression) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for BigIntType {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(BIG_INT_TYPE as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == BIG_INT_TYPE
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for BigIntType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("BigIntType")
      .field(
        "bigint_token",
        &support::DebugSyntaxResult(self.bigint_token()),
      )
      .finish()
  }
}
impl From<BigIntType> for SyntaxNode {
  fn from(n: BigIntType) -> SyntaxNode {
    n.syntax
  }
}
impl From<BigIntType> for SyntaxElement {
  fn from(n: BigIntType) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for BooleanLiteralExpression {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(BOOLEAN_LITERAL_EXPRESSION as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == BOOLEAN_LITERAL_EXPRESSION
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for BooleanLiteralExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("BooleanLiteralExpression")
      .field(
        "value_token",
        &support::DebugSyntaxResult(self.value_token()),
      )
      .finish()
  }
}
impl From<BooleanLiteralExpression> for SyntaxNode {
  fn from(n: BooleanLiteralExpression) -> SyntaxNode {
    n.syntax
  }
}
impl From<BooleanLiteralExpression> for SyntaxElement {
  fn from(n: BooleanLiteralExpression) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for BooleanType {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(BOOLEAN_TYPE as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == BOOLEAN_TYPE
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for BooleanType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("BooleanType")
      .field("bool_token", &support::DebugSyntaxResult(self.bool_token()))
      .finish()
  }
}
impl From<BooleanType> for SyntaxNode {
  fn from(n: BooleanType) -> SyntaxNode {
    n.syntax
  }
}
impl From<BooleanType> for SyntaxElement {
  fn from(n: BooleanType) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for BytesType {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(BYTES_TYPE as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == BYTES_TYPE
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for BytesType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("BytesType")
      .field(
        "bytes_token",
        &support::DebugSyntaxResult(self.bytes_token()),
      )
      .finish()
  }
}
impl From<BytesType> for SyntaxNode {
  fn from(n: BytesType) -> SyntaxNode {
    n.syntax
  }
}
impl From<BytesType> for SyntaxElement {
  fn from(n: BytesType) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for DateTimeType {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(DATE_TIME_TYPE as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == DATE_TIME_TYPE
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for DateTimeType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("DateTimeType")
      .field(
        "datetime_token",
        &support::DebugSyntaxResult(self.datetime_token()),
      )
      .finish()
  }
}
impl From<DateTimeType> for SyntaxNode {
  fn from(n: DateTimeType) -> SyntaxNode {
    n.syntax
  }
}
impl From<DateTimeType> for SyntaxElement {
  fn from(n: DateTimeType) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for DecimalLiteralExpression {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(DECIMAL_LITERAL_EXPRESSION as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == DECIMAL_LITERAL_EXPRESSION
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for DecimalLiteralExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("DecimalLiteralExpression")
      .field(
        "value_token",
        &support::DebugSyntaxResult(self.value_token()),
      )
      .finish()
  }
}
impl From<DecimalLiteralExpression> for SyntaxNode {
  fn from(n: DecimalLiteralExpression) -> SyntaxNode {
    n.syntax
  }
}
impl From<DecimalLiteralExpression> for SyntaxElement {
  fn from(n: DecimalLiteralExpression) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for DecimalType {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(DECIMAL_TYPE as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == DECIMAL_TYPE
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for DecimalType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("DecimalType")
      .field(
        "decimal_token",
        &support::DebugSyntaxResult(self.decimal_token()),
      )
      .finish()
  }
}
impl From<DecimalType> for SyntaxNode {
  fn from(n: DecimalType) -> SyntaxNode {
    n.syntax
  }
}
impl From<DecimalType> for SyntaxElement {
  fn from(n: DecimalType) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for DotReferenceName {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(DOT_REFERENCE_NAME as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == DOT_REFERENCE_NAME
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for DotReferenceName {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("DotReferenceName")
      .field("name_token", &support::DebugSyntaxResult(self.name_token()))
      .field("dot_token", &support::DebugSyntaxResult(self.dot_token()))
      .field("path_token", &support::DebugSyntaxResult(self.path_token()))
      .finish()
  }
}
impl From<DotReferenceName> for SyntaxNode {
  fn from(n: DotReferenceName) -> SyntaxNode {
    n.syntax
  }
}
impl From<DotReferenceName> for SyntaxElement {
  fn from(n: DotReferenceName) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for DurationType {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(DURATION_TYPE as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == DURATION_TYPE
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for DurationType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("DurationType")
      .field(
        "duration_token",
        &support::DebugSyntaxResult(self.duration_token()),
      )
      .finish()
  }
}
impl From<DurationType> for SyntaxNode {
  fn from(n: DurationType) -> SyntaxNode {
    n.syntax
  }
}
impl From<DurationType> for SyntaxElement {
  fn from(n: DurationType) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for EmptyStatement {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(EMPTY_STATEMENT as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == EMPTY_STATEMENT
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for EmptyStatement {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("EmptyStatement")
      .field(
        "semicolon_token",
        &support::DebugSyntaxResult(self.semicolon_token()),
      )
      .finish()
  }
}
impl From<EmptyStatement> for SyntaxNode {
  fn from(n: EmptyStatement) -> SyntaxNode {
    n.syntax
  }
}
impl From<EmptyStatement> for SyntaxElement {
  fn from(n: EmptyStatement) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for Expression {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(EXPRESSION as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == EXPRESSION
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for Expression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Expression")
      .field("unknown", &support::DebugSyntaxResult(self.unknown()))
      .finish()
  }
}
impl From<Expression> for SyntaxNode {
  fn from(n: Expression) -> SyntaxNode {
    n.syntax
  }
}
impl From<Expression> for SyntaxElement {
  fn from(n: Expression) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for FloatLiteralExpression {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(FLOAT_LITERAL_EXPRESSION as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == FLOAT_LITERAL_EXPRESSION
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for FloatLiteralExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("FloatLiteralExpression")
      .field(
        "value_token",
        &support::DebugSyntaxResult(self.value_token()),
      )
      .finish()
  }
}
impl From<FloatLiteralExpression> for SyntaxNode {
  fn from(n: FloatLiteralExpression) -> SyntaxNode {
    n.syntax
  }
}
impl From<FloatLiteralExpression> for SyntaxElement {
  fn from(n: FloatLiteralExpression) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for FloatSixtyFourType {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(FLOAT_SIXTY_FOUR_TYPE as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == FLOAT_SIXTY_FOUR_TYPE
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for FloatSixtyFourType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("FloatSixtyFourType")
      .field(
        "float64_token",
        &support::DebugSyntaxResult(self.float64_token()),
      )
      .finish()
  }
}
impl From<FloatSixtyFourType> for SyntaxNode {
  fn from(n: FloatSixtyFourType) -> SyntaxNode {
    n.syntax
  }
}
impl From<FloatSixtyFourType> for SyntaxElement {
  fn from(n: FloatSixtyFourType) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for FloatThirtyTwoType {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(FLOAT_THIRTY_TWO_TYPE as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == FLOAT_THIRTY_TWO_TYPE
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for FloatThirtyTwoType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("FloatThirtyTwoType")
      .field(
        "float32_token",
        &support::DebugSyntaxResult(self.float32_token()),
      )
      .finish()
  }
}
impl From<FloatThirtyTwoType> for SyntaxNode {
  fn from(n: FloatThirtyTwoType) -> SyntaxNode {
    n.syntax
  }
}
impl From<FloatThirtyTwoType> for SyntaxElement {
  fn from(n: FloatThirtyTwoType) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for IntLiteralExpression {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(INT_LITERAL_EXPRESSION as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == INT_LITERAL_EXPRESSION
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for IntLiteralExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("IntLiteralExpression")
      .field(
        "value_token",
        &support::DebugSyntaxResult(self.value_token()),
      )
      .finish()
  }
}
impl From<IntLiteralExpression> for SyntaxNode {
  fn from(n: IntLiteralExpression) -> SyntaxNode {
    n.syntax
  }
}
impl From<IntLiteralExpression> for SyntaxElement {
  fn from(n: IntLiteralExpression) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for IntSixteenType {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(INT_SIXTEEN_TYPE as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == INT_SIXTEEN_TYPE
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for IntSixteenType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("IntSixteenType")
      .field(
        "int16_token",
        &support::DebugSyntaxResult(self.int16_token()),
      )
      .finish()
  }
}
impl From<IntSixteenType> for SyntaxNode {
  fn from(n: IntSixteenType) -> SyntaxNode {
    n.syntax
  }
}
impl From<IntSixteenType> for SyntaxElement {
  fn from(n: IntSixteenType) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for IntSixtyFourType {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(INT_SIXTY_FOUR_TYPE as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == INT_SIXTY_FOUR_TYPE
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for IntSixtyFourType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("IntSixtyFourType")
      .field(
        "int64_token",
        &support::DebugSyntaxResult(self.int64_token()),
      )
      .finish()
  }
}
impl From<IntSixtyFourType> for SyntaxNode {
  fn from(n: IntSixtyFourType) -> SyntaxNode {
    n.syntax
  }
}
impl From<IntSixtyFourType> for SyntaxElement {
  fn from(n: IntSixtyFourType) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for IntThirtyTwoType {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(INT_THIRTY_TWO_TYPE as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == INT_THIRTY_TWO_TYPE
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for IntThirtyTwoType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("IntThirtyTwoType")
      .field(
        "int32_token",
        &support::DebugSyntaxResult(self.int32_token()),
      )
      .finish()
  }
}
impl From<IntThirtyTwoType> for SyntaxNode {
  fn from(n: IntThirtyTwoType) -> SyntaxNode {
    n.syntax
  }
}
impl From<IntThirtyTwoType> for SyntaxElement {
  fn from(n: IntThirtyTwoType) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for JsonType {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(JSON_TYPE as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == JSON_TYPE
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for JsonType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("JsonType")
      .field("json_token", &support::DebugSyntaxResult(self.json_token()))
      .finish()
  }
}
impl From<JsonType> for SyntaxNode {
  fn from(n: JsonType) -> SyntaxNode {
    n.syntax
  }
}
impl From<JsonType> for SyntaxElement {
  fn from(n: JsonType) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for ParameterName {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(PARAMETER_NAME as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == PARAMETER_NAME
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for ParameterName {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("ParameterName")
      .field(
        "dollar_token",
        &support::DebugSyntaxResult(self.dollar_token()),
      )
      .field(
        "parameter_name_token",
        &support::DebugSyntaxResult(self.parameter_name_token()),
      )
      .finish()
  }
}
impl From<ParameterName> for SyntaxNode {
  fn from(n: ParameterName) -> SyntaxNode {
    n.syntax
  }
}
impl From<ParameterName> for SyntaxElement {
  fn from(n: ParameterName) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for QualifiedName {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(QUALIFIED_NAME as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == QUALIFIED_NAME
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for QualifiedName {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("QualifiedName")
      .field("name_token", &support::DebugSyntaxResult(self.name_token()))
      .field(
        "namespace_token",
        &support::DebugSyntaxResult(self.namespace_token()),
      )
      .field("path_token", &support::DebugSyntaxResult(self.path_token()))
      .finish()
  }
}
impl From<QualifiedName> for SyntaxNode {
  fn from(n: QualifiedName) -> SyntaxNode {
    n.syntax
  }
}
impl From<QualifiedName> for SyntaxElement {
  fn from(n: QualifiedName) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for RangeType {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(RANGE_TYPE as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == RANGE_TYPE
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for RangeType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("RangeType")
      .field(
        "range_token",
        &support::DebugSyntaxResult(self.range_token()),
      )
      .field("less_token", &support::DebugSyntaxResult(self.less_token()))
      .field("member", &support::DebugSyntaxResult(self.member()))
      .field(
        "greater_token",
        &support::DebugSyntaxResult(self.greater_token()),
      )
      .finish()
  }
}
impl From<RangeType> for SyntaxNode {
  fn from(n: RangeType) -> SyntaxNode {
    n.syntax
  }
}
impl From<RangeType> for SyntaxElement {
  fn from(n: RangeType) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for RawBytesLiteralExpression {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(RAW_BYTES_LITERAL_EXPRESSION as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == RAW_BYTES_LITERAL_EXPRESSION
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for RawBytesLiteralExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("RawBytesLiteralExpression")
      .field("b_token", &support::DebugSyntaxResult(self.b_token()))
      .field(
        "raw_string_literal_expression",
        &support::DebugSyntaxResult(self.raw_string_literal_expression()),
      )
      .finish()
  }
}
impl From<RawBytesLiteralExpression> for SyntaxNode {
  fn from(n: RawBytesLiteralExpression) -> SyntaxNode {
    n.syntax
  }
}
impl From<RawBytesLiteralExpression> for SyntaxElement {
  fn from(n: RawBytesLiteralExpression) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for RawStringLiteralExpression {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(RAW_STRING_LITERAL_EXPRESSION as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == RAW_STRING_LITERAL_EXPRESSION
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for RawStringLiteralExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("RawStringLiteralExpression")
      .field("r_token", &support::DebugSyntaxResult(self.r_token()))
      .field(
        "value_token",
        &support::DebugSyntaxResult(self.value_token()),
      )
      .finish()
  }
}
impl From<RawStringLiteralExpression> for SyntaxNode {
  fn from(n: RawStringLiteralExpression) -> SyntaxNode {
    n.syntax
  }
}
impl From<RawStringLiteralExpression> for SyntaxElement {
  fn from(n: RawStringLiteralExpression) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for SdlAnnotation {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_ANNOTATION as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_ANNOTATION
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for SdlAnnotation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SdlAnnotation")
      .field(
        "annotation_token",
        &support::DebugSyntaxResult(self.annotation_token()),
      )
      .field("name", &support::DebugSyntaxResult(self.name()))
      .field(
        "assign_token",
        &support::DebugSyntaxResult(self.assign_token()),
      )
      .field("value", &support::DebugSyntaxResult(self.value()))
      .field(
        "empty_statement",
        &support::DebugSyntaxResult(self.empty_statement()),
      )
      .finish()
  }
}
impl From<SdlAnnotation> for SyntaxNode {
  fn from(n: SdlAnnotation) -> SyntaxNode {
    n.syntax
  }
}
impl From<SdlAnnotation> for SyntaxElement {
  fn from(n: SdlAnnotation) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for SdlAnnotationSchema {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_ANNOTATION_SCHEMA as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_ANNOTATION_SCHEMA
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for SdlAnnotationSchema {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SdlAnnotationSchema")
      .field(
        "abstract_token",
        &support::DebugSyntaxResult(self.abstract_token()),
      )
      .field(
        "inheritable_token",
        &support::DebugOptionalElement(self.inheritable_token()),
      )
      .field(
        "annotation_token",
        &support::DebugSyntaxResult(self.annotation_token()),
      )
      .field("name", &support::DebugSyntaxResult(self.name()))
      .field("body", &support::DebugSyntaxResult(self.body()))
      .finish()
  }
}
impl From<SdlAnnotationSchema> for SyntaxNode {
  fn from(n: SdlAnnotationSchema) -> SyntaxNode {
    n.syntax
  }
}
impl From<SdlAnnotationSchema> for SyntaxElement {
  fn from(n: SdlAnnotationSchema) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for SdlAnnotationSchemaBlock {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_ANNOTATION_SCHEMA_BLOCK as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_ANNOTATION_SCHEMA_BLOCK
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for SdlAnnotationSchemaBlock {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SdlAnnotationSchemaBlock")
      .field(
        "open_curly_token",
        &support::DebugSyntaxResult(self.open_curly_token()),
      )
      .field("annotations", &self.annotations())
      .field(
        "close_curly_token",
        &support::DebugSyntaxResult(self.close_curly_token()),
      )
      .field(
        "semicolon_token",
        &support::DebugOptionalElement(self.semicolon_token()),
      )
      .finish()
  }
}
impl From<SdlAnnotationSchemaBlock> for SyntaxNode {
  fn from(n: SdlAnnotationSchemaBlock) -> SyntaxNode {
    n.syntax
  }
}
impl From<SdlAnnotationSchemaBlock> for SyntaxElement {
  fn from(n: SdlAnnotationSchemaBlock) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for SdlConstraint {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_CONSTRAINT as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_CONSTRAINT
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for SdlConstraint {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SdlConstraint")
      .field(
        "delegated_token",
        &support::DebugSyntaxResult(self.delegated_token()),
      )
      .field(
        "constraint_token",
        &support::DebugSyntaxResult(self.constraint_token()),
      )
      .field("name", &support::DebugSyntaxResult(self.name()))
      .field("args", &support::DebugOptionalElement(self.args()))
      .field("subject", &support::DebugOptionalElement(self.subject()))
      .field("except", &support::DebugOptionalElement(self.except()))
      .field("body", &support::DebugSyntaxResult(self.body()))
      .finish()
  }
}
impl From<SdlConstraint> for SyntaxNode {
  fn from(n: SdlConstraint) -> SyntaxNode {
    n.syntax
  }
}
impl From<SdlConstraint> for SyntaxElement {
  fn from(n: SdlConstraint) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for SdlConstraintArg {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_CONSTRAINT_ARG as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_CONSTRAINT_ARG
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for SdlConstraintArg {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SdlConstraintArg")
      .field("name", &support::DebugSyntaxResult(self.name()))
      .field("name_token", &support::DebugSyntaxResult(self.name_token()))
      .field("value", &support::DebugSyntaxResult(self.value()))
      .finish()
  }
}
impl From<SdlConstraintArg> for SyntaxNode {
  fn from(n: SdlConstraintArg) -> SyntaxNode {
    n.syntax
  }
}
impl From<SdlConstraintArg> for SyntaxElement {
  fn from(n: SdlConstraintArg) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for SdlConstraintArgs {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_CONSTRAINT_ARGS as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_CONSTRAINT_ARGS
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for SdlConstraintArgs {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SdlConstraintArgs")
      .field(
        "open_paren_token",
        &support::DebugSyntaxResult(self.open_paren_token()),
      )
      .field("args", &self.args())
      .field(
        "close_paren_token",
        &support::DebugSyntaxResult(self.close_paren_token()),
      )
      .finish()
  }
}
impl From<SdlConstraintArgs> for SyntaxNode {
  fn from(n: SdlConstraintArgs) -> SyntaxNode {
    n.syntax
  }
}
impl From<SdlConstraintArgs> for SyntaxElement {
  fn from(n: SdlConstraintArgs) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for SdlConstraintBlock {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_CONSTRAINT_BLOCK as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_CONSTRAINT_BLOCK
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for SdlConstraintBlock {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SdlConstraintBlock")
      .field(
        "open_curly_token",
        &support::DebugSyntaxResult(self.open_curly_token()),
      )
      .field("annotations", &self.annotations())
      .field(
        "error_message",
        &support::DebugOptionalElement(self.error_message()),
      )
      .field(
        "close_curly_token",
        &support::DebugSyntaxResult(self.close_curly_token()),
      )
      .field(
        "semicolon_token",
        &support::DebugOptionalElement(self.semicolon_token()),
      )
      .finish()
  }
}
impl From<SdlConstraintBlock> for SyntaxNode {
  fn from(n: SdlConstraintBlock) -> SyntaxNode {
    n.syntax
  }
}
impl From<SdlConstraintBlock> for SyntaxElement {
  fn from(n: SdlConstraintBlock) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for SdlConstraintErrorMessage {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_CONSTRAINT_ERROR_MESSAGE as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_CONSTRAINT_ERROR_MESSAGE
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for SdlConstraintErrorMessage {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SdlConstraintErrorMessage")
      .field(
        "errmessage_token",
        &support::DebugSyntaxResult(self.errmessage_token()),
      )
      .field(
        "assign_token",
        &support::DebugSyntaxResult(self.assign_token()),
      )
      .field("message", &support::DebugSyntaxResult(self.message()))
      .field(
        "empty_statement",
        &support::DebugSyntaxResult(self.empty_statement()),
      )
      .finish()
  }
}
impl From<SdlConstraintErrorMessage> for SyntaxNode {
  fn from(n: SdlConstraintErrorMessage) -> SyntaxNode {
    n.syntax
  }
}
impl From<SdlConstraintErrorMessage> for SyntaxElement {
  fn from(n: SdlConstraintErrorMessage) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for SdlConstraintExcept {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_CONSTRAINT_EXCEPT as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_CONSTRAINT_EXCEPT
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for SdlConstraintExcept {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SdlConstraintExcept")
      .field(
        "except_token",
        &support::DebugSyntaxResult(self.except_token()),
      )
      .field("expression", &support::DebugSyntaxResult(self.expression()))
      .finish()
  }
}
impl From<SdlConstraintExcept> for SyntaxNode {
  fn from(n: SdlConstraintExcept) -> SyntaxNode {
    n.syntax
  }
}
impl From<SdlConstraintExcept> for SyntaxElement {
  fn from(n: SdlConstraintExcept) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for SdlConstraintSubjectExpression {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_CONSTRAINT_SUBJECT_EXPRESSION as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_CONSTRAINT_SUBJECT_EXPRESSION
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for SdlConstraintSubjectExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SdlConstraintSubjectExpression")
      .field("on_token", &support::DebugSyntaxResult(self.on_token()))
      .field(
        "open_paren_token",
        &support::DebugSyntaxResult(self.open_paren_token()),
      )
      .field("expression", &support::DebugSyntaxResult(self.expression()))
      .field(
        "close_paren_token",
        &support::DebugSyntaxResult(self.close_paren_token()),
      )
      .finish()
  }
}
impl From<SdlConstraintSubjectExpression> for SyntaxNode {
  fn from(n: SdlConstraintSubjectExpression) -> SyntaxNode {
    n.syntax
  }
}
impl From<SdlConstraintSubjectExpression> for SyntaxElement {
  fn from(n: SdlConstraintSubjectExpression) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for SdlEnumDeclaration {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_ENUM_DECLARATION as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_ENUM_DECLARATION
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for SdlEnumDeclaration {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SdlEnumDeclaration")
      .field("enum_token", &support::DebugSyntaxResult(self.enum_token()))
      .field("less_token", &support::DebugSyntaxResult(self.less_token()))
      .field("members", &self.members())
      .field(
        "greater_token",
        &support::DebugSyntaxResult(self.greater_token()),
      )
      .finish()
  }
}
impl From<SdlEnumDeclaration> for SyntaxNode {
  fn from(n: SdlEnumDeclaration) -> SyntaxNode {
    n.syntax
  }
}
impl From<SdlEnumDeclaration> for SyntaxElement {
  fn from(n: SdlEnumDeclaration) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for SdlExtending {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_EXTENDING as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_EXTENDING
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for SdlExtending {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SdlExtending")
      .field(
        "extending_token",
        &support::DebugSyntaxResult(self.extending_token()),
      )
      .field("extends", &self.extends())
      .finish()
  }
}
impl From<SdlExtending> for SyntaxNode {
  fn from(n: SdlExtending) -> SyntaxNode {
    n.syntax
  }
}
impl From<SdlExtending> for SyntaxElement {
  fn from(n: SdlExtending) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for SdlIndex {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_INDEX as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_INDEX
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for SdlIndex {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SdlIndex")
      .field(
        "sdl_unknown",
        &support::DebugSyntaxResult(self.sdl_unknown()),
      )
      .finish()
  }
}
impl From<SdlIndex> for SyntaxNode {
  fn from(n: SdlIndex) -> SyntaxNode {
    n.syntax
  }
}
impl From<SdlIndex> for SyntaxElement {
  fn from(n: SdlIndex) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for SdlLink {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> = SyntaxKindSet::from_raw(RawSyntaxKind(SDL_LINK as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_LINK
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for SdlLink {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SdlLink")
      .field(
        "sdl_unknown",
        &support::DebugSyntaxResult(self.sdl_unknown()),
      )
      .finish()
  }
}
impl From<SdlLink> for SyntaxNode {
  fn from(n: SdlLink) -> SyntaxNode {
    n.syntax
  }
}
impl From<SdlLink> for SyntaxElement {
  fn from(n: SdlLink) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for SdlModule {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_MODULE as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_MODULE
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for SdlModule {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SdlModule")
      .field(
        "module_token",
        &support::DebugSyntaxResult(self.module_token()),
      )
      .field(
        "unqualified_name",
        &support::DebugSyntaxResult(self.unqualified_name()),
      )
      .field(
        "open_curly_token",
        &support::DebugSyntaxResult(self.open_curly_token()),
      )
      .field("statements", &self.statements())
      .field(
        "close_curly_token",
        &support::DebugSyntaxResult(self.close_curly_token()),
      )
      .field(
        "semicolon_token",
        &support::DebugOptionalElement(self.semicolon_token()),
      )
      .finish()
  }
}
impl From<SdlModule> for SyntaxNode {
  fn from(n: SdlModule) -> SyntaxNode {
    n.syntax
  }
}
impl From<SdlModule> for SyntaxElement {
  fn from(n: SdlModule) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for SdlObjectBlock {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_OBJECT_BLOCK as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_OBJECT_BLOCK
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for SdlObjectBlock {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SdlObjectBlock")
      .field(
        "open_curly_token",
        &support::DebugSyntaxResult(self.open_curly_token()),
      )
      .field("annotations", &self.annotations())
      .field("properties", &self.properties())
      .field("links", &self.links())
      .field("constraints", &self.constraints())
      .field("indexes", &self.indexes())
      .field(
        "close_curly_token",
        &support::DebugSyntaxResult(self.close_curly_token()),
      )
      .field(
        "semicolon_token",
        &support::DebugOptionalElement(self.semicolon_token()),
      )
      .finish()
  }
}
impl From<SdlObjectBlock> for SyntaxNode {
  fn from(n: SdlObjectBlock) -> SyntaxNode {
    n.syntax
  }
}
impl From<SdlObjectBlock> for SyntaxElement {
  fn from(n: SdlObjectBlock) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for SdlObjectSchema {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_OBJECT_SCHEMA as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_OBJECT_SCHEMA
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for SdlObjectSchema {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SdlObjectSchema")
      .field(
        "abstract_token",
        &support::DebugOptionalElement(self.abstract_token()),
      )
      .field("type_token", &support::DebugSyntaxResult(self.type_token()))
      .field("name", &support::DebugSyntaxResult(self.name()))
      .field(
        "extending",
        &support::DebugOptionalElement(self.extending()),
      )
      .field("body", &support::DebugSyntaxResult(self.body()))
      .finish()
  }
}
impl From<SdlObjectSchema> for SyntaxNode {
  fn from(n: SdlObjectSchema) -> SyntaxNode {
    n.syntax
  }
}
impl From<SdlObjectSchema> for SyntaxElement {
  fn from(n: SdlObjectSchema) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for SdlProperty {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_PROPERTY as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_PROPERTY
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for SdlProperty {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SdlProperty")
      .field(
        "sdl_unknown",
        &support::DebugSyntaxResult(self.sdl_unknown()),
      )
      .finish()
  }
}
impl From<SdlProperty> for SyntaxNode {
  fn from(n: SdlProperty) -> SyntaxNode {
    n.syntax
  }
}
impl From<SdlProperty> for SyntaxElement {
  fn from(n: SdlProperty) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for SdlScalarBlock {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_SCALAR_BLOCK as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_SCALAR_BLOCK
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for SdlScalarBlock {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SdlScalarBlock")
      .field(
        "open_curly_token",
        &support::DebugSyntaxResult(self.open_curly_token()),
      )
      .field("annotations", &self.annotations())
      .field("constraints", &self.constraints())
      .field(
        "close_curly_token",
        &support::DebugSyntaxResult(self.close_curly_token()),
      )
      .field(
        "semicolon_token",
        &support::DebugOptionalElement(self.semicolon_token()),
      )
      .finish()
  }
}
impl From<SdlScalarBlock> for SyntaxNode {
  fn from(n: SdlScalarBlock) -> SyntaxNode {
    n.syntax
  }
}
impl From<SdlScalarBlock> for SyntaxElement {
  fn from(n: SdlScalarBlock) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for SdlScalarExtendingEnum {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_SCALAR_EXTENDING_ENUM as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_SCALAR_EXTENDING_ENUM
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for SdlScalarExtendingEnum {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SdlScalarExtendingEnum")
      .field(
        "extending_token",
        &support::DebugSyntaxResult(self.extending_token()),
      )
      .field("extends", &support::DebugSyntaxResult(self.extends()))
      .finish()
  }
}
impl From<SdlScalarExtendingEnum> for SyntaxNode {
  fn from(n: SdlScalarExtendingEnum) -> SyntaxNode {
    n.syntax
  }
}
impl From<SdlScalarExtendingEnum> for SyntaxElement {
  fn from(n: SdlScalarExtendingEnum) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for SdlScalarExtendingType {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_SCALAR_EXTENDING_TYPE as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_SCALAR_EXTENDING_TYPE
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for SdlScalarExtendingType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SdlScalarExtendingType")
      .field(
        "extending_token",
        &support::DebugSyntaxResult(self.extending_token()),
      )
      .field("extends", &self.extends())
      .finish()
  }
}
impl From<SdlScalarExtendingType> for SyntaxNode {
  fn from(n: SdlScalarExtendingType) -> SyntaxNode {
    n.syntax
  }
}
impl From<SdlScalarExtendingType> for SyntaxElement {
  fn from(n: SdlScalarExtendingType) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for SdlScalarSchema {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_SCALAR_SCHEMA as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_SCALAR_SCHEMA
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for SdlScalarSchema {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SdlScalarSchema")
      .field(
        "abstract_token",
        &support::DebugOptionalElement(self.abstract_token()),
      )
      .field(
        "scalar_token",
        &support::DebugSyntaxResult(self.scalar_token()),
      )
      .field("type_token", &support::DebugSyntaxResult(self.type_token()))
      .field("name", &support::DebugSyntaxResult(self.name()))
      .field(
        "extending",
        &support::DebugOptionalElement(self.extending()),
      )
      .field("body", &support::DebugSyntaxResult(self.body()))
      .finish()
  }
}
impl From<SdlScalarSchema> for SyntaxNode {
  fn from(n: SdlScalarSchema) -> SyntaxNode {
    n.syntax
  }
}
impl From<SdlScalarSchema> for SyntaxElement {
  fn from(n: SdlScalarSchema) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for SdlSchemaConstrainParam {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_SCHEMA_CONSTRAIN_PARAM as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_SCHEMA_CONSTRAIN_PARAM
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for SdlSchemaConstrainParam {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SdlSchemaConstrainParam")
      .field("name", &support::DebugSyntaxResult(self.name()))
      .field("name_token", &support::DebugSyntaxResult(self.name_token()))
      .field("param_type", &support::DebugSyntaxResult(self.param_type()))
      .finish()
  }
}
impl From<SdlSchemaConstrainParam> for SyntaxNode {
  fn from(n: SdlSchemaConstrainParam) -> SyntaxNode {
    n.syntax
  }
}
impl From<SdlSchemaConstrainParam> for SyntaxElement {
  fn from(n: SdlSchemaConstrainParam) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for SdlSchemaConstraint {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_SCHEMA_CONSTRAINT as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_SCHEMA_CONSTRAINT
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for SdlSchemaConstraint {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SdlSchemaConstraint")
      .field(
        "abstract_token",
        &support::DebugSyntaxResult(self.abstract_token()),
      )
      .field(
        "constraint_token",
        &support::DebugSyntaxResult(self.constraint_token()),
      )
      .field("name", &support::DebugSyntaxResult(self.name()))
      .field("params", &support::DebugOptionalElement(self.params()))
      .field("subject", &support::DebugOptionalElement(self.subject()))
      .field(
        "extending",
        &support::DebugOptionalElement(self.extending()),
      )
      .field("body", &support::DebugSyntaxResult(self.body()))
      .finish()
  }
}
impl From<SdlSchemaConstraint> for SyntaxNode {
  fn from(n: SdlSchemaConstraint) -> SyntaxNode {
    n.syntax
  }
}
impl From<SdlSchemaConstraint> for SyntaxElement {
  fn from(n: SdlSchemaConstraint) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for SdlSchemaConstraintArgs {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_SCHEMA_CONSTRAINT_ARGS as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_SCHEMA_CONSTRAINT_ARGS
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for SdlSchemaConstraintArgs {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SdlSchemaConstraintArgs")
      .field(
        "open_paren_token",
        &support::DebugSyntaxResult(self.open_paren_token()),
      )
      .field("params", &self.params())
      .field(
        "close_paren_token",
        &support::DebugSyntaxResult(self.close_paren_token()),
      )
      .finish()
  }
}
impl From<SdlSchemaConstraintArgs> for SyntaxNode {
  fn from(n: SdlSchemaConstraintArgs) -> SyntaxNode {
    n.syntax
  }
}
impl From<SdlSchemaConstraintArgs> for SyntaxElement {
  fn from(n: SdlSchemaConstraintArgs) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for SdlSchemaConstraintBlock {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_SCHEMA_CONSTRAINT_BLOCK as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_SCHEMA_CONSTRAINT_BLOCK
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for SdlSchemaConstraintBlock {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SdlSchemaConstraintBlock")
      .field(
        "open_curly_token",
        &support::DebugSyntaxResult(self.open_curly_token()),
      )
      .field("using", &support::DebugSyntaxResult(self.using()))
      .field("annotations", &self.annotations())
      .field(
        "error_message",
        &support::DebugOptionalElement(self.error_message()),
      )
      .field(
        "close_curly_token",
        &support::DebugSyntaxResult(self.close_curly_token()),
      )
      .field(
        "semicolon_token",
        &support::DebugOptionalElement(self.semicolon_token()),
      )
      .finish()
  }
}
impl From<SdlSchemaConstraintBlock> for SyntaxNode {
  fn from(n: SdlSchemaConstraintBlock) -> SyntaxNode {
    n.syntax
  }
}
impl From<SdlSchemaConstraintBlock> for SyntaxElement {
  fn from(n: SdlSchemaConstraintBlock) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for SequenceType {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SEQUENCE_TYPE as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SEQUENCE_TYPE
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for SequenceType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SequenceType")
      .field(
        "sequence_token",
        &support::DebugSyntaxResult(self.sequence_token()),
      )
      .finish()
  }
}
impl From<SequenceType> for SyntaxNode {
  fn from(n: SequenceType) -> SyntaxNode {
    n.syntax
  }
}
impl From<SequenceType> for SyntaxElement {
  fn from(n: SequenceType) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for StringType {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(STRING_TYPE as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == STRING_TYPE
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for StringType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("StringType")
      .field("str_token", &support::DebugSyntaxResult(self.str_token()))
      .finish()
  }
}
impl From<StringType> for SyntaxNode {
  fn from(n: StringType) -> SyntaxNode {
    n.syntax
  }
}
impl From<StringType> for SyntaxElement {
  fn from(n: StringType) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for TupleType {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(TUPLE_TYPE as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == TUPLE_TYPE
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for TupleType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("TupleType")
      .field(
        "tuple_token",
        &support::DebugSyntaxResult(self.tuple_token()),
      )
      .field("less_token", &support::DebugSyntaxResult(self.less_token()))
      .field("members", &self.members())
      .field(
        "greater_token",
        &support::DebugSyntaxResult(self.greater_token()),
      )
      .finish()
  }
}
impl From<TupleType> for SyntaxNode {
  fn from(n: TupleType) -> SyntaxNode {
    n.syntax
  }
}
impl From<TupleType> for SyntaxElement {
  fn from(n: TupleType) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for TypeCastExpression {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(TYPE_CAST_EXPRESSION as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == TYPE_CAST_EXPRESSION
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for TypeCastExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("TypeCastExpression")
      .field("less_token", &support::DebugSyntaxResult(self.less_token()))
      .field("ty", &support::DebugSyntaxResult(self.ty()))
      .field(
        "greater_token",
        &support::DebugSyntaxResult(self.greater_token()),
      )
      .field("target", &support::DebugSyntaxResult(self.target()))
      .finish()
  }
}
impl From<TypeCastExpression> for SyntaxNode {
  fn from(n: TypeCastExpression) -> SyntaxNode {
    n.syntax
  }
}
impl From<TypeCastExpression> for SyntaxElement {
  fn from(n: TypeCastExpression) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for UnqualifiedName {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(UNQUALIFIED_NAME as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == UNQUALIFIED_NAME
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for UnqualifiedName {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("UnqualifiedName")
      .field("name_token", &support::DebugSyntaxResult(self.name_token()))
      .finish()
  }
}
impl From<UnqualifiedName> for SyntaxNode {
  fn from(n: UnqualifiedName) -> SyntaxNode {
    n.syntax
  }
}
impl From<UnqualifiedName> for SyntaxElement {
  fn from(n: UnqualifiedName) -> SyntaxElement {
    n.syntax.into()
  }
}
impl AstNode for UuidType {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(UUID_TYPE as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == UUID_TYPE
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for UuidType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("UuidType")
      .field("uuid_token", &support::DebugSyntaxResult(self.uuid_token()))
      .finish()
  }
}
impl From<UuidType> for SyntaxNode {
  fn from(n: UuidType) -> SyntaxNode {
    n.syntax
  }
}
impl From<UuidType> for SyntaxElement {
  fn from(n: UuidType) -> SyntaxElement {
    n.syntax.into()
  }
}
impl From<BigIntLiteralExpression> for AnyLiteralExpression {
  fn from(node: BigIntLiteralExpression) -> AnyLiteralExpression {
    AnyLiteralExpression::BigIntLiteralExpression(node)
  }
}
impl From<BooleanLiteralExpression> for AnyLiteralExpression {
  fn from(node: BooleanLiteralExpression) -> AnyLiteralExpression {
    AnyLiteralExpression::BooleanLiteralExpression(node)
  }
}
impl From<DecimalLiteralExpression> for AnyLiteralExpression {
  fn from(node: DecimalLiteralExpression) -> AnyLiteralExpression {
    AnyLiteralExpression::DecimalLiteralExpression(node)
  }
}
impl From<FloatLiteralExpression> for AnyLiteralExpression {
  fn from(node: FloatLiteralExpression) -> AnyLiteralExpression {
    AnyLiteralExpression::FloatLiteralExpression(node)
  }
}
impl From<IntLiteralExpression> for AnyLiteralExpression {
  fn from(node: IntLiteralExpression) -> AnyLiteralExpression {
    AnyLiteralExpression::IntLiteralExpression(node)
  }
}
impl AstNode for AnyLiteralExpression {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> = BigIntLiteralExpression::KIND_SET
    .union(BooleanLiteralExpression::KIND_SET)
    .union(BytesLiteralExpression::KIND_SET)
    .union(DecimalLiteralExpression::KIND_SET)
    .union(FloatLiteralExpression::KIND_SET)
    .union(IntLiteralExpression::KIND_SET)
    .union(StringLiteralExpression::KIND_SET);

  fn can_cast(kind: SyntaxKind) -> bool {
    match kind {
      BIG_INT_LITERAL_EXPRESSION
      | BOOLEAN_LITERAL_EXPRESSION
      | DECIMAL_LITERAL_EXPRESSION
      | FLOAT_LITERAL_EXPRESSION
      | INT_LITERAL_EXPRESSION => true,
      k if BytesLiteralExpression::can_cast(k) => true,
      k if StringLiteralExpression::can_cast(k) => true,
      _ => false,
    }
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    let res = match syntax.kind() {
      BIG_INT_LITERAL_EXPRESSION => {
        AnyLiteralExpression::BigIntLiteralExpression(BigIntLiteralExpression { syntax })
      }
      BOOLEAN_LITERAL_EXPRESSION => {
        AnyLiteralExpression::BooleanLiteralExpression(BooleanLiteralExpression { syntax })
      }
      DECIMAL_LITERAL_EXPRESSION => {
        AnyLiteralExpression::DecimalLiteralExpression(DecimalLiteralExpression { syntax })
      }
      FLOAT_LITERAL_EXPRESSION => {
        AnyLiteralExpression::FloatLiteralExpression(FloatLiteralExpression { syntax })
      }
      INT_LITERAL_EXPRESSION => {
        AnyLiteralExpression::IntLiteralExpression(IntLiteralExpression { syntax })
      }
      _ => {
        if let Some(bytes_literal_expression) = BytesLiteralExpression::cast(syntax.clone()) {
          return Some(AnyLiteralExpression::BytesLiteralExpression(
            bytes_literal_expression,
          ));
        }
        if let Some(string_literal_expression) = StringLiteralExpression::cast(syntax) {
          return Some(AnyLiteralExpression::StringLiteralExpression(
            string_literal_expression,
          ));
        }
        return None;
      }
    };
    Some(res)
  }

  fn syntax(&self) -> &SyntaxNode {
    match self {
      AnyLiteralExpression::BigIntLiteralExpression(it) => &it.syntax,
      AnyLiteralExpression::BooleanLiteralExpression(it) => &it.syntax,
      AnyLiteralExpression::DecimalLiteralExpression(it) => &it.syntax,
      AnyLiteralExpression::FloatLiteralExpression(it) => &it.syntax,
      AnyLiteralExpression::IntLiteralExpression(it) => &it.syntax,
      AnyLiteralExpression::BytesLiteralExpression(it) => it.syntax(),
      AnyLiteralExpression::StringLiteralExpression(it) => it.syntax(),
    }
  }

  fn into_syntax(self) -> SyntaxNode {
    match self {
      AnyLiteralExpression::BigIntLiteralExpression(it) => it.syntax,
      AnyLiteralExpression::BooleanLiteralExpression(it) => it.syntax,
      AnyLiteralExpression::DecimalLiteralExpression(it) => it.syntax,
      AnyLiteralExpression::FloatLiteralExpression(it) => it.syntax,
      AnyLiteralExpression::IntLiteralExpression(it) => it.syntax,
      AnyLiteralExpression::BytesLiteralExpression(it) => it.into_syntax(),
      AnyLiteralExpression::StringLiteralExpression(it) => it.into_syntax(),
    }
  }
}
impl std::fmt::Debug for AnyLiteralExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      AnyLiteralExpression::BigIntLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
      AnyLiteralExpression::BooleanLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
      AnyLiteralExpression::BytesLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
      AnyLiteralExpression::DecimalLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
      AnyLiteralExpression::FloatLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
      AnyLiteralExpression::IntLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
      AnyLiteralExpression::StringLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
    }
  }
}
impl From<AnyLiteralExpression> for SyntaxNode {
  fn from(n: AnyLiteralExpression) -> SyntaxNode {
    match n {
      AnyLiteralExpression::BigIntLiteralExpression(it) => it.into(),
      AnyLiteralExpression::BooleanLiteralExpression(it) => it.into(),
      AnyLiteralExpression::BytesLiteralExpression(it) => it.into(),
      AnyLiteralExpression::DecimalLiteralExpression(it) => it.into(),
      AnyLiteralExpression::FloatLiteralExpression(it) => it.into(),
      AnyLiteralExpression::IntLiteralExpression(it) => it.into(),
      AnyLiteralExpression::StringLiteralExpression(it) => it.into(),
    }
  }
}
impl From<AnyLiteralExpression> for SyntaxElement {
  fn from(n: AnyLiteralExpression) -> SyntaxElement {
    let node: SyntaxNode = n.into();
    node.into()
  }
}
impl From<BareBytesLiteralExpression> for BytesLiteralExpression {
  fn from(node: BareBytesLiteralExpression) -> BytesLiteralExpression {
    BytesLiteralExpression::BareBytesLiteralExpression(node)
  }
}
impl From<RawBytesLiteralExpression> for BytesLiteralExpression {
  fn from(node: RawBytesLiteralExpression) -> BytesLiteralExpression {
    BytesLiteralExpression::RawBytesLiteralExpression(node)
  }
}
impl AstNode for BytesLiteralExpression {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    BareBytesLiteralExpression::KIND_SET.union(RawBytesLiteralExpression::KIND_SET);

  fn can_cast(kind: SyntaxKind) -> bool {
    matches!(
      kind,
      BARE_BYTES_LITERAL_EXPRESSION | RAW_BYTES_LITERAL_EXPRESSION
    )
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    let res = match syntax.kind() {
      BARE_BYTES_LITERAL_EXPRESSION => {
        BytesLiteralExpression::BareBytesLiteralExpression(BareBytesLiteralExpression { syntax })
      }
      RAW_BYTES_LITERAL_EXPRESSION => {
        BytesLiteralExpression::RawBytesLiteralExpression(RawBytesLiteralExpression { syntax })
      }
      _ => return None,
    };
    Some(res)
  }

  fn syntax(&self) -> &SyntaxNode {
    match self {
      BytesLiteralExpression::BareBytesLiteralExpression(it) => &it.syntax,
      BytesLiteralExpression::RawBytesLiteralExpression(it) => &it.syntax,
    }
  }

  fn into_syntax(self) -> SyntaxNode {
    match self {
      BytesLiteralExpression::BareBytesLiteralExpression(it) => it.syntax,
      BytesLiteralExpression::RawBytesLiteralExpression(it) => it.syntax,
    }
  }
}
impl std::fmt::Debug for BytesLiteralExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      BytesLiteralExpression::BareBytesLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
      BytesLiteralExpression::RawBytesLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
    }
  }
}
impl From<BytesLiteralExpression> for SyntaxNode {
  fn from(n: BytesLiteralExpression) -> SyntaxNode {
    match n {
      BytesLiteralExpression::BareBytesLiteralExpression(it) => it.into(),
      BytesLiteralExpression::RawBytesLiteralExpression(it) => it.into(),
    }
  }
}
impl From<BytesLiteralExpression> for SyntaxElement {
  fn from(n: BytesLiteralExpression) -> SyntaxElement {
    let node: SyntaxNode = n.into();
    node.into()
  }
}
impl From<QualifiedName> for Name {
  fn from(node: QualifiedName) -> Name {
    Name::QualifiedName(node)
  }
}
impl From<UnqualifiedName> for Name {
  fn from(node: UnqualifiedName) -> Name {
    Name::UnqualifiedName(node)
  }
}
impl AstNode for Name {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    QualifiedName::KIND_SET.union(UnqualifiedName::KIND_SET);

  fn can_cast(kind: SyntaxKind) -> bool {
    matches!(kind, QUALIFIED_NAME | UNQUALIFIED_NAME)
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    let res = match syntax.kind() {
      QUALIFIED_NAME => Name::QualifiedName(QualifiedName { syntax }),
      UNQUALIFIED_NAME => Name::UnqualifiedName(UnqualifiedName { syntax }),
      _ => return None,
    };
    Some(res)
  }

  fn syntax(&self) -> &SyntaxNode {
    match self {
      Name::QualifiedName(it) => &it.syntax,
      Name::UnqualifiedName(it) => &it.syntax,
    }
  }

  fn into_syntax(self) -> SyntaxNode {
    match self {
      Name::QualifiedName(it) => it.syntax,
      Name::UnqualifiedName(it) => it.syntax,
    }
  }
}
impl std::fmt::Debug for Name {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Name::QualifiedName(it) => std::fmt::Debug::fmt(it, f),
      Name::UnqualifiedName(it) => std::fmt::Debug::fmt(it, f),
    }
  }
}
impl From<Name> for SyntaxNode {
  fn from(n: Name) -> SyntaxNode {
    match n {
      Name::QualifiedName(it) => it.into(),
      Name::UnqualifiedName(it) => it.into(),
    }
  }
}
impl From<Name> for SyntaxElement {
  fn from(n: Name) -> SyntaxElement {
    let node: SyntaxNode = n.into();
    node.into()
  }
}
impl From<BigIntType> for PrimitiveType {
  fn from(node: BigIntType) -> PrimitiveType {
    PrimitiveType::BigIntType(node)
  }
}
impl From<BooleanType> for PrimitiveType {
  fn from(node: BooleanType) -> PrimitiveType {
    PrimitiveType::BooleanType(node)
  }
}
impl From<BytesType> for PrimitiveType {
  fn from(node: BytesType) -> PrimitiveType {
    PrimitiveType::BytesType(node)
  }
}
impl From<DateTimeType> for PrimitiveType {
  fn from(node: DateTimeType) -> PrimitiveType {
    PrimitiveType::DateTimeType(node)
  }
}
impl From<DecimalType> for PrimitiveType {
  fn from(node: DecimalType) -> PrimitiveType {
    PrimitiveType::DecimalType(node)
  }
}
impl From<DurationType> for PrimitiveType {
  fn from(node: DurationType) -> PrimitiveType {
    PrimitiveType::DurationType(node)
  }
}
impl From<FloatSixtyFourType> for PrimitiveType {
  fn from(node: FloatSixtyFourType) -> PrimitiveType {
    PrimitiveType::FloatSixtyFourType(node)
  }
}
impl From<FloatThirtyTwoType> for PrimitiveType {
  fn from(node: FloatThirtyTwoType) -> PrimitiveType {
    PrimitiveType::FloatThirtyTwoType(node)
  }
}
impl From<IntSixteenType> for PrimitiveType {
  fn from(node: IntSixteenType) -> PrimitiveType {
    PrimitiveType::IntSixteenType(node)
  }
}
impl From<IntSixtyFourType> for PrimitiveType {
  fn from(node: IntSixtyFourType) -> PrimitiveType {
    PrimitiveType::IntSixtyFourType(node)
  }
}
impl From<IntThirtyTwoType> for PrimitiveType {
  fn from(node: IntThirtyTwoType) -> PrimitiveType {
    PrimitiveType::IntThirtyTwoType(node)
  }
}
impl From<JsonType> for PrimitiveType {
  fn from(node: JsonType) -> PrimitiveType {
    PrimitiveType::JsonType(node)
  }
}
impl From<SdlEnumDeclaration> for PrimitiveType {
  fn from(node: SdlEnumDeclaration) -> PrimitiveType {
    PrimitiveType::SdlEnumDeclaration(node)
  }
}
impl From<SequenceType> for PrimitiveType {
  fn from(node: SequenceType) -> PrimitiveType {
    PrimitiveType::SequenceType(node)
  }
}
impl From<StringType> for PrimitiveType {
  fn from(node: StringType) -> PrimitiveType {
    PrimitiveType::StringType(node)
  }
}
impl From<UuidType> for PrimitiveType {
  fn from(node: UuidType) -> PrimitiveType {
    PrimitiveType::UuidType(node)
  }
}
impl AstNode for PrimitiveType {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> = BigIntType::KIND_SET
    .union(BooleanType::KIND_SET)
    .union(BytesType::KIND_SET)
    .union(DateTimeType::KIND_SET)
    .union(DecimalType::KIND_SET)
    .union(DurationType::KIND_SET)
    .union(FloatSixtyFourType::KIND_SET)
    .union(FloatThirtyTwoType::KIND_SET)
    .union(IntSixteenType::KIND_SET)
    .union(IntSixtyFourType::KIND_SET)
    .union(IntThirtyTwoType::KIND_SET)
    .union(JsonType::KIND_SET)
    .union(SdlEnumDeclaration::KIND_SET)
    .union(SequenceType::KIND_SET)
    .union(StringType::KIND_SET)
    .union(UuidType::KIND_SET);

  fn can_cast(kind: SyntaxKind) -> bool {
    matches!(
      kind,
      BIG_INT_TYPE
        | BOOLEAN_TYPE
        | BYTES_TYPE
        | DATE_TIME_TYPE
        | DECIMAL_TYPE
        | DURATION_TYPE
        | FLOAT_SIXTY_FOUR_TYPE
        | FLOAT_THIRTY_TWO_TYPE
        | INT_SIXTEEN_TYPE
        | INT_SIXTY_FOUR_TYPE
        | INT_THIRTY_TWO_TYPE
        | JSON_TYPE
        | SDL_ENUM_DECLARATION
        | SEQUENCE_TYPE
        | STRING_TYPE
        | UUID_TYPE
    )
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    let res = match syntax.kind() {
      BIG_INT_TYPE => PrimitiveType::BigIntType(BigIntType { syntax }),
      BOOLEAN_TYPE => PrimitiveType::BooleanType(BooleanType { syntax }),
      BYTES_TYPE => PrimitiveType::BytesType(BytesType { syntax }),
      DATE_TIME_TYPE => PrimitiveType::DateTimeType(DateTimeType { syntax }),
      DECIMAL_TYPE => PrimitiveType::DecimalType(DecimalType { syntax }),
      DURATION_TYPE => PrimitiveType::DurationType(DurationType { syntax }),
      FLOAT_SIXTY_FOUR_TYPE => PrimitiveType::FloatSixtyFourType(FloatSixtyFourType { syntax }),
      FLOAT_THIRTY_TWO_TYPE => PrimitiveType::FloatThirtyTwoType(FloatThirtyTwoType { syntax }),
      INT_SIXTEEN_TYPE => PrimitiveType::IntSixteenType(IntSixteenType { syntax }),
      INT_SIXTY_FOUR_TYPE => PrimitiveType::IntSixtyFourType(IntSixtyFourType { syntax }),
      INT_THIRTY_TWO_TYPE => PrimitiveType::IntThirtyTwoType(IntThirtyTwoType { syntax }),
      JSON_TYPE => PrimitiveType::JsonType(JsonType { syntax }),
      SDL_ENUM_DECLARATION => PrimitiveType::SdlEnumDeclaration(SdlEnumDeclaration { syntax }),
      SEQUENCE_TYPE => PrimitiveType::SequenceType(SequenceType { syntax }),
      STRING_TYPE => PrimitiveType::StringType(StringType { syntax }),
      UUID_TYPE => PrimitiveType::UuidType(UuidType { syntax }),
      _ => return None,
    };
    Some(res)
  }

  fn syntax(&self) -> &SyntaxNode {
    match self {
      PrimitiveType::BigIntType(it) => &it.syntax,
      PrimitiveType::BooleanType(it) => &it.syntax,
      PrimitiveType::BytesType(it) => &it.syntax,
      PrimitiveType::DateTimeType(it) => &it.syntax,
      PrimitiveType::DecimalType(it) => &it.syntax,
      PrimitiveType::DurationType(it) => &it.syntax,
      PrimitiveType::FloatSixtyFourType(it) => &it.syntax,
      PrimitiveType::FloatThirtyTwoType(it) => &it.syntax,
      PrimitiveType::IntSixteenType(it) => &it.syntax,
      PrimitiveType::IntSixtyFourType(it) => &it.syntax,
      PrimitiveType::IntThirtyTwoType(it) => &it.syntax,
      PrimitiveType::JsonType(it) => &it.syntax,
      PrimitiveType::SdlEnumDeclaration(it) => &it.syntax,
      PrimitiveType::SequenceType(it) => &it.syntax,
      PrimitiveType::StringType(it) => &it.syntax,
      PrimitiveType::UuidType(it) => &it.syntax,
    }
  }

  fn into_syntax(self) -> SyntaxNode {
    match self {
      PrimitiveType::BigIntType(it) => it.syntax,
      PrimitiveType::BooleanType(it) => it.syntax,
      PrimitiveType::BytesType(it) => it.syntax,
      PrimitiveType::DateTimeType(it) => it.syntax,
      PrimitiveType::DecimalType(it) => it.syntax,
      PrimitiveType::DurationType(it) => it.syntax,
      PrimitiveType::FloatSixtyFourType(it) => it.syntax,
      PrimitiveType::FloatThirtyTwoType(it) => it.syntax,
      PrimitiveType::IntSixteenType(it) => it.syntax,
      PrimitiveType::IntSixtyFourType(it) => it.syntax,
      PrimitiveType::IntThirtyTwoType(it) => it.syntax,
      PrimitiveType::JsonType(it) => it.syntax,
      PrimitiveType::SdlEnumDeclaration(it) => it.syntax,
      PrimitiveType::SequenceType(it) => it.syntax,
      PrimitiveType::StringType(it) => it.syntax,
      PrimitiveType::UuidType(it) => it.syntax,
    }
  }
}
impl std::fmt::Debug for PrimitiveType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      PrimitiveType::BigIntType(it) => std::fmt::Debug::fmt(it, f),
      PrimitiveType::BooleanType(it) => std::fmt::Debug::fmt(it, f),
      PrimitiveType::BytesType(it) => std::fmt::Debug::fmt(it, f),
      PrimitiveType::DateTimeType(it) => std::fmt::Debug::fmt(it, f),
      PrimitiveType::DecimalType(it) => std::fmt::Debug::fmt(it, f),
      PrimitiveType::DurationType(it) => std::fmt::Debug::fmt(it, f),
      PrimitiveType::FloatSixtyFourType(it) => std::fmt::Debug::fmt(it, f),
      PrimitiveType::FloatThirtyTwoType(it) => std::fmt::Debug::fmt(it, f),
      PrimitiveType::IntSixteenType(it) => std::fmt::Debug::fmt(it, f),
      PrimitiveType::IntSixtyFourType(it) => std::fmt::Debug::fmt(it, f),
      PrimitiveType::IntThirtyTwoType(it) => std::fmt::Debug::fmt(it, f),
      PrimitiveType::JsonType(it) => std::fmt::Debug::fmt(it, f),
      PrimitiveType::SdlEnumDeclaration(it) => std::fmt::Debug::fmt(it, f),
      PrimitiveType::SequenceType(it) => std::fmt::Debug::fmt(it, f),
      PrimitiveType::StringType(it) => std::fmt::Debug::fmt(it, f),
      PrimitiveType::UuidType(it) => std::fmt::Debug::fmt(it, f),
    }
  }
}
impl From<PrimitiveType> for SyntaxNode {
  fn from(n: PrimitiveType) -> SyntaxNode {
    match n {
      PrimitiveType::BigIntType(it) => it.into(),
      PrimitiveType::BooleanType(it) => it.into(),
      PrimitiveType::BytesType(it) => it.into(),
      PrimitiveType::DateTimeType(it) => it.into(),
      PrimitiveType::DecimalType(it) => it.into(),
      PrimitiveType::DurationType(it) => it.into(),
      PrimitiveType::FloatSixtyFourType(it) => it.into(),
      PrimitiveType::FloatThirtyTwoType(it) => it.into(),
      PrimitiveType::IntSixteenType(it) => it.into(),
      PrimitiveType::IntSixtyFourType(it) => it.into(),
      PrimitiveType::IntThirtyTwoType(it) => it.into(),
      PrimitiveType::JsonType(it) => it.into(),
      PrimitiveType::SdlEnumDeclaration(it) => it.into(),
      PrimitiveType::SequenceType(it) => it.into(),
      PrimitiveType::StringType(it) => it.into(),
      PrimitiveType::UuidType(it) => it.into(),
    }
  }
}
impl From<PrimitiveType> for SyntaxElement {
  fn from(n: PrimitiveType) -> SyntaxElement {
    let node: SyntaxNode = n.into();
    node.into()
  }
}
impl From<DecimalType> for RangeTypeMember {
  fn from(node: DecimalType) -> RangeTypeMember {
    RangeTypeMember::DecimalType(node)
  }
}
impl From<FloatSixtyFourType> for RangeTypeMember {
  fn from(node: FloatSixtyFourType) -> RangeTypeMember {
    RangeTypeMember::FloatSixtyFourType(node)
  }
}
impl From<FloatThirtyTwoType> for RangeTypeMember {
  fn from(node: FloatThirtyTwoType) -> RangeTypeMember {
    RangeTypeMember::FloatThirtyTwoType(node)
  }
}
impl From<IntSixtyFourType> for RangeTypeMember {
  fn from(node: IntSixtyFourType) -> RangeTypeMember {
    RangeTypeMember::IntSixtyFourType(node)
  }
}
impl From<IntThirtyTwoType> for RangeTypeMember {
  fn from(node: IntThirtyTwoType) -> RangeTypeMember {
    RangeTypeMember::IntThirtyTwoType(node)
  }
}
impl From<QualifiedName> for RangeTypeMember {
  fn from(node: QualifiedName) -> RangeTypeMember {
    RangeTypeMember::QualifiedName(node)
  }
}
impl AstNode for RangeTypeMember {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> = DecimalType::KIND_SET
    .union(FloatSixtyFourType::KIND_SET)
    .union(FloatThirtyTwoType::KIND_SET)
    .union(IntSixtyFourType::KIND_SET)
    .union(IntThirtyTwoType::KIND_SET)
    .union(QualifiedName::KIND_SET);

  fn can_cast(kind: SyntaxKind) -> bool {
    matches!(
      kind,
      DECIMAL_TYPE
        | FLOAT_SIXTY_FOUR_TYPE
        | FLOAT_THIRTY_TWO_TYPE
        | INT_SIXTY_FOUR_TYPE
        | INT_THIRTY_TWO_TYPE
        | QUALIFIED_NAME
    )
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    let res = match syntax.kind() {
      DECIMAL_TYPE => RangeTypeMember::DecimalType(DecimalType { syntax }),
      FLOAT_SIXTY_FOUR_TYPE => RangeTypeMember::FloatSixtyFourType(FloatSixtyFourType { syntax }),
      FLOAT_THIRTY_TWO_TYPE => RangeTypeMember::FloatThirtyTwoType(FloatThirtyTwoType { syntax }),
      INT_SIXTY_FOUR_TYPE => RangeTypeMember::IntSixtyFourType(IntSixtyFourType { syntax }),
      INT_THIRTY_TWO_TYPE => RangeTypeMember::IntThirtyTwoType(IntThirtyTwoType { syntax }),
      QUALIFIED_NAME => RangeTypeMember::QualifiedName(QualifiedName { syntax }),
      _ => return None,
    };
    Some(res)
  }

  fn syntax(&self) -> &SyntaxNode {
    match self {
      RangeTypeMember::DecimalType(it) => &it.syntax,
      RangeTypeMember::FloatSixtyFourType(it) => &it.syntax,
      RangeTypeMember::FloatThirtyTwoType(it) => &it.syntax,
      RangeTypeMember::IntSixtyFourType(it) => &it.syntax,
      RangeTypeMember::IntThirtyTwoType(it) => &it.syntax,
      RangeTypeMember::QualifiedName(it) => &it.syntax,
    }
  }

  fn into_syntax(self) -> SyntaxNode {
    match self {
      RangeTypeMember::DecimalType(it) => it.syntax,
      RangeTypeMember::FloatSixtyFourType(it) => it.syntax,
      RangeTypeMember::FloatThirtyTwoType(it) => it.syntax,
      RangeTypeMember::IntSixtyFourType(it) => it.syntax,
      RangeTypeMember::IntThirtyTwoType(it) => it.syntax,
      RangeTypeMember::QualifiedName(it) => it.syntax,
    }
  }
}
impl std::fmt::Debug for RangeTypeMember {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      RangeTypeMember::DecimalType(it) => std::fmt::Debug::fmt(it, f),
      RangeTypeMember::FloatSixtyFourType(it) => std::fmt::Debug::fmt(it, f),
      RangeTypeMember::FloatThirtyTwoType(it) => std::fmt::Debug::fmt(it, f),
      RangeTypeMember::IntSixtyFourType(it) => std::fmt::Debug::fmt(it, f),
      RangeTypeMember::IntThirtyTwoType(it) => std::fmt::Debug::fmt(it, f),
      RangeTypeMember::QualifiedName(it) => std::fmt::Debug::fmt(it, f),
    }
  }
}
impl From<RangeTypeMember> for SyntaxNode {
  fn from(n: RangeTypeMember) -> SyntaxNode {
    match n {
      RangeTypeMember::DecimalType(it) => it.into(),
      RangeTypeMember::FloatSixtyFourType(it) => it.into(),
      RangeTypeMember::FloatThirtyTwoType(it) => it.into(),
      RangeTypeMember::IntSixtyFourType(it) => it.into(),
      RangeTypeMember::IntThirtyTwoType(it) => it.into(),
      RangeTypeMember::QualifiedName(it) => it.into(),
    }
  }
}
impl From<RangeTypeMember> for SyntaxElement {
  fn from(n: RangeTypeMember) -> SyntaxElement {
    let node: SyntaxNode = n.into();
    node.into()
  }
}
impl From<EmptyStatement> for SdlAnnotationSchemaBody {
  fn from(node: EmptyStatement) -> SdlAnnotationSchemaBody {
    SdlAnnotationSchemaBody::EmptyStatement(node)
  }
}
impl From<SdlAnnotationSchemaBlock> for SdlAnnotationSchemaBody {
  fn from(node: SdlAnnotationSchemaBlock) -> SdlAnnotationSchemaBody {
    SdlAnnotationSchemaBody::SdlAnnotationSchemaBlock(node)
  }
}
impl AstNode for SdlAnnotationSchemaBody {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    EmptyStatement::KIND_SET.union(SdlAnnotationSchemaBlock::KIND_SET);

  fn can_cast(kind: SyntaxKind) -> bool {
    matches!(kind, EMPTY_STATEMENT | SDL_ANNOTATION_SCHEMA_BLOCK)
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    let res = match syntax.kind() {
      EMPTY_STATEMENT => SdlAnnotationSchemaBody::EmptyStatement(EmptyStatement { syntax }),
      SDL_ANNOTATION_SCHEMA_BLOCK => {
        SdlAnnotationSchemaBody::SdlAnnotationSchemaBlock(SdlAnnotationSchemaBlock { syntax })
      }
      _ => return None,
    };
    Some(res)
  }

  fn syntax(&self) -> &SyntaxNode {
    match self {
      SdlAnnotationSchemaBody::EmptyStatement(it) => &it.syntax,
      SdlAnnotationSchemaBody::SdlAnnotationSchemaBlock(it) => &it.syntax,
    }
  }

  fn into_syntax(self) -> SyntaxNode {
    match self {
      SdlAnnotationSchemaBody::EmptyStatement(it) => it.syntax,
      SdlAnnotationSchemaBody::SdlAnnotationSchemaBlock(it) => it.syntax,
    }
  }
}
impl std::fmt::Debug for SdlAnnotationSchemaBody {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      SdlAnnotationSchemaBody::EmptyStatement(it) => std::fmt::Debug::fmt(it, f),
      SdlAnnotationSchemaBody::SdlAnnotationSchemaBlock(it) => std::fmt::Debug::fmt(it, f),
    }
  }
}
impl From<SdlAnnotationSchemaBody> for SyntaxNode {
  fn from(n: SdlAnnotationSchemaBody) -> SyntaxNode {
    match n {
      SdlAnnotationSchemaBody::EmptyStatement(it) => it.into(),
      SdlAnnotationSchemaBody::SdlAnnotationSchemaBlock(it) => it.into(),
    }
  }
}
impl From<SdlAnnotationSchemaBody> for SyntaxElement {
  fn from(n: SdlAnnotationSchemaBody) -> SyntaxElement {
    let node: SyntaxNode = n.into();
    node.into()
  }
}
impl From<EmptyStatement> for SdlConstraintBody {
  fn from(node: EmptyStatement) -> SdlConstraintBody {
    SdlConstraintBody::EmptyStatement(node)
  }
}
impl From<SdlConstraintBlock> for SdlConstraintBody {
  fn from(node: SdlConstraintBlock) -> SdlConstraintBody {
    SdlConstraintBody::SdlConstraintBlock(node)
  }
}
impl AstNode for SdlConstraintBody {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    EmptyStatement::KIND_SET.union(SdlConstraintBlock::KIND_SET);

  fn can_cast(kind: SyntaxKind) -> bool {
    matches!(kind, EMPTY_STATEMENT | SDL_CONSTRAINT_BLOCK)
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    let res = match syntax.kind() {
      EMPTY_STATEMENT => SdlConstraintBody::EmptyStatement(EmptyStatement { syntax }),
      SDL_CONSTRAINT_BLOCK => SdlConstraintBody::SdlConstraintBlock(SdlConstraintBlock { syntax }),
      _ => return None,
    };
    Some(res)
  }

  fn syntax(&self) -> &SyntaxNode {
    match self {
      SdlConstraintBody::EmptyStatement(it) => &it.syntax,
      SdlConstraintBody::SdlConstraintBlock(it) => &it.syntax,
    }
  }

  fn into_syntax(self) -> SyntaxNode {
    match self {
      SdlConstraintBody::EmptyStatement(it) => it.syntax,
      SdlConstraintBody::SdlConstraintBlock(it) => it.syntax,
    }
  }
}
impl std::fmt::Debug for SdlConstraintBody {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      SdlConstraintBody::EmptyStatement(it) => std::fmt::Debug::fmt(it, f),
      SdlConstraintBody::SdlConstraintBlock(it) => std::fmt::Debug::fmt(it, f),
    }
  }
}
impl From<SdlConstraintBody> for SyntaxNode {
  fn from(n: SdlConstraintBody) -> SyntaxNode {
    match n {
      SdlConstraintBody::EmptyStatement(it) => it.into(),
      SdlConstraintBody::SdlConstraintBlock(it) => it.into(),
    }
  }
}
impl From<SdlConstraintBody> for SyntaxElement {
  fn from(n: SdlConstraintBody) -> SyntaxElement {
    let node: SyntaxNode = n.into();
    node.into()
  }
}
impl From<EmptyStatement> for SdlObjectBody {
  fn from(node: EmptyStatement) -> SdlObjectBody {
    SdlObjectBody::EmptyStatement(node)
  }
}
impl From<SdlObjectBlock> for SdlObjectBody {
  fn from(node: SdlObjectBlock) -> SdlObjectBody {
    SdlObjectBody::SdlObjectBlock(node)
  }
}
impl AstNode for SdlObjectBody {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    EmptyStatement::KIND_SET.union(SdlObjectBlock::KIND_SET);

  fn can_cast(kind: SyntaxKind) -> bool {
    matches!(kind, EMPTY_STATEMENT | SDL_OBJECT_BLOCK)
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    let res = match syntax.kind() {
      EMPTY_STATEMENT => SdlObjectBody::EmptyStatement(EmptyStatement { syntax }),
      SDL_OBJECT_BLOCK => SdlObjectBody::SdlObjectBlock(SdlObjectBlock { syntax }),
      _ => return None,
    };
    Some(res)
  }

  fn syntax(&self) -> &SyntaxNode {
    match self {
      SdlObjectBody::EmptyStatement(it) => &it.syntax,
      SdlObjectBody::SdlObjectBlock(it) => &it.syntax,
    }
  }

  fn into_syntax(self) -> SyntaxNode {
    match self {
      SdlObjectBody::EmptyStatement(it) => it.syntax,
      SdlObjectBody::SdlObjectBlock(it) => it.syntax,
    }
  }
}
impl std::fmt::Debug for SdlObjectBody {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      SdlObjectBody::EmptyStatement(it) => std::fmt::Debug::fmt(it, f),
      SdlObjectBody::SdlObjectBlock(it) => std::fmt::Debug::fmt(it, f),
    }
  }
}
impl From<SdlObjectBody> for SyntaxNode {
  fn from(n: SdlObjectBody) -> SyntaxNode {
    match n {
      SdlObjectBody::EmptyStatement(it) => it.into(),
      SdlObjectBody::SdlObjectBlock(it) => it.into(),
    }
  }
}
impl From<SdlObjectBody> for SyntaxElement {
  fn from(n: SdlObjectBody) -> SyntaxElement {
    let node: SyntaxNode = n.into();
    node.into()
  }
}
impl From<EmptyStatement> for SdlScalarBody {
  fn from(node: EmptyStatement) -> SdlScalarBody {
    SdlScalarBody::EmptyStatement(node)
  }
}
impl From<SdlScalarBlock> for SdlScalarBody {
  fn from(node: SdlScalarBlock) -> SdlScalarBody {
    SdlScalarBody::SdlScalarBlock(node)
  }
}
impl AstNode for SdlScalarBody {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    EmptyStatement::KIND_SET.union(SdlScalarBlock::KIND_SET);

  fn can_cast(kind: SyntaxKind) -> bool {
    matches!(kind, EMPTY_STATEMENT | SDL_SCALAR_BLOCK)
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    let res = match syntax.kind() {
      EMPTY_STATEMENT => SdlScalarBody::EmptyStatement(EmptyStatement { syntax }),
      SDL_SCALAR_BLOCK => SdlScalarBody::SdlScalarBlock(SdlScalarBlock { syntax }),
      _ => return None,
    };
    Some(res)
  }

  fn syntax(&self) -> &SyntaxNode {
    match self {
      SdlScalarBody::EmptyStatement(it) => &it.syntax,
      SdlScalarBody::SdlScalarBlock(it) => &it.syntax,
    }
  }

  fn into_syntax(self) -> SyntaxNode {
    match self {
      SdlScalarBody::EmptyStatement(it) => it.syntax,
      SdlScalarBody::SdlScalarBlock(it) => it.syntax,
    }
  }
}
impl std::fmt::Debug for SdlScalarBody {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      SdlScalarBody::EmptyStatement(it) => std::fmt::Debug::fmt(it, f),
      SdlScalarBody::SdlScalarBlock(it) => std::fmt::Debug::fmt(it, f),
    }
  }
}
impl From<SdlScalarBody> for SyntaxNode {
  fn from(n: SdlScalarBody) -> SyntaxNode {
    match n {
      SdlScalarBody::EmptyStatement(it) => it.into(),
      SdlScalarBody::SdlScalarBlock(it) => it.into(),
    }
  }
}
impl From<SdlScalarBody> for SyntaxElement {
  fn from(n: SdlScalarBody) -> SyntaxElement {
    let node: SyntaxNode = n.into();
    node.into()
  }
}
impl From<SdlScalarExtendingEnum> for SdlScalarExtending {
  fn from(node: SdlScalarExtendingEnum) -> SdlScalarExtending {
    SdlScalarExtending::SdlScalarExtendingEnum(node)
  }
}
impl From<SdlScalarExtendingType> for SdlScalarExtending {
  fn from(node: SdlScalarExtendingType) -> SdlScalarExtending {
    SdlScalarExtending::SdlScalarExtendingType(node)
  }
}
impl AstNode for SdlScalarExtending {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SdlScalarExtendingEnum::KIND_SET.union(SdlScalarExtendingType::KIND_SET);

  fn can_cast(kind: SyntaxKind) -> bool {
    matches!(kind, SDL_SCALAR_EXTENDING_ENUM | SDL_SCALAR_EXTENDING_TYPE)
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    let res = match syntax.kind() {
      SDL_SCALAR_EXTENDING_ENUM => {
        SdlScalarExtending::SdlScalarExtendingEnum(SdlScalarExtendingEnum { syntax })
      }
      SDL_SCALAR_EXTENDING_TYPE => {
        SdlScalarExtending::SdlScalarExtendingType(SdlScalarExtendingType { syntax })
      }
      _ => return None,
    };
    Some(res)
  }

  fn syntax(&self) -> &SyntaxNode {
    match self {
      SdlScalarExtending::SdlScalarExtendingEnum(it) => &it.syntax,
      SdlScalarExtending::SdlScalarExtendingType(it) => &it.syntax,
    }
  }

  fn into_syntax(self) -> SyntaxNode {
    match self {
      SdlScalarExtending::SdlScalarExtendingEnum(it) => it.syntax,
      SdlScalarExtending::SdlScalarExtendingType(it) => it.syntax,
    }
  }
}
impl std::fmt::Debug for SdlScalarExtending {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      SdlScalarExtending::SdlScalarExtendingEnum(it) => std::fmt::Debug::fmt(it, f),
      SdlScalarExtending::SdlScalarExtendingType(it) => std::fmt::Debug::fmt(it, f),
    }
  }
}
impl From<SdlScalarExtending> for SyntaxNode {
  fn from(n: SdlScalarExtending) -> SyntaxNode {
    match n {
      SdlScalarExtending::SdlScalarExtendingEnum(it) => it.into(),
      SdlScalarExtending::SdlScalarExtendingType(it) => it.into(),
    }
  }
}
impl From<SdlScalarExtending> for SyntaxElement {
  fn from(n: SdlScalarExtending) -> SyntaxElement {
    let node: SyntaxNode = n.into();
    node.into()
  }
}
impl From<EmptyStatement> for SdlSchema {
  fn from(node: EmptyStatement) -> SdlSchema {
    SdlSchema::EmptyStatement(node)
  }
}
impl From<SdlAnnotationSchema> for SdlSchema {
  fn from(node: SdlAnnotationSchema) -> SdlSchema {
    SdlSchema::SdlAnnotationSchema(node)
  }
}
impl From<SdlObjectSchema> for SdlSchema {
  fn from(node: SdlObjectSchema) -> SdlSchema {
    SdlSchema::SdlObjectSchema(node)
  }
}
impl AstNode for SdlSchema {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> = EmptyStatement::KIND_SET
    .union(SdlAnnotationSchema::KIND_SET)
    .union(SdlObjectSchema::KIND_SET);

  fn can_cast(kind: SyntaxKind) -> bool {
    matches!(
      kind,
      EMPTY_STATEMENT | SDL_ANNOTATION_SCHEMA | SDL_OBJECT_SCHEMA
    )
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    let res = match syntax.kind() {
      EMPTY_STATEMENT => SdlSchema::EmptyStatement(EmptyStatement { syntax }),
      SDL_ANNOTATION_SCHEMA => SdlSchema::SdlAnnotationSchema(SdlAnnotationSchema { syntax }),
      SDL_OBJECT_SCHEMA => SdlSchema::SdlObjectSchema(SdlObjectSchema { syntax }),
      _ => return None,
    };
    Some(res)
  }

  fn syntax(&self) -> &SyntaxNode {
    match self {
      SdlSchema::EmptyStatement(it) => &it.syntax,
      SdlSchema::SdlAnnotationSchema(it) => &it.syntax,
      SdlSchema::SdlObjectSchema(it) => &it.syntax,
    }
  }

  fn into_syntax(self) -> SyntaxNode {
    match self {
      SdlSchema::EmptyStatement(it) => it.syntax,
      SdlSchema::SdlAnnotationSchema(it) => it.syntax,
      SdlSchema::SdlObjectSchema(it) => it.syntax,
    }
  }
}
impl std::fmt::Debug for SdlSchema {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      SdlSchema::EmptyStatement(it) => std::fmt::Debug::fmt(it, f),
      SdlSchema::SdlAnnotationSchema(it) => std::fmt::Debug::fmt(it, f),
      SdlSchema::SdlObjectSchema(it) => std::fmt::Debug::fmt(it, f),
    }
  }
}
impl From<SdlSchema> for SyntaxNode {
  fn from(n: SdlSchema) -> SyntaxNode {
    match n {
      SdlSchema::EmptyStatement(it) => it.into(),
      SdlSchema::SdlAnnotationSchema(it) => it.into(),
      SdlSchema::SdlObjectSchema(it) => it.into(),
    }
  }
}
impl From<SdlSchema> for SyntaxElement {
  fn from(n: SdlSchema) -> SyntaxElement {
    let node: SyntaxNode = n.into();
    node.into()
  }
}
impl From<EmptyStatement> for SdlSchemaConstraintBody {
  fn from(node: EmptyStatement) -> SdlSchemaConstraintBody {
    SdlSchemaConstraintBody::EmptyStatement(node)
  }
}
impl From<SdlSchemaConstraintBlock> for SdlSchemaConstraintBody {
  fn from(node: SdlSchemaConstraintBlock) -> SdlSchemaConstraintBody {
    SdlSchemaConstraintBody::SdlSchemaConstraintBlock(node)
  }
}
impl AstNode for SdlSchemaConstraintBody {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    EmptyStatement::KIND_SET.union(SdlSchemaConstraintBlock::KIND_SET);

  fn can_cast(kind: SyntaxKind) -> bool {
    matches!(kind, EMPTY_STATEMENT | SDL_SCHEMA_CONSTRAINT_BLOCK)
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    let res = match syntax.kind() {
      EMPTY_STATEMENT => SdlSchemaConstraintBody::EmptyStatement(EmptyStatement { syntax }),
      SDL_SCHEMA_CONSTRAINT_BLOCK => {
        SdlSchemaConstraintBody::SdlSchemaConstraintBlock(SdlSchemaConstraintBlock { syntax })
      }
      _ => return None,
    };
    Some(res)
  }

  fn syntax(&self) -> &SyntaxNode {
    match self {
      SdlSchemaConstraintBody::EmptyStatement(it) => &it.syntax,
      SdlSchemaConstraintBody::SdlSchemaConstraintBlock(it) => &it.syntax,
    }
  }

  fn into_syntax(self) -> SyntaxNode {
    match self {
      SdlSchemaConstraintBody::EmptyStatement(it) => it.syntax,
      SdlSchemaConstraintBody::SdlSchemaConstraintBlock(it) => it.syntax,
    }
  }
}
impl std::fmt::Debug for SdlSchemaConstraintBody {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      SdlSchemaConstraintBody::EmptyStatement(it) => std::fmt::Debug::fmt(it, f),
      SdlSchemaConstraintBody::SdlSchemaConstraintBlock(it) => std::fmt::Debug::fmt(it, f),
    }
  }
}
impl From<SdlSchemaConstraintBody> for SyntaxNode {
  fn from(n: SdlSchemaConstraintBody) -> SyntaxNode {
    match n {
      SdlSchemaConstraintBody::EmptyStatement(it) => it.into(),
      SdlSchemaConstraintBody::SdlSchemaConstraintBlock(it) => it.into(),
    }
  }
}
impl From<SdlSchemaConstraintBody> for SyntaxElement {
  fn from(n: SdlSchemaConstraintBody) -> SyntaxElement {
    let node: SyntaxNode = n.into();
    node.into()
  }
}
impl From<BareStringLiteralExpression> for StringLiteralExpression {
  fn from(node: BareStringLiteralExpression) -> StringLiteralExpression {
    StringLiteralExpression::BareStringLiteralExpression(node)
  }
}
impl From<RawStringLiteralExpression> for StringLiteralExpression {
  fn from(node: RawStringLiteralExpression) -> StringLiteralExpression {
    StringLiteralExpression::RawStringLiteralExpression(node)
  }
}
impl AstNode for StringLiteralExpression {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    BareStringLiteralExpression::KIND_SET.union(RawStringLiteralExpression::KIND_SET);

  fn can_cast(kind: SyntaxKind) -> bool {
    matches!(
      kind,
      BARE_STRING_LITERAL_EXPRESSION | RAW_STRING_LITERAL_EXPRESSION
    )
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    let res = match syntax.kind() {
      BARE_STRING_LITERAL_EXPRESSION => {
        StringLiteralExpression::BareStringLiteralExpression(BareStringLiteralExpression { syntax })
      }
      RAW_STRING_LITERAL_EXPRESSION => {
        StringLiteralExpression::RawStringLiteralExpression(RawStringLiteralExpression { syntax })
      }
      _ => return None,
    };
    Some(res)
  }

  fn syntax(&self) -> &SyntaxNode {
    match self {
      StringLiteralExpression::BareStringLiteralExpression(it) => &it.syntax,
      StringLiteralExpression::RawStringLiteralExpression(it) => &it.syntax,
    }
  }

  fn into_syntax(self) -> SyntaxNode {
    match self {
      StringLiteralExpression::BareStringLiteralExpression(it) => it.syntax,
      StringLiteralExpression::RawStringLiteralExpression(it) => it.syntax,
    }
  }
}
impl std::fmt::Debug for StringLiteralExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      StringLiteralExpression::BareStringLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
      StringLiteralExpression::RawStringLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
    }
  }
}
impl From<StringLiteralExpression> for SyntaxNode {
  fn from(n: StringLiteralExpression) -> SyntaxNode {
    match n {
      StringLiteralExpression::BareStringLiteralExpression(it) => it.into(),
      StringLiteralExpression::RawStringLiteralExpression(it) => it.into(),
    }
  }
}
impl From<StringLiteralExpression> for SyntaxElement {
  fn from(n: StringLiteralExpression) -> SyntaxElement {
    let node: SyntaxNode = n.into();
    node.into()
  }
}
impl From<ParameterName> for TypeCastTarget {
  fn from(node: ParameterName) -> TypeCastTarget {
    TypeCastTarget::ParameterName(node)
  }
}
impl AstNode for TypeCastTarget {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    AnyLiteralExpression::KIND_SET.union(ParameterName::KIND_SET);

  fn can_cast(kind: SyntaxKind) -> bool {
    match kind {
      PARAMETER_NAME => true,
      k if AnyLiteralExpression::can_cast(k) => true,
      _ => false,
    }
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    let res = match syntax.kind() {
      PARAMETER_NAME => TypeCastTarget::ParameterName(ParameterName { syntax }),
      _ => {
        if let Some(any_literal_expression) = AnyLiteralExpression::cast(syntax) {
          return Some(TypeCastTarget::AnyLiteralExpression(any_literal_expression));
        }
        return None;
      }
    };
    Some(res)
  }

  fn syntax(&self) -> &SyntaxNode {
    match self {
      TypeCastTarget::ParameterName(it) => &it.syntax,
      TypeCastTarget::AnyLiteralExpression(it) => it.syntax(),
    }
  }

  fn into_syntax(self) -> SyntaxNode {
    match self {
      TypeCastTarget::ParameterName(it) => it.syntax,
      TypeCastTarget::AnyLiteralExpression(it) => it.into_syntax(),
    }
  }
}
impl std::fmt::Debug for TypeCastTarget {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      TypeCastTarget::AnyLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
      TypeCastTarget::ParameterName(it) => std::fmt::Debug::fmt(it, f),
    }
  }
}
impl From<TypeCastTarget> for SyntaxNode {
  fn from(n: TypeCastTarget) -> SyntaxNode {
    match n {
      TypeCastTarget::AnyLiteralExpression(it) => it.into(),
      TypeCastTarget::ParameterName(it) => it.into(),
    }
  }
}
impl From<TypeCastTarget> for SyntaxElement {
  fn from(n: TypeCastTarget) -> SyntaxElement {
    let node: SyntaxNode = n.into();
    node.into()
  }
}
impl From<ArrayType> for TypeExpression {
  fn from(node: ArrayType) -> TypeExpression {
    TypeExpression::ArrayType(node)
  }
}
impl From<RangeType> for TypeExpression {
  fn from(node: RangeType) -> TypeExpression {
    TypeExpression::RangeType(node)
  }
}
impl From<TupleType> for TypeExpression {
  fn from(node: TupleType) -> TypeExpression {
    TypeExpression::TupleType(node)
  }
}
impl AstNode for TypeExpression {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> = ArrayType::KIND_SET
    .union(Name::KIND_SET)
    .union(PrimitiveType::KIND_SET)
    .union(RangeType::KIND_SET)
    .union(TupleType::KIND_SET);

  fn can_cast(kind: SyntaxKind) -> bool {
    match kind {
      ARRAY_TYPE | RANGE_TYPE | TUPLE_TYPE => true,
      k if Name::can_cast(k) => true,
      k if PrimitiveType::can_cast(k) => true,
      _ => false,
    }
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    let res = match syntax.kind() {
      ARRAY_TYPE => TypeExpression::ArrayType(ArrayType { syntax }),
      RANGE_TYPE => TypeExpression::RangeType(RangeType { syntax }),
      TUPLE_TYPE => TypeExpression::TupleType(TupleType { syntax }),
      _ => {
        if let Some(name) = Name::cast(syntax.clone()) {
          return Some(TypeExpression::Name(name));
        }
        if let Some(primitive_type) = PrimitiveType::cast(syntax) {
          return Some(TypeExpression::PrimitiveType(primitive_type));
        }
        return None;
      }
    };
    Some(res)
  }

  fn syntax(&self) -> &SyntaxNode {
    match self {
      TypeExpression::ArrayType(it) => &it.syntax,
      TypeExpression::RangeType(it) => &it.syntax,
      TypeExpression::TupleType(it) => &it.syntax,
      TypeExpression::Name(it) => it.syntax(),
      TypeExpression::PrimitiveType(it) => it.syntax(),
    }
  }

  fn into_syntax(self) -> SyntaxNode {
    match self {
      TypeExpression::ArrayType(it) => it.syntax,
      TypeExpression::RangeType(it) => it.syntax,
      TypeExpression::TupleType(it) => it.syntax,
      TypeExpression::Name(it) => it.into_syntax(),
      TypeExpression::PrimitiveType(it) => it.into_syntax(),
    }
  }
}
impl std::fmt::Debug for TypeExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      TypeExpression::ArrayType(it) => std::fmt::Debug::fmt(it, f),
      TypeExpression::Name(it) => std::fmt::Debug::fmt(it, f),
      TypeExpression::PrimitiveType(it) => std::fmt::Debug::fmt(it, f),
      TypeExpression::RangeType(it) => std::fmt::Debug::fmt(it, f),
      TypeExpression::TupleType(it) => std::fmt::Debug::fmt(it, f),
    }
  }
}
impl From<TypeExpression> for SyntaxNode {
  fn from(n: TypeExpression) -> SyntaxNode {
    match n {
      TypeExpression::ArrayType(it) => it.into(),
      TypeExpression::Name(it) => it.into(),
      TypeExpression::PrimitiveType(it) => it.into(),
      TypeExpression::RangeType(it) => it.into(),
      TypeExpression::TupleType(it) => it.into(),
    }
  }
}
impl From<TypeExpression> for SyntaxElement {
  fn from(n: TypeExpression) -> SyntaxElement {
    let node: SyntaxNode = n.into();
    node.into()
  }
}
impl std::fmt::Display for AnyLiteralExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for BytesLiteralExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for Name {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for PrimitiveType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for RangeTypeMember {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for SdlAnnotationSchemaBody {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for SdlConstraintBody {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for SdlObjectBody {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for SdlScalarBody {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for SdlScalarExtending {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for SdlSchema {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for SdlSchemaConstraintBody {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for StringLiteralExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for TypeCastTarget {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for TypeExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for ArrayType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for BareBytesLiteralExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for BareStringLiteralExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for BigIntLiteralExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for BigIntType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for BooleanLiteralExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for BooleanType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for BytesType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for DateTimeType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for DecimalLiteralExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for DecimalType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for DotReferenceName {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for DurationType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for EmptyStatement {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for Expression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for FloatLiteralExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for FloatSixtyFourType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for FloatThirtyTwoType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for IntLiteralExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for IntSixteenType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for IntSixtyFourType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for IntThirtyTwoType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for JsonType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for ParameterName {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for QualifiedName {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for RangeType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for RawBytesLiteralExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for RawStringLiteralExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for SdlAnnotation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for SdlAnnotationSchema {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for SdlAnnotationSchemaBlock {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for SdlConstraint {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for SdlConstraintArg {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for SdlConstraintArgs {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for SdlConstraintBlock {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for SdlConstraintErrorMessage {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for SdlConstraintExcept {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for SdlConstraintSubjectExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for SdlEnumDeclaration {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for SdlExtending {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for SdlIndex {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for SdlLink {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for SdlModule {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for SdlObjectBlock {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for SdlObjectSchema {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for SdlProperty {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for SdlScalarBlock {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for SdlScalarExtendingEnum {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for SdlScalarExtendingType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for SdlScalarSchema {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for SdlSchemaConstrainParam {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for SdlSchemaConstraint {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for SdlSchemaConstraintArgs {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for SdlSchemaConstraintBlock {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for SequenceType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for StringType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for TupleType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for TypeCastExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for UnqualifiedName {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
impl std::fmt::Display for UuidType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(self.syntax(), f)
  }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SdlUnknown {
  syntax: SyntaxNode,
}
impl SdlUnknown {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn items(&self) -> SyntaxElementChildren {
    support::elements(&self.syntax)
  }
}
impl AstNode for SdlUnknown {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_UNKNOWN as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_UNKNOWN
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for SdlUnknown {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SdlUnknown")
      .field("items", &DebugSyntaxElementChildren(self.items()))
      .finish()
  }
}
impl From<SdlUnknown> for SyntaxNode {
  fn from(n: SdlUnknown) -> SyntaxNode {
    n.syntax
  }
}
impl From<SdlUnknown> for SyntaxElement {
  fn from(n: SdlUnknown) -> SyntaxElement {
    n.syntax.into()
  }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SdlUnknownExpression {
  syntax: SyntaxNode,
}
impl SdlUnknownExpression {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn items(&self) -> SyntaxElementChildren {
    support::elements(&self.syntax)
  }
}
impl AstNode for SdlUnknownExpression {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_UNKNOWN_EXPRESSION as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_UNKNOWN_EXPRESSION
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for SdlUnknownExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SdlUnknownExpression")
      .field("items", &DebugSyntaxElementChildren(self.items()))
      .finish()
  }
}
impl From<SdlUnknownExpression> for SyntaxNode {
  fn from(n: SdlUnknownExpression) -> SyntaxNode {
    n.syntax
  }
}
impl From<SdlUnknownExpression> for SyntaxElement {
  fn from(n: SdlUnknownExpression) -> SyntaxElement {
    n.syntax.into()
  }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SdlUnknownSchema {
  syntax: SyntaxNode,
}
impl SdlUnknownSchema {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn items(&self) -> SyntaxElementChildren {
    support::elements(&self.syntax)
  }
}
impl AstNode for SdlUnknownSchema {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_UNKNOWN_SCHEMA as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_UNKNOWN_SCHEMA
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for SdlUnknownSchema {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SdlUnknownSchema")
      .field("items", &DebugSyntaxElementChildren(self.items()))
      .finish()
  }
}
impl From<SdlUnknownSchema> for SyntaxNode {
  fn from(n: SdlUnknownSchema) -> SyntaxNode {
    n.syntax
  }
}
impl From<SdlUnknownSchema> for SyntaxElement {
  fn from(n: SdlUnknownSchema) -> SyntaxElement {
    n.syntax.into()
  }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Unknown {
  syntax: SyntaxNode,
}
impl Unknown {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self { syntax }
  }

  pub fn items(&self) -> SyntaxElementChildren {
    support::elements(&self.syntax)
  }
}
impl AstNode for Unknown {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> = SyntaxKindSet::from_raw(RawSyntaxKind(UNKNOWN as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == UNKNOWN
  }

  fn cast(syntax: SyntaxNode) -> Option<Self> {
    if Self::can_cast(syntax.kind()) {
      Some(Self { syntax })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    &self.syntax
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax
  }
}
impl std::fmt::Debug for Unknown {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Unknown")
      .field("items", &DebugSyntaxElementChildren(self.items()))
      .finish()
  }
}
impl From<Unknown> for SyntaxNode {
  fn from(n: Unknown) -> SyntaxNode {
    n.syntax
  }
}
impl From<Unknown> for SyntaxElement {
  fn from(n: Unknown) -> SyntaxElement {
    n.syntax.into()
  }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SdlAnnotationList {
  syntax_list: SyntaxList,
}
impl SdlAnnotationList {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self {
      syntax_list: syntax.into_list(),
    }
  }
}
impl AstNode for SdlAnnotationList {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_ANNOTATION_LIST as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_ANNOTATION_LIST
  }

  fn cast(syntax: SyntaxNode) -> Option<SdlAnnotationList> {
    if Self::can_cast(syntax.kind()) {
      Some(SdlAnnotationList {
        syntax_list: syntax.into_list(),
      })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    self.syntax_list.node()
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax_list.into_node()
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlAnnotationList {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut seq = serializer.serialize_seq(Some(self.len()))?;
    for e in self.iter() {
      seq.serialize_element(&e)?;
    }
    seq.end()
  }
}
impl AstNodeList for SdlAnnotationList {
  type Language = Language;
  type Node = SdlAnnotation;

  fn syntax_list(&self) -> &SyntaxList {
    &self.syntax_list
  }

  fn into_syntax_list(self) -> SyntaxList {
    self.syntax_list
  }
}
impl Debug for SdlAnnotationList {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str("SdlAnnotationList ")?;
    f.debug_list().entries(self.iter()).finish()
  }
}
impl IntoIterator for &SdlAnnotationList {
  type IntoIter = AstNodeListIterator<Language, SdlAnnotation>;
  type Item = SdlAnnotation;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}
impl IntoIterator for SdlAnnotationList {
  type IntoIter = AstNodeListIterator<Language, SdlAnnotation>;
  type Item = SdlAnnotation;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SdlConstraintArgList {
  syntax_list: SyntaxList,
}
impl SdlConstraintArgList {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self {
      syntax_list: syntax.into_list(),
    }
  }
}
impl AstNode for SdlConstraintArgList {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_CONSTRAINT_ARG_LIST as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_CONSTRAINT_ARG_LIST
  }

  fn cast(syntax: SyntaxNode) -> Option<SdlConstraintArgList> {
    if Self::can_cast(syntax.kind()) {
      Some(SdlConstraintArgList {
        syntax_list: syntax.into_list(),
      })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    self.syntax_list.node()
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax_list.into_node()
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlConstraintArgList {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut seq = serializer.serialize_seq(Some(self.len()))?;
    for e in self.iter() {
      seq.serialize_element(&e)?;
    }
    seq.end()
  }
}
impl AstNodeList for SdlConstraintArgList {
  type Language = Language;
  type Node = SdlConstraintArg;

  fn syntax_list(&self) -> &SyntaxList {
    &self.syntax_list
  }

  fn into_syntax_list(self) -> SyntaxList {
    self.syntax_list
  }
}
impl Debug for SdlConstraintArgList {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str("SdlConstraintArgList ")?;
    f.debug_list().entries(self.iter()).finish()
  }
}
impl IntoIterator for &SdlConstraintArgList {
  type IntoIter = AstNodeListIterator<Language, SdlConstraintArg>;
  type Item = SdlConstraintArg;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}
impl IntoIterator for SdlConstraintArgList {
  type IntoIter = AstNodeListIterator<Language, SdlConstraintArg>;
  type Item = SdlConstraintArg;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SdlConstraintList {
  syntax_list: SyntaxList,
}
impl SdlConstraintList {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self {
      syntax_list: syntax.into_list(),
    }
  }
}
impl AstNode for SdlConstraintList {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_CONSTRAINT_LIST as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_CONSTRAINT_LIST
  }

  fn cast(syntax: SyntaxNode) -> Option<SdlConstraintList> {
    if Self::can_cast(syntax.kind()) {
      Some(SdlConstraintList {
        syntax_list: syntax.into_list(),
      })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    self.syntax_list.node()
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax_list.into_node()
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlConstraintList {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut seq = serializer.serialize_seq(Some(self.len()))?;
    for e in self.iter() {
      seq.serialize_element(&e)?;
    }
    seq.end()
  }
}
impl AstNodeList for SdlConstraintList {
  type Language = Language;
  type Node = SdlConstraint;

  fn syntax_list(&self) -> &SyntaxList {
    &self.syntax_list
  }

  fn into_syntax_list(self) -> SyntaxList {
    self.syntax_list
  }
}
impl Debug for SdlConstraintList {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str("SdlConstraintList ")?;
    f.debug_list().entries(self.iter()).finish()
  }
}
impl IntoIterator for &SdlConstraintList {
  type IntoIter = AstNodeListIterator<Language, SdlConstraint>;
  type Item = SdlConstraint;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}
impl IntoIterator for SdlConstraintList {
  type IntoIter = AstNodeListIterator<Language, SdlConstraint>;
  type Item = SdlConstraint;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SdlEnumDeclarationMembers {
  syntax_list: SyntaxList,
}
impl SdlEnumDeclarationMembers {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self {
      syntax_list: syntax.into_list(),
    }
  }
}
impl AstNode for SdlEnumDeclarationMembers {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_ENUM_DECLARATION_MEMBERS as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_ENUM_DECLARATION_MEMBERS
  }

  fn cast(syntax: SyntaxNode) -> Option<SdlEnumDeclarationMembers> {
    if Self::can_cast(syntax.kind()) {
      Some(SdlEnumDeclarationMembers {
        syntax_list: syntax.into_list(),
      })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    self.syntax_list.node()
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax_list.into_node()
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlEnumDeclarationMembers {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut seq = serializer.serialize_seq(Some(self.len()))?;
    for e in self.iter() {
      seq.serialize_element(&e)?;
    }
    seq.end()
  }
}
impl AstSeparatedList for SdlEnumDeclarationMembers {
  type Language = Language;
  type Node = UnqualifiedName;

  fn syntax_list(&self) -> &SyntaxList {
    &self.syntax_list
  }

  fn into_syntax_list(self) -> SyntaxList {
    self.syntax_list
  }
}
impl Debug for SdlEnumDeclarationMembers {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str("SdlEnumDeclarationMembers ")?;
    f.debug_list().entries(self.elements()).finish()
  }
}
impl IntoIterator for SdlEnumDeclarationMembers {
  type IntoIter = AstSeparatedListNodesIterator<Language, UnqualifiedName>;
  type Item = SyntaxResult<UnqualifiedName>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}
impl IntoIterator for &SdlEnumDeclarationMembers {
  type IntoIter = AstSeparatedListNodesIterator<Language, UnqualifiedName>;
  type Item = SyntaxResult<UnqualifiedName>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SdlExtendingNames {
  syntax_list: SyntaxList,
}
impl SdlExtendingNames {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self {
      syntax_list: syntax.into_list(),
    }
  }
}
impl AstNode for SdlExtendingNames {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_EXTENDING_NAMES as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_EXTENDING_NAMES
  }

  fn cast(syntax: SyntaxNode) -> Option<SdlExtendingNames> {
    if Self::can_cast(syntax.kind()) {
      Some(SdlExtendingNames {
        syntax_list: syntax.into_list(),
      })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    self.syntax_list.node()
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax_list.into_node()
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlExtendingNames {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut seq = serializer.serialize_seq(Some(self.len()))?;
    for e in self.iter() {
      seq.serialize_element(&e)?;
    }
    seq.end()
  }
}
impl AstSeparatedList for SdlExtendingNames {
  type Language = Language;
  type Node = Name;

  fn syntax_list(&self) -> &SyntaxList {
    &self.syntax_list
  }

  fn into_syntax_list(self) -> SyntaxList {
    self.syntax_list
  }
}
impl Debug for SdlExtendingNames {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str("SdlExtendingNames ")?;
    f.debug_list().entries(self.elements()).finish()
  }
}
impl IntoIterator for SdlExtendingNames {
  type IntoIter = AstSeparatedListNodesIterator<Language, Name>;
  type Item = SyntaxResult<Name>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}
impl IntoIterator for &SdlExtendingNames {
  type IntoIter = AstSeparatedListNodesIterator<Language, Name>;
  type Item = SyntaxResult<Name>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SdlIndexList {
  syntax_list: SyntaxList,
}
impl SdlIndexList {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self {
      syntax_list: syntax.into_list(),
    }
  }
}
impl AstNode for SdlIndexList {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_INDEX_LIST as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_INDEX_LIST
  }

  fn cast(syntax: SyntaxNode) -> Option<SdlIndexList> {
    if Self::can_cast(syntax.kind()) {
      Some(SdlIndexList {
        syntax_list: syntax.into_list(),
      })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    self.syntax_list.node()
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax_list.into_node()
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlIndexList {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut seq = serializer.serialize_seq(Some(self.len()))?;
    for e in self.iter() {
      seq.serialize_element(&e)?;
    }
    seq.end()
  }
}
impl AstNodeList for SdlIndexList {
  type Language = Language;
  type Node = SdlIndex;

  fn syntax_list(&self) -> &SyntaxList {
    &self.syntax_list
  }

  fn into_syntax_list(self) -> SyntaxList {
    self.syntax_list
  }
}
impl Debug for SdlIndexList {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str("SdlIndexList ")?;
    f.debug_list().entries(self.iter()).finish()
  }
}
impl IntoIterator for &SdlIndexList {
  type IntoIter = AstNodeListIterator<Language, SdlIndex>;
  type Item = SdlIndex;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}
impl IntoIterator for SdlIndexList {
  type IntoIter = AstNodeListIterator<Language, SdlIndex>;
  type Item = SdlIndex;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SdlLinkList {
  syntax_list: SyntaxList,
}
impl SdlLinkList {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self {
      syntax_list: syntax.into_list(),
    }
  }
}
impl AstNode for SdlLinkList {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_LINK_LIST as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_LINK_LIST
  }

  fn cast(syntax: SyntaxNode) -> Option<SdlLinkList> {
    if Self::can_cast(syntax.kind()) {
      Some(SdlLinkList {
        syntax_list: syntax.into_list(),
      })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    self.syntax_list.node()
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax_list.into_node()
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlLinkList {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut seq = serializer.serialize_seq(Some(self.len()))?;
    for e in self.iter() {
      seq.serialize_element(&e)?;
    }
    seq.end()
  }
}
impl AstNodeList for SdlLinkList {
  type Language = Language;
  type Node = SdlLink;

  fn syntax_list(&self) -> &SyntaxList {
    &self.syntax_list
  }

  fn into_syntax_list(self) -> SyntaxList {
    self.syntax_list
  }
}
impl Debug for SdlLinkList {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str("SdlLinkList ")?;
    f.debug_list().entries(self.iter()).finish()
  }
}
impl IntoIterator for &SdlLinkList {
  type IntoIter = AstNodeListIterator<Language, SdlLink>;
  type Item = SdlLink;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}
impl IntoIterator for SdlLinkList {
  type IntoIter = AstNodeListIterator<Language, SdlLink>;
  type Item = SdlLink;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SdlPropertyList {
  syntax_list: SyntaxList,
}
impl SdlPropertyList {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self {
      syntax_list: syntax.into_list(),
    }
  }
}
impl AstNode for SdlPropertyList {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_PROPERTY_LIST as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_PROPERTY_LIST
  }

  fn cast(syntax: SyntaxNode) -> Option<SdlPropertyList> {
    if Self::can_cast(syntax.kind()) {
      Some(SdlPropertyList {
        syntax_list: syntax.into_list(),
      })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    self.syntax_list.node()
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax_list.into_node()
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlPropertyList {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut seq = serializer.serialize_seq(Some(self.len()))?;
    for e in self.iter() {
      seq.serialize_element(&e)?;
    }
    seq.end()
  }
}
impl AstNodeList for SdlPropertyList {
  type Language = Language;
  type Node = SdlProperty;

  fn syntax_list(&self) -> &SyntaxList {
    &self.syntax_list
  }

  fn into_syntax_list(self) -> SyntaxList {
    self.syntax_list
  }
}
impl Debug for SdlPropertyList {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str("SdlPropertyList ")?;
    f.debug_list().entries(self.iter()).finish()
  }
}
impl IntoIterator for &SdlPropertyList {
  type IntoIter = AstNodeListIterator<Language, SdlProperty>;
  type Item = SdlProperty;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}
impl IntoIterator for SdlPropertyList {
  type IntoIter = AstNodeListIterator<Language, SdlProperty>;
  type Item = SdlProperty;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SdlSchemaConstrainParamList {
  syntax_list: SyntaxList,
}
impl SdlSchemaConstrainParamList {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self {
      syntax_list: syntax.into_list(),
    }
  }
}
impl AstNode for SdlSchemaConstrainParamList {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_SCHEMA_CONSTRAIN_PARAM_LIST as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_SCHEMA_CONSTRAIN_PARAM_LIST
  }

  fn cast(syntax: SyntaxNode) -> Option<SdlSchemaConstrainParamList> {
    if Self::can_cast(syntax.kind()) {
      Some(SdlSchemaConstrainParamList {
        syntax_list: syntax.into_list(),
      })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    self.syntax_list.node()
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax_list.into_node()
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlSchemaConstrainParamList {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut seq = serializer.serialize_seq(Some(self.len()))?;
    for e in self.iter() {
      seq.serialize_element(&e)?;
    }
    seq.end()
  }
}
impl AstNodeList for SdlSchemaConstrainParamList {
  type Language = Language;
  type Node = SdlSchemaConstrainParam;

  fn syntax_list(&self) -> &SyntaxList {
    &self.syntax_list
  }

  fn into_syntax_list(self) -> SyntaxList {
    self.syntax_list
  }
}
impl Debug for SdlSchemaConstrainParamList {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str("SdlSchemaConstrainParamList ")?;
    f.debug_list().entries(self.iter()).finish()
  }
}
impl IntoIterator for &SdlSchemaConstrainParamList {
  type IntoIter = AstNodeListIterator<Language, SdlSchemaConstrainParam>;
  type Item = SdlSchemaConstrainParam;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}
impl IntoIterator for SdlSchemaConstrainParamList {
  type IntoIter = AstNodeListIterator<Language, SdlSchemaConstrainParam>;
  type Item = SdlSchemaConstrainParam;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SdlSchemaStatements {
  syntax_list: SyntaxList,
}
impl SdlSchemaStatements {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self {
      syntax_list: syntax.into_list(),
    }
  }
}
impl AstNode for SdlSchemaStatements {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(SDL_SCHEMA_STATEMENTS as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == SDL_SCHEMA_STATEMENTS
  }

  fn cast(syntax: SyntaxNode) -> Option<SdlSchemaStatements> {
    if Self::can_cast(syntax.kind()) {
      Some(SdlSchemaStatements {
        syntax_list: syntax.into_list(),
      })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    self.syntax_list.node()
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax_list.into_node()
  }
}
#[cfg(feature = "serde")]
impl Serialize for SdlSchemaStatements {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut seq = serializer.serialize_seq(Some(self.len()))?;
    for e in self.iter() {
      seq.serialize_element(&e)?;
    }
    seq.end()
  }
}
impl AstNodeList for SdlSchemaStatements {
  type Language = Language;
  type Node = SdlSchema;

  fn syntax_list(&self) -> &SyntaxList {
    &self.syntax_list
  }

  fn into_syntax_list(self) -> SyntaxList {
    self.syntax_list
  }
}
impl Debug for SdlSchemaStatements {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str("SdlSchemaStatements ")?;
    f.debug_list().entries(self.iter()).finish()
  }
}
impl IntoIterator for &SdlSchemaStatements {
  type IntoIter = AstNodeListIterator<Language, SdlSchema>;
  type Item = SdlSchema;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}
impl IntoIterator for SdlSchemaStatements {
  type IntoIter = AstNodeListIterator<Language, SdlSchema>;
  type Item = SdlSchema;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct TupleTypeMembers {
  syntax_list: SyntaxList,
}
impl TupleTypeMembers {
  #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
  #[doc = r""]
  #[doc = r" # Safety"]
  #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
  #[doc = r" or a match on [SyntaxNode::kind]"]
  #[inline]
  pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
    Self {
      syntax_list: syntax.into_list(),
    }
  }
}
impl AstNode for TupleTypeMembers {
  type Language = Language;

  const KIND_SET: SyntaxKindSet<Language> =
    SyntaxKindSet::from_raw(RawSyntaxKind(TUPLE_TYPE_MEMBERS as u16));

  fn can_cast(kind: SyntaxKind) -> bool {
    kind == TUPLE_TYPE_MEMBERS
  }

  fn cast(syntax: SyntaxNode) -> Option<TupleTypeMembers> {
    if Self::can_cast(syntax.kind()) {
      Some(TupleTypeMembers {
        syntax_list: syntax.into_list(),
      })
    } else {
      None
    }
  }

  fn syntax(&self) -> &SyntaxNode {
    self.syntax_list.node()
  }

  fn into_syntax(self) -> SyntaxNode {
    self.syntax_list.into_node()
  }
}
#[cfg(feature = "serde")]
impl Serialize for TupleTypeMembers {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut seq = serializer.serialize_seq(Some(self.len()))?;
    for e in self.iter() {
      seq.serialize_element(&e)?;
    }
    seq.end()
  }
}
impl AstSeparatedList for TupleTypeMembers {
  type Language = Language;
  type Node = TypeExpression;

  fn syntax_list(&self) -> &SyntaxList {
    &self.syntax_list
  }

  fn into_syntax_list(self) -> SyntaxList {
    self.syntax_list
  }
}
impl Debug for TupleTypeMembers {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str("TupleTypeMembers ")?;
    f.debug_list().entries(self.elements()).finish()
  }
}
impl IntoIterator for TupleTypeMembers {
  type IntoIter = AstSeparatedListNodesIterator<Language, TypeExpression>;
  type Item = SyntaxResult<TypeExpression>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}
impl IntoIterator for &TupleTypeMembers {
  type IntoIter = AstSeparatedListNodesIterator<Language, TypeExpression>;
  type Item = SyntaxResult<TypeExpression>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}
#[derive(Clone)]
pub struct DebugSyntaxElementChildren(pub SyntaxElementChildren);
impl Debug for DebugSyntaxElementChildren {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_list()
      .entries(self.clone().0.map(DebugSyntaxElement))
      .finish()
  }
}
struct DebugSyntaxElement(SyntaxElement);
impl Debug for DebugSyntaxElement {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match &self.0 {
      SyntaxElement::Node(node) => {
        map_syntax_node ! (node . clone () , node => std :: fmt :: Debug :: fmt (& node , f))
      }
      SyntaxElement::Token(token) => Debug::fmt(token, f),
    }
  }
}
