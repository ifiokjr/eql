use eql_syntax::JsLanguage;
use rome_rowan::TreeBuilder;

use crate::generated::JsSyntaxFactory;

mod generated;
pub mod make;

// Re-exported for tests
#[doc(hidden)]
pub use eql_syntax as syntax;

pub type JsSyntaxTreeBuilder = TreeBuilder<'static, JsLanguage, JsSyntaxFactory>;
