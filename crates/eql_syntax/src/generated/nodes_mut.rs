// Generated file, do not edit by hand, see `xtask/src/codegen`

use std::iter::once;

use rome_rowan::AstNode;

use crate::generated::nodes::*;
use crate::EqlSyntaxToken as SyntaxToken;
impl ArrayType {
  pub fn with_array_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }

  pub fn with_less_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(1usize..=1usize, once(Some(element.into()))),
    )
  }

  pub fn with_member(self, element: PrimitiveType) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
    )
  }

  pub fn with_greater_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(3usize..=3usize, once(Some(element.into()))),
    )
  }
}
impl BareBytesLiteralExpression {
  pub fn with_r_token(self, element: Option<SyntaxToken>) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(element.map(|element| element.into()))),
    )
  }

  pub fn with_b_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(1usize..=1usize, once(Some(element.into()))),
    )
  }

  pub fn with_bare_string_literal_expression(self, element: BareStringLiteralExpression) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
    )
  }
}
impl BareStringLiteralExpression {
  pub fn with_value_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }
}
impl BigIntLiteralExpression {
  pub fn with_value_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }
}
impl BigIntType {
  pub fn with_bigint_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }
}
impl BooleanLiteralExpression {
  pub fn with_value_token_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }
}
impl BooleanType {
  pub fn with_bool_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }
}
impl BytesType {
  pub fn with_bytes_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }
}
impl DateTimeType {
  pub fn with_datetime_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }
}
impl DecimalLiteralExpression {
  pub fn with_value_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }
}
impl DecimalType {
  pub fn with_decimal_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }
}
impl DotReferenceName {
  pub fn with_name_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }

  pub fn with_dot_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(1usize..=1usize, once(Some(element.into()))),
    )
  }

  pub fn with_path_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(2usize..=2usize, once(Some(element.into()))),
    )
  }
}
impl DurationType {
  pub fn with_duration_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }
}
impl EmptyStatement {
  pub fn with_semicolon_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }
}
impl EqlRoot {
  pub fn with_eql_root_item_list(self, element: EqlRootItemList) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
    )
  }
}
impl Expression {
  pub fn with_unknown_expression(self, element: UnknownExpression) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
    )
  }
}
impl FloatLiteralExpression {
  pub fn with_value_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }
}
impl FloatSixtyFourType {
  pub fn with_float64_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }
}
impl FloatThirtyTwoType {
  pub fn with_float32_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }
}
impl IntLiteralExpression {
  pub fn with_value_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }
}
impl IntSixteenType {
  pub fn with_int16_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }
}
impl IntSixtyFourType {
  pub fn with_int64_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }
}
impl IntThirtyTwoType {
  pub fn with_int32_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }
}
impl JsonType {
  pub fn with_json_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }
}
impl ParameterName {
  pub fn with_dollar_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }

  pub fn with_parameter_name_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(1usize..=1usize, once(Some(element.into()))),
    )
  }
}
impl QualifiedName {
  pub fn with_name_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }

  pub fn with_namespace_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(1usize..=1usize, once(Some(element.into()))),
    )
  }

  pub fn with_path_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(2usize..=2usize, once(Some(element.into()))),
    )
  }
}
impl RangeType {
  pub fn with_range_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }

  pub fn with_less_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(1usize..=1usize, once(Some(element.into()))),
    )
  }

  pub fn with_member(self, element: RangeTypeMember) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
    )
  }

  pub fn with_greater_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(3usize..=3usize, once(Some(element.into()))),
    )
  }
}
impl RawBytesLiteralExpression {
  pub fn with_b_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }

  pub fn with_raw_string_literal_expression(self, element: RawStringLiteralExpression) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
    )
  }
}
impl RawStringLiteralExpression {
  pub fn with_r_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }

  pub fn with_value_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(1usize..=1usize, once(Some(element.into()))),
    )
  }
}
impl SdlAnnotation {
  pub fn with_annotation_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }

  pub fn with_name(self, element: UnqualifiedName) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
    )
  }

  pub fn with_assign_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(2usize..=2usize, once(Some(element.into()))),
    )
  }

  pub fn with_value(self, element: StringLiteralExpression) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(3usize..=3usize, once(Some(element.into_syntax().into()))),
    )
  }

  pub fn with_empty_statement(self, element: EmptyStatement) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(4usize..=4usize, once(Some(element.into_syntax().into()))),
    )
  }
}
impl SdlAnnotationSchema {
  pub fn with_abstract_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }

  pub fn with_inheritable_token(self, element: Option<SyntaxToken>) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(1usize..=1usize, once(element.map(|element| element.into()))),
    )
  }

  pub fn with_annotation_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(2usize..=2usize, once(Some(element.into()))),
    )
  }

  pub fn with_name(self, element: Name) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(3usize..=3usize, once(Some(element.into_syntax().into()))),
    )
  }

  pub fn with_body(self, element: SdlAnnotationSchemaBody) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(4usize..=4usize, once(Some(element.into_syntax().into()))),
    )
  }
}
impl SdlAnnotationSchemaBlock {
  pub fn with_open_curly_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }

  pub fn with_annotations(self, element: SdlAnnotationList) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
    )
  }

  pub fn with_close_curly_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(2usize..=2usize, once(Some(element.into()))),
    )
  }

  pub fn with_semicolon_token(self, element: Option<SyntaxToken>) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(3usize..=3usize, once(element.map(|element| element.into()))),
    )
  }
}
impl SdlConstraint {
  pub fn with_delegated_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }

  pub fn with_constraint_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(1usize..=1usize, once(Some(element.into()))),
    )
  }

  pub fn with_name(self, element: Name) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
    )
  }

  pub fn with_args(self, element: Option<SdlConstraintArgs>) -> Self {
    Self::unwrap_cast(self.syntax.splice_slots(
      3usize..=3usize,
      once(element.map(|element| element.into_syntax().into())),
    ))
  }

  pub fn with_subject(self, element: Option<SdlConstraintSubjectExpression>) -> Self {
    Self::unwrap_cast(self.syntax.splice_slots(
      4usize..=4usize,
      once(element.map(|element| element.into_syntax().into())),
    ))
  }

  pub fn with_except(self, element: Option<SdlConstraintExcept>) -> Self {
    Self::unwrap_cast(self.syntax.splice_slots(
      5usize..=5usize,
      once(element.map(|element| element.into_syntax().into())),
    ))
  }

  pub fn with_body(self, element: SdlConstraintBody) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(6usize..=6usize, once(Some(element.into_syntax().into()))),
    )
  }
}
impl SdlConstraintArg {
  pub fn with_name(self, element: UnqualifiedName) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
    )
  }

  pub fn with_name_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(1usize..=1usize, once(Some(element.into()))),
    )
  }

  pub fn with_value(self, element: Expression) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
    )
  }
}
impl SdlConstraintArgs {
  pub fn with_open_paren_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }

  pub fn with_args(self, element: SdlConstraintArgList) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
    )
  }

  pub fn with_close_paren_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(2usize..=2usize, once(Some(element.into()))),
    )
  }
}
impl SdlConstraintBlock {
  pub fn with_open_curly_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }

  pub fn with_annotations(self, element: SdlAnnotationList) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
    )
  }

  pub fn with_error_message(self, element: Option<SdlConstraintErrorMessage>) -> Self {
    Self::unwrap_cast(self.syntax.splice_slots(
      2usize..=2usize,
      once(element.map(|element| element.into_syntax().into())),
    ))
  }

  pub fn with_close_curly_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(3usize..=3usize, once(Some(element.into()))),
    )
  }

  pub fn with_semicolon_token(self, element: Option<SyntaxToken>) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(4usize..=4usize, once(element.map(|element| element.into()))),
    )
  }
}
impl SdlConstraintErrorMessage {
  pub fn with_errmessage_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }

  pub fn with_assign_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(1usize..=1usize, once(Some(element.into()))),
    )
  }

  pub fn with_message(self, element: StringLiteralExpression) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
    )
  }

  pub fn with_empty_statement(self, element: EmptyStatement) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(3usize..=3usize, once(Some(element.into_syntax().into()))),
    )
  }
}
impl SdlConstraintExcept {
  pub fn with_except_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }

  pub fn with_expression(self, element: Expression) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
    )
  }
}
impl SdlConstraintSubjectExpression {
  pub fn with_on_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }

  pub fn with_open_paren_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(1usize..=1usize, once(Some(element.into()))),
    )
  }

  pub fn with_expression(self, element: Expression) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
    )
  }

  pub fn with_close_paren_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(3usize..=3usize, once(Some(element.into()))),
    )
  }
}
impl SdlEnumDeclaration {
  pub fn with_enum_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }

  pub fn with_less_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(1usize..=1usize, once(Some(element.into()))),
    )
  }

  pub fn with_members(self, element: SdlEnumDeclarationMembers) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
    )
  }

  pub fn with_greater_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(3usize..=3usize, once(Some(element.into()))),
    )
  }
}
impl SdlExtending {
  pub fn with_extending_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }

  pub fn with_extends(self, element: SdlExtendingNames) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
    )
  }
}
impl SdlIndex {
  pub fn with_sdl_unknown(self, element: SdlUnknown) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
    )
  }
}
impl SdlLink {
  pub fn with_sdl_unknown(self, element: SdlUnknown) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
    )
  }
}
impl SdlModule {
  pub fn with_module_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }

  pub fn with_unqualified_name(self, element: UnqualifiedName) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
    )
  }

  pub fn with_open_curly_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(2usize..=2usize, once(Some(element.into()))),
    )
  }

  pub fn with_statements(self, element: SdlSchemaStatements) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(3usize..=3usize, once(Some(element.into_syntax().into()))),
    )
  }

  pub fn with_close_curly_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(4usize..=4usize, once(Some(element.into()))),
    )
  }

  pub fn with_semicolon_token(self, element: Option<SyntaxToken>) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(5usize..=5usize, once(element.map(|element| element.into()))),
    )
  }
}
impl SdlObjectBlock {
  pub fn with_open_curly_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }

  pub fn with_annotations(self, element: SdlAnnotationList) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
    )
  }

  pub fn with_properties(self, element: SdlPropertyList) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
    )
  }

  pub fn with_links(self, element: SdlLinkList) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(3usize..=3usize, once(Some(element.into_syntax().into()))),
    )
  }

  pub fn with_constraints(self, element: SdlConstraintList) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(4usize..=4usize, once(Some(element.into_syntax().into()))),
    )
  }

  pub fn with_indexes(self, element: SdlIndexList) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(5usize..=5usize, once(Some(element.into_syntax().into()))),
    )
  }

  pub fn with_close_curly_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(6usize..=6usize, once(Some(element.into()))),
    )
  }

  pub fn with_semicolon_token(self, element: Option<SyntaxToken>) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(7usize..=7usize, once(element.map(|element| element.into()))),
    )
  }
}
impl SdlObjectSchema {
  pub fn with_abstract_token(self, element: Option<SyntaxToken>) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(element.map(|element| element.into()))),
    )
  }

  pub fn with_type_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(1usize..=1usize, once(Some(element.into()))),
    )
  }

  pub fn with_name(self, element: Name) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
    )
  }

  pub fn with_extending(self, element: Option<SdlExtending>) -> Self {
    Self::unwrap_cast(self.syntax.splice_slots(
      3usize..=3usize,
      once(element.map(|element| element.into_syntax().into())),
    ))
  }

  pub fn with_body(self, element: SdlObjectBody) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(4usize..=4usize, once(Some(element.into_syntax().into()))),
    )
  }
}
impl SdlProperty {
  pub fn with_sdl_unknown(self, element: SdlUnknown) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
    )
  }
}
impl SdlRoot {
  pub fn with_sdl_root_item_list(self, element: SdlRootItemList) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
    )
  }
}
impl SdlScalarBlock {
  pub fn with_open_curly_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }

  pub fn with_annotations(self, element: SdlAnnotationList) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
    )
  }

  pub fn with_constraints(self, element: SdlConstraintList) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
    )
  }

  pub fn with_close_curly_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(3usize..=3usize, once(Some(element.into()))),
    )
  }

  pub fn with_semicolon_token(self, element: Option<SyntaxToken>) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(4usize..=4usize, once(element.map(|element| element.into()))),
    )
  }
}
impl SdlScalarExtendingEnum {
  pub fn with_extending_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }

  pub fn with_extends(self, element: SdlEnumDeclaration) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
    )
  }
}
impl SdlScalarExtendingType {
  pub fn with_extending_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }

  pub fn with_extends(self, element: SdlExtendingNames) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
    )
  }
}
impl SdlSchemaConstrainParam {
  pub fn with_name(self, element: UnqualifiedName) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
    )
  }

  pub fn with_name_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(1usize..=1usize, once(Some(element.into()))),
    )
  }

  pub fn with_param_type(self, element: TypeExpression) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
    )
  }
}
impl SdlSchemaConstraint {
  pub fn with_abstract_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }

  pub fn with_constraint_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(1usize..=1usize, once(Some(element.into()))),
    )
  }

  pub fn with_name(self, element: Name) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
    )
  }

  pub fn with_params(self, element: Option<SdlSchemaConstraintArgs>) -> Self {
    Self::unwrap_cast(self.syntax.splice_slots(
      3usize..=3usize,
      once(element.map(|element| element.into_syntax().into())),
    ))
  }

  pub fn with_subject(self, element: Option<SdlConstraintSubjectExpression>) -> Self {
    Self::unwrap_cast(self.syntax.splice_slots(
      4usize..=4usize,
      once(element.map(|element| element.into_syntax().into())),
    ))
  }

  pub fn with_extending(self, element: Option<SdlExtending>) -> Self {
    Self::unwrap_cast(self.syntax.splice_slots(
      5usize..=5usize,
      once(element.map(|element| element.into_syntax().into())),
    ))
  }

  pub fn with_body(self, element: SdlSchemaConstraintBody) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(6usize..=6usize, once(Some(element.into_syntax().into()))),
    )
  }
}
impl SdlSchemaConstraintArgs {
  pub fn with_open_paren_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }

  pub fn with_params(self, element: SdlSchemaConstrainParamList) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
    )
  }

  pub fn with_close_paren_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(2usize..=2usize, once(Some(element.into()))),
    )
  }
}
impl SdlSchemaConstraintBlock {
  pub fn with_open_curly_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }

  pub fn with_using(self, element: Expression) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
    )
  }

  pub fn with_annotations(self, element: SdlAnnotationList) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
    )
  }

  pub fn with_error_message(self, element: Option<SdlConstraintErrorMessage>) -> Self {
    Self::unwrap_cast(self.syntax.splice_slots(
      3usize..=3usize,
      once(element.map(|element| element.into_syntax().into())),
    ))
  }

  pub fn with_close_curly_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(4usize..=4usize, once(Some(element.into()))),
    )
  }

  pub fn with_semicolon_token(self, element: Option<SyntaxToken>) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(5usize..=5usize, once(element.map(|element| element.into()))),
    )
  }
}
impl SdlSchemaScalar {
  pub fn with_abstract_token(self, element: Option<SyntaxToken>) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(element.map(|element| element.into()))),
    )
  }

  pub fn with_scalar_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(1usize..=1usize, once(Some(element.into()))),
    )
  }

  pub fn with_type_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(2usize..=2usize, once(Some(element.into()))),
    )
  }

  pub fn with_name(self, element: Name) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(3usize..=3usize, once(Some(element.into_syntax().into()))),
    )
  }

  pub fn with_extending(self, element: Option<SdlScalarExtending>) -> Self {
    Self::unwrap_cast(self.syntax.splice_slots(
      4usize..=4usize,
      once(element.map(|element| element.into_syntax().into())),
    ))
  }

  pub fn with_body(self, element: SdlScalarBody) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(5usize..=5usize, once(Some(element.into_syntax().into()))),
    )
  }
}
impl SequenceType {
  pub fn with_sequence_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }
}
impl Statement {
  pub fn with_unknown_statement(self, element: UnknownStatement) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
    )
  }
}
impl StringType {
  pub fn with_str_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }
}
impl TupleType {
  pub fn with_tuple_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }

  pub fn with_less_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(1usize..=1usize, once(Some(element.into()))),
    )
  }

  pub fn with_members(self, element: TupleTypeMembers) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
    )
  }

  pub fn with_greater_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(3usize..=3usize, once(Some(element.into()))),
    )
  }
}
impl TypeCastExpression {
  pub fn with_less_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }

  pub fn with_ty(self, element: TypeExpression) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
    )
  }

  pub fn with_greater_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(2usize..=2usize, once(Some(element.into()))),
    )
  }

  pub fn with_target(self, element: TypeCastTarget) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(3usize..=3usize, once(Some(element.into_syntax().into()))),
    )
  }
}
impl UnqualifiedName {
  pub fn with_name_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }
}
impl UuidType {
  pub fn with_uuid_token(self, element: SyntaxToken) -> Self {
    Self::unwrap_cast(
      self
        .syntax
        .splice_slots(0usize..=0usize, once(Some(element.into()))),
    )
  }
}
