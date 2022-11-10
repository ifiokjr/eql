use eql_syntax::EqlLanguage;
use rome_rowan::TreeBuilder;

use crate::generated::EqlSyntaxFactory;

mod generated;
pub mod make;

// Re-exported for tests
#[doc(hidden)]
pub use eql_syntax as syntax;

pub type JsSyntaxTreeBuilder = TreeBuilder<'static, EqlLanguage, EqlSyntaxFactory>;
