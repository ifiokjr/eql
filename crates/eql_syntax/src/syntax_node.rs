//! This module defines the Concrete Syntax Tree used by `eql`.
//!
//! The tree is entirely lossless, whitespace, comments, and errors are
//! preserved. It also provides traversal methods including parent, children,
//! and siblings of nodes.
//!
//! This is a simple wrapper around the `rowan` crate which does most of the
//! heavy lifting and is language agnostic.

use rome_rowan::Language;
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::AnyRoot;
use crate::EqlSyntaxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, schemars::JsonSchema))]
pub struct EqlLanguage;

impl Language for EqlLanguage {
  type Kind = EqlSyntaxKind;
  type Root = AnyRoot;
}

pub type EqlSyntaxNode = rome_rowan::SyntaxNode<EqlLanguage>;
pub type EqlSyntaxToken = rome_rowan::SyntaxToken<EqlLanguage>;
pub type EqlSyntaxElement = rome_rowan::SyntaxElement<EqlLanguage>;
pub type EqlSyntaxNodeChildren = rome_rowan::SyntaxNodeChildren<EqlLanguage>;
pub type EqlSyntaxElementChildren = rome_rowan::SyntaxElementChildren<EqlLanguage>;
pub type EqlSyntaxList = rome_rowan::SyntaxList<EqlLanguage>;
pub type EqlSyntaxTrivia = rome_rowan::syntax::SyntaxTrivia<EqlLanguage>;
