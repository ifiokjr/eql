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
impl std::fmt::Display for EmptyStatement {
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
