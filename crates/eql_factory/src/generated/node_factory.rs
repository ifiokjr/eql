#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
use eql_syntax::EqlSyntaxElement as SyntaxElement;
use eql_syntax::EqlSyntaxNode as SyntaxNode;
use eql_syntax::EqlSyntaxToken as SyntaxToken;
use eql_syntax::*;
use rome_rowan::AstNode;
pub fn array_type(
  array_token: SyntaxToken,
  less_token: SyntaxToken,
  member: PrimitiveType,
  greater_token: SyntaxToken,
) -> ArrayType {
  ArrayType::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::ARRAY_TYPE,
    [
      Some(SyntaxElement::Token(array_token)),
      Some(SyntaxElement::Token(less_token)),
      Some(SyntaxElement::Node(member.into_syntax())),
      Some(SyntaxElement::Token(greater_token)),
    ],
  ))
}
pub fn bare_bytes_literal_expression(
  b_token: SyntaxToken,
  bare_string_literal_expression: BareStringLiteralExpression,
) -> BareBytesLiteralExpressionBuilder {
  BareBytesLiteralExpressionBuilder {
    b_token,
    bare_string_literal_expression,
    r_token: None,
  }
}
pub struct BareBytesLiteralExpressionBuilder {
  b_token: SyntaxToken,
  bare_string_literal_expression: BareStringLiteralExpression,
  r_token: Option<SyntaxToken>,
}
impl BareBytesLiteralExpressionBuilder {
  pub fn with_r_token(mut self, r_token: SyntaxToken) -> Self {
    self.r_token = Some(r_token);
    self
  }

  pub fn build(self) -> BareBytesLiteralExpression {
    BareBytesLiteralExpression::unwrap_cast(SyntaxNode::new_detached(
      EqlSyntaxKind::BARE_BYTES_LITERAL_EXPRESSION,
      [
        self.r_token.map(|token| SyntaxElement::Token(token)),
        Some(SyntaxElement::Token(self.b_token)),
        Some(SyntaxElement::Node(
          self.bare_string_literal_expression.into_syntax(),
        )),
      ],
    ))
  }
}
pub fn bare_string_literal_expression(value_token: SyntaxToken) -> BareStringLiteralExpression {
  BareStringLiteralExpression::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::BARE_STRING_LITERAL_EXPRESSION,
    [Some(SyntaxElement::Token(value_token))],
  ))
}
pub fn big_int_literal_expression(value_token: SyntaxToken) -> BigIntLiteralExpression {
  BigIntLiteralExpression::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::BIG_INT_LITERAL_EXPRESSION,
    [Some(SyntaxElement::Token(value_token))],
  ))
}
pub fn big_int_type(bigint_token: SyntaxToken) -> BigIntType {
  BigIntType::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::BIG_INT_TYPE,
    [Some(SyntaxElement::Token(bigint_token))],
  ))
}
pub fn boolean_literal_expression(value_token_token: SyntaxToken) -> BooleanLiteralExpression {
  BooleanLiteralExpression::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::BOOLEAN_LITERAL_EXPRESSION,
    [Some(SyntaxElement::Token(value_token_token))],
  ))
}
pub fn boolean_type(bool_token: SyntaxToken) -> BooleanType {
  BooleanType::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::BOOLEAN_TYPE,
    [Some(SyntaxElement::Token(bool_token))],
  ))
}
pub fn bytes_type(bytes_token: SyntaxToken) -> BytesType {
  BytesType::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::BYTES_TYPE,
    [Some(SyntaxElement::Token(bytes_token))],
  ))
}
pub fn date_time_type(datetime_token: SyntaxToken) -> DateTimeType {
  DateTimeType::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::DATE_TIME_TYPE,
    [Some(SyntaxElement::Token(datetime_token))],
  ))
}
pub fn decimal_literal_expression(value_token: SyntaxToken) -> DecimalLiteralExpression {
  DecimalLiteralExpression::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::DECIMAL_LITERAL_EXPRESSION,
    [Some(SyntaxElement::Token(value_token))],
  ))
}
pub fn decimal_type(decimal_token: SyntaxToken) -> DecimalType {
  DecimalType::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::DECIMAL_TYPE,
    [Some(SyntaxElement::Token(decimal_token))],
  ))
}
pub fn dot_reference_name(
  unqualified_name: UnqualifiedName,
  dot_token: SyntaxToken,
  unqualified_name: UnqualifiedName,
) -> DotReferenceName {
  DotReferenceName::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::DOT_REFERENCE_NAME,
    [
      Some(SyntaxElement::Node(unqualified_name.into_syntax())),
      Some(SyntaxElement::Token(dot_token)),
      Some(SyntaxElement::Node(unqualified_name.into_syntax())),
    ],
  ))
}
pub fn duration_type(duration_token: SyntaxToken) -> DurationType {
  DurationType::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::DURATION_TYPE,
    [Some(SyntaxElement::Token(duration_token))],
  ))
}
pub fn empty_statement(semicolon_token: SyntaxToken) -> EmptyStatement {
  EmptyStatement::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::EMPTY_STATEMENT,
    [Some(SyntaxElement::Token(semicolon_token))],
  ))
}
pub fn expression(unknown: Unknown) -> Expression {
  Expression::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::EXPRESSION,
    [Some(SyntaxElement::Node(unknown.into_syntax()))],
  ))
}
pub fn float_literal_expression(value_token: SyntaxToken) -> FloatLiteralExpression {
  FloatLiteralExpression::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::FLOAT_LITERAL_EXPRESSION,
    [Some(SyntaxElement::Token(value_token))],
  ))
}
pub fn float_sixty_four_type(float64_token: SyntaxToken) -> FloatSixtyFourType {
  FloatSixtyFourType::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::FLOAT_SIXTY_FOUR_TYPE,
    [Some(SyntaxElement::Token(float64_token))],
  ))
}
pub fn float_thirty_two_type(float32_token: SyntaxToken) -> FloatThirtyTwoType {
  FloatThirtyTwoType::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::FLOAT_THIRTY_TWO_TYPE,
    [Some(SyntaxElement::Token(float32_token))],
  ))
}
pub fn int_literal_expression(value_token: SyntaxToken) -> IntLiteralExpression {
  IntLiteralExpression::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::INT_LITERAL_EXPRESSION,
    [Some(SyntaxElement::Token(value_token))],
  ))
}
pub fn int_sixteen_type(int16_token: SyntaxToken) -> IntSixteenType {
  IntSixteenType::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::INT_SIXTEEN_TYPE,
    [Some(SyntaxElement::Token(int16_token))],
  ))
}
pub fn int_sixty_four_type(int64_token: SyntaxToken) -> IntSixtyFourType {
  IntSixtyFourType::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::INT_SIXTY_FOUR_TYPE,
    [Some(SyntaxElement::Token(int64_token))],
  ))
}
pub fn int_thirty_two_type(int32_token: SyntaxToken) -> IntThirtyTwoType {
  IntThirtyTwoType::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::INT_THIRTY_TWO_TYPE,
    [Some(SyntaxElement::Token(int32_token))],
  ))
}
pub fn json_type(json_token: SyntaxToken) -> JsonType {
  JsonType::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::JSON_TYPE,
    [Some(SyntaxElement::Token(json_token))],
  ))
}
pub fn parameter_name(dollar_token: SyntaxToken, name: UnqualifiedName) -> ParameterName {
  ParameterName::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::PARAMETER_NAME,
    [
      Some(SyntaxElement::Token(dollar_token)),
      Some(SyntaxElement::Node(name.into_syntax())),
    ],
  ))
}
pub fn qualified_name(
  namespace: UnqualifiedName,
  namespace_token: SyntaxToken,
  name: UnqualifiedName,
) -> QualifiedName {
  QualifiedName::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::QUALIFIED_NAME,
    [
      Some(SyntaxElement::Node(namespace.into_syntax())),
      Some(SyntaxElement::Token(namespace_token)),
      Some(SyntaxElement::Node(name.into_syntax())),
    ],
  ))
}
pub fn range_type(
  range_token: SyntaxToken,
  less_token: SyntaxToken,
  member: RangeTypeMember,
  greater_token: SyntaxToken,
) -> RangeType {
  RangeType::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::RANGE_TYPE,
    [
      Some(SyntaxElement::Token(range_token)),
      Some(SyntaxElement::Token(less_token)),
      Some(SyntaxElement::Node(member.into_syntax())),
      Some(SyntaxElement::Token(greater_token)),
    ],
  ))
}
pub fn raw_bytes_literal_expression(
  b_token: SyntaxToken,
  raw_string_literal_expression: RawStringLiteralExpression,
) -> RawBytesLiteralExpression {
  RawBytesLiteralExpression::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::RAW_BYTES_LITERAL_EXPRESSION,
    [
      Some(SyntaxElement::Token(b_token)),
      Some(SyntaxElement::Node(
        raw_string_literal_expression.into_syntax(),
      )),
    ],
  ))
}
pub fn raw_string_literal_expression(
  r_token: SyntaxToken,
  value_token: SyntaxToken,
) -> RawStringLiteralExpression {
  RawStringLiteralExpression::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::RAW_STRING_LITERAL_EXPRESSION,
    [
      Some(SyntaxElement::Token(r_token)),
      Some(SyntaxElement::Token(value_token)),
    ],
  ))
}
pub fn sdl_annotation(
  annotation_token: SyntaxToken,
  name: UnqualifiedName,
  assign_token: SyntaxToken,
  value: StringLiteralExpression,
  empty_statement: EmptyStatement,
) -> SdlAnnotation {
  SdlAnnotation::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::SDL_ANNOTATION,
    [
      Some(SyntaxElement::Token(annotation_token)),
      Some(SyntaxElement::Node(name.into_syntax())),
      Some(SyntaxElement::Token(assign_token)),
      Some(SyntaxElement::Node(value.into_syntax())),
      Some(SyntaxElement::Node(empty_statement.into_syntax())),
    ],
  ))
}
pub fn sdl_annotation_schema(
  abstract_token: SyntaxToken,
  annotation_token: SyntaxToken,
  name: Name,
  body: SdlAnnotationSchemaBody,
) -> SdlAnnotationSchemaBuilder {
  SdlAnnotationSchemaBuilder {
    abstract_token,
    annotation_token,
    name,
    body,
    inheritable_token: None,
  }
}
pub struct SdlAnnotationSchemaBuilder {
  abstract_token: SyntaxToken,
  annotation_token: SyntaxToken,
  name: Name,
  body: SdlAnnotationSchemaBody,
  inheritable_token: Option<SyntaxToken>,
}
impl SdlAnnotationSchemaBuilder {
  pub fn with_inheritable_token(mut self, inheritable_token: SyntaxToken) -> Self {
    self.inheritable_token = Some(inheritable_token);
    self
  }

  pub fn build(self) -> SdlAnnotationSchema {
    SdlAnnotationSchema::unwrap_cast(SyntaxNode::new_detached(
      EqlSyntaxKind::SDL_ANNOTATION_SCHEMA,
      [
        Some(SyntaxElement::Token(self.abstract_token)),
        self
          .inheritable_token
          .map(|token| SyntaxElement::Token(token)),
        Some(SyntaxElement::Token(self.annotation_token)),
        Some(SyntaxElement::Node(self.name.into_syntax())),
        Some(SyntaxElement::Node(self.body.into_syntax())),
      ],
    ))
  }
}
pub fn sdl_annotation_schema_block(
  open_curly_token: SyntaxToken,
  annotations: SdlAnnotationList,
  close_curly_token: SyntaxToken,
) -> SdlAnnotationSchemaBlockBuilder {
  SdlAnnotationSchemaBlockBuilder {
    open_curly_token,
    annotations,
    close_curly_token,
    semicolon_token: None,
  }
}
pub struct SdlAnnotationSchemaBlockBuilder {
  open_curly_token: SyntaxToken,
  annotations: SdlAnnotationList,
  close_curly_token: SyntaxToken,
  semicolon_token: Option<SyntaxToken>,
}
impl SdlAnnotationSchemaBlockBuilder {
  pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
    self.semicolon_token = Some(semicolon_token);
    self
  }

  pub fn build(self) -> SdlAnnotationSchemaBlock {
    SdlAnnotationSchemaBlock::unwrap_cast(SyntaxNode::new_detached(
      EqlSyntaxKind::SDL_ANNOTATION_SCHEMA_BLOCK,
      [
        Some(SyntaxElement::Token(self.open_curly_token)),
        Some(SyntaxElement::Node(self.annotations.into_syntax())),
        Some(SyntaxElement::Token(self.close_curly_token)),
        self
          .semicolon_token
          .map(|token| SyntaxElement::Token(token)),
      ],
    ))
  }
}
pub fn sdl_constraint(
  delegated_token: SyntaxToken,
  constraint_token: SyntaxToken,
  name: Name,
  body: SdlConstraintBody,
) -> SdlConstraintBuilder {
  SdlConstraintBuilder {
    delegated_token,
    constraint_token,
    name,
    body,
    args: None,
    subject: None,
    except: None,
  }
}
pub struct SdlConstraintBuilder {
  delegated_token: SyntaxToken,
  constraint_token: SyntaxToken,
  name: Name,
  body: SdlConstraintBody,
  args: Option<SdlConstraintArgs>,
  subject: Option<SdlConstraintSubjectExpression>,
  except: Option<SdlConstraintExcept>,
}
impl SdlConstraintBuilder {
  pub fn with_args(mut self, args: SdlConstraintArgs) -> Self {
    self.args = Some(args);
    self
  }

  pub fn with_subject(mut self, subject: SdlConstraintSubjectExpression) -> Self {
    self.subject = Some(subject);
    self
  }

  pub fn with_except(mut self, except: SdlConstraintExcept) -> Self {
    self.except = Some(except);
    self
  }

  pub fn build(self) -> SdlConstraint {
    SdlConstraint::unwrap_cast(SyntaxNode::new_detached(
      EqlSyntaxKind::SDL_CONSTRAINT,
      [
        Some(SyntaxElement::Token(self.delegated_token)),
        Some(SyntaxElement::Token(self.constraint_token)),
        Some(SyntaxElement::Node(self.name.into_syntax())),
        self
          .args
          .map(|token| SyntaxElement::Node(token.into_syntax())),
        self
          .subject
          .map(|token| SyntaxElement::Node(token.into_syntax())),
        self
          .except
          .map(|token| SyntaxElement::Node(token.into_syntax())),
        Some(SyntaxElement::Node(self.body.into_syntax())),
      ],
    ))
  }
}
pub fn sdl_constraint_arg(
  name: UnqualifiedName,
  name_token: SyntaxToken,
  value: Expression,
) -> SdlConstraintArg {
  SdlConstraintArg::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::SDL_CONSTRAINT_ARG,
    [
      Some(SyntaxElement::Node(name.into_syntax())),
      Some(SyntaxElement::Token(name_token)),
      Some(SyntaxElement::Node(value.into_syntax())),
    ],
  ))
}
pub fn sdl_constraint_args(
  open_paren_token: SyntaxToken,
  args: SdlConstraintArgList,
  close_paren_token: SyntaxToken,
) -> SdlConstraintArgs {
  SdlConstraintArgs::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::SDL_CONSTRAINT_ARGS,
    [
      Some(SyntaxElement::Token(open_paren_token)),
      Some(SyntaxElement::Node(args.into_syntax())),
      Some(SyntaxElement::Token(close_paren_token)),
    ],
  ))
}
pub fn sdl_constraint_block(
  open_curly_token: SyntaxToken,
  annotations: SdlAnnotationList,
  close_curly_token: SyntaxToken,
) -> SdlConstraintBlockBuilder {
  SdlConstraintBlockBuilder {
    open_curly_token,
    annotations,
    close_curly_token,
    error_message: None,
    semicolon_token: None,
  }
}
pub struct SdlConstraintBlockBuilder {
  open_curly_token: SyntaxToken,
  annotations: SdlAnnotationList,
  close_curly_token: SyntaxToken,
  error_message: Option<SdlConstraintErrorMessage>,
  semicolon_token: Option<SyntaxToken>,
}
impl SdlConstraintBlockBuilder {
  pub fn with_error_message(mut self, error_message: SdlConstraintErrorMessage) -> Self {
    self.error_message = Some(error_message);
    self
  }

  pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
    self.semicolon_token = Some(semicolon_token);
    self
  }

  pub fn build(self) -> SdlConstraintBlock {
    SdlConstraintBlock::unwrap_cast(SyntaxNode::new_detached(
      EqlSyntaxKind::SDL_CONSTRAINT_BLOCK,
      [
        Some(SyntaxElement::Token(self.open_curly_token)),
        Some(SyntaxElement::Node(self.annotations.into_syntax())),
        self
          .error_message
          .map(|token| SyntaxElement::Node(token.into_syntax())),
        Some(SyntaxElement::Token(self.close_curly_token)),
        self
          .semicolon_token
          .map(|token| SyntaxElement::Token(token)),
      ],
    ))
  }
}
pub fn sdl_constraint_error_message(
  errmessage_token: SyntaxToken,
  assign_token: SyntaxToken,
  message: StringLiteralExpression,
  empty_statement: EmptyStatement,
) -> SdlConstraintErrorMessage {
  SdlConstraintErrorMessage::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::SDL_CONSTRAINT_ERROR_MESSAGE,
    [
      Some(SyntaxElement::Token(errmessage_token)),
      Some(SyntaxElement::Token(assign_token)),
      Some(SyntaxElement::Node(message.into_syntax())),
      Some(SyntaxElement::Node(empty_statement.into_syntax())),
    ],
  ))
}
pub fn sdl_constraint_except(
  except_token: SyntaxToken,
  expression: Expression,
) -> SdlConstraintExcept {
  SdlConstraintExcept::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::SDL_CONSTRAINT_EXCEPT,
    [
      Some(SyntaxElement::Token(except_token)),
      Some(SyntaxElement::Node(expression.into_syntax())),
    ],
  ))
}
pub fn sdl_constraint_subject_expression(
  on_token: SyntaxToken,
  open_paren_token: SyntaxToken,
  expression: Expression,
  close_paren_token: SyntaxToken,
) -> SdlConstraintSubjectExpression {
  SdlConstraintSubjectExpression::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::SDL_CONSTRAINT_SUBJECT_EXPRESSION,
    [
      Some(SyntaxElement::Token(on_token)),
      Some(SyntaxElement::Token(open_paren_token)),
      Some(SyntaxElement::Node(expression.into_syntax())),
      Some(SyntaxElement::Token(close_paren_token)),
    ],
  ))
}
pub fn sdl_enum_declaration(
  enum_token: SyntaxToken,
  less_token: SyntaxToken,
  members: SdlEnumDeclarationMembers,
  greater_token: SyntaxToken,
) -> SdlEnumDeclaration {
  SdlEnumDeclaration::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::SDL_ENUM_DECLARATION,
    [
      Some(SyntaxElement::Token(enum_token)),
      Some(SyntaxElement::Token(less_token)),
      Some(SyntaxElement::Node(members.into_syntax())),
      Some(SyntaxElement::Token(greater_token)),
    ],
  ))
}
pub fn sdl_extending(extending_token: SyntaxToken, extends: SdlExtendingNames) -> SdlExtending {
  SdlExtending::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::SDL_EXTENDING,
    [
      Some(SyntaxElement::Token(extending_token)),
      Some(SyntaxElement::Node(extends.into_syntax())),
    ],
  ))
}
pub fn sdl_index(sdl_unknown: SdlUnknown) -> SdlIndex {
  SdlIndex::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::SDL_INDEX,
    [Some(SyntaxElement::Node(sdl_unknown.into_syntax()))],
  ))
}
pub fn sdl_link(sdl_unknown: SdlUnknown) -> SdlLink {
  SdlLink::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::SDL_LINK,
    [Some(SyntaxElement::Node(sdl_unknown.into_syntax()))],
  ))
}
pub fn sdl_module(
  module_token: SyntaxToken,
  unqualified_name: UnqualifiedName,
  open_curly_token: SyntaxToken,
  statements: SdlSchemaStatments,
  close_curly_token: SyntaxToken,
) -> SdlModuleBuilder {
  SdlModuleBuilder {
    module_token,
    unqualified_name,
    open_curly_token,
    statements,
    close_curly_token,
    semicolon_token: None,
  }
}
pub struct SdlModuleBuilder {
  module_token: SyntaxToken,
  unqualified_name: UnqualifiedName,
  open_curly_token: SyntaxToken,
  statements: SdlSchemaStatments,
  close_curly_token: SyntaxToken,
  semicolon_token: Option<SyntaxToken>,
}
impl SdlModuleBuilder {
  pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
    self.semicolon_token = Some(semicolon_token);
    self
  }

  pub fn build(self) -> SdlModule {
    SdlModule::unwrap_cast(SyntaxNode::new_detached(
      EqlSyntaxKind::SDL_MODULE,
      [
        Some(SyntaxElement::Token(self.module_token)),
        Some(SyntaxElement::Node(self.unqualified_name.into_syntax())),
        Some(SyntaxElement::Token(self.open_curly_token)),
        Some(SyntaxElement::Node(self.statements.into_syntax())),
        Some(SyntaxElement::Token(self.close_curly_token)),
        self
          .semicolon_token
          .map(|token| SyntaxElement::Token(token)),
      ],
    ))
  }
}
pub fn sdl_object_block(
  open_curly_token: SyntaxToken,
  annotations: SdlAnnotationList,
  properties: SdlPropertyList,
  links: SdlLinkList,
  constraints: SdlConstraintList,
  indexes: SdlIndexList,
  close_curly_token: SyntaxToken,
) -> SdlObjectBlockBuilder {
  SdlObjectBlockBuilder {
    open_curly_token,
    annotations,
    properties,
    links,
    constraints,
    indexes,
    close_curly_token,
    semicolon_token: None,
  }
}
pub struct SdlObjectBlockBuilder {
  open_curly_token: SyntaxToken,
  annotations: SdlAnnotationList,
  properties: SdlPropertyList,
  links: SdlLinkList,
  constraints: SdlConstraintList,
  indexes: SdlIndexList,
  close_curly_token: SyntaxToken,
  semicolon_token: Option<SyntaxToken>,
}
impl SdlObjectBlockBuilder {
  pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
    self.semicolon_token = Some(semicolon_token);
    self
  }

  pub fn build(self) -> SdlObjectBlock {
    SdlObjectBlock::unwrap_cast(SyntaxNode::new_detached(
      EqlSyntaxKind::SDL_OBJECT_BLOCK,
      [
        Some(SyntaxElement::Token(self.open_curly_token)),
        Some(SyntaxElement::Node(self.annotations.into_syntax())),
        Some(SyntaxElement::Node(self.properties.into_syntax())),
        Some(SyntaxElement::Node(self.links.into_syntax())),
        Some(SyntaxElement::Node(self.constraints.into_syntax())),
        Some(SyntaxElement::Node(self.indexes.into_syntax())),
        Some(SyntaxElement::Token(self.close_curly_token)),
        self
          .semicolon_token
          .map(|token| SyntaxElement::Token(token)),
      ],
    ))
  }
}
pub fn sdl_object_schema(
  type_token: SyntaxToken,
  name: Name,
  body: SdlObjectBody,
) -> SdlObjectSchemaBuilder {
  SdlObjectSchemaBuilder {
    type_token,
    name,
    body,
    abstract_token: None,
    extending: None,
  }
}
pub struct SdlObjectSchemaBuilder {
  type_token: SyntaxToken,
  name: Name,
  body: SdlObjectBody,
  abstract_token: Option<SyntaxToken>,
  extending: Option<SdlExtending>,
}
impl SdlObjectSchemaBuilder {
  pub fn with_abstract_token(mut self, abstract_token: SyntaxToken) -> Self {
    self.abstract_token = Some(abstract_token);
    self
  }

  pub fn with_extending(mut self, extending: SdlExtending) -> Self {
    self.extending = Some(extending);
    self
  }

  pub fn build(self) -> SdlObjectSchema {
    SdlObjectSchema::unwrap_cast(SyntaxNode::new_detached(
      EqlSyntaxKind::SDL_OBJECT_SCHEMA,
      [
        self.abstract_token.map(|token| SyntaxElement::Token(token)),
        Some(SyntaxElement::Token(self.type_token)),
        Some(SyntaxElement::Node(self.name.into_syntax())),
        self
          .extending
          .map(|token| SyntaxElement::Node(token.into_syntax())),
        Some(SyntaxElement::Node(self.body.into_syntax())),
      ],
    ))
  }
}
pub fn sdl_property(sdl_unknown: SdlUnknown) -> SdlProperty {
  SdlProperty::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::SDL_PROPERTY,
    [Some(SyntaxElement::Node(sdl_unknown.into_syntax()))],
  ))
}
pub fn sdl_scalar_block(
  open_curly_token: SyntaxToken,
  annotations: SdlAnnotationList,
  constraints: SdlConstraintList,
  close_curly_token: SyntaxToken,
) -> SdlScalarBlockBuilder {
  SdlScalarBlockBuilder {
    open_curly_token,
    annotations,
    constraints,
    close_curly_token,
    semicolon_token: None,
  }
}
pub struct SdlScalarBlockBuilder {
  open_curly_token: SyntaxToken,
  annotations: SdlAnnotationList,
  constraints: SdlConstraintList,
  close_curly_token: SyntaxToken,
  semicolon_token: Option<SyntaxToken>,
}
impl SdlScalarBlockBuilder {
  pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
    self.semicolon_token = Some(semicolon_token);
    self
  }

  pub fn build(self) -> SdlScalarBlock {
    SdlScalarBlock::unwrap_cast(SyntaxNode::new_detached(
      EqlSyntaxKind::SDL_SCALAR_BLOCK,
      [
        Some(SyntaxElement::Token(self.open_curly_token)),
        Some(SyntaxElement::Node(self.annotations.into_syntax())),
        Some(SyntaxElement::Node(self.constraints.into_syntax())),
        Some(SyntaxElement::Token(self.close_curly_token)),
        self
          .semicolon_token
          .map(|token| SyntaxElement::Token(token)),
      ],
    ))
  }
}
pub fn sdl_scalar_extending(
  extending_token: SyntaxToken,
  extends: SdlExtendingNames,
  extends: SdlEnumDeclaration,
) -> SdlScalarExtending {
  SdlScalarExtending::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::SDL_SCALAR_EXTENDING,
    [
      Some(SyntaxElement::Token(extending_token)),
      Some(SyntaxElement::Node(extends.into_syntax())),
      Some(SyntaxElement::Node(extends.into_syntax())),
    ],
  ))
}
pub fn sdl_scalar_schema(
  scalar_token: SyntaxToken,
  type_token: SyntaxToken,
  name: Name,
  body: SdlScalarBody,
) -> SdlScalarSchemaBuilder {
  SdlScalarSchemaBuilder {
    scalar_token,
    type_token,
    name,
    body,
    abstract_token: None,
    extending: None,
  }
}
pub struct SdlScalarSchemaBuilder {
  scalar_token: SyntaxToken,
  type_token: SyntaxToken,
  name: Name,
  body: SdlScalarBody,
  abstract_token: Option<SyntaxToken>,
  extending: Option<SdlScalarExtending>,
}
impl SdlScalarSchemaBuilder {
  pub fn with_abstract_token(mut self, abstract_token: SyntaxToken) -> Self {
    self.abstract_token = Some(abstract_token);
    self
  }

  pub fn with_extending(mut self, extending: SdlScalarExtending) -> Self {
    self.extending = Some(extending);
    self
  }

  pub fn build(self) -> SdlScalarSchema {
    SdlScalarSchema::unwrap_cast(SyntaxNode::new_detached(
      EqlSyntaxKind::SDL_SCALAR_SCHEMA,
      [
        self.abstract_token.map(|token| SyntaxElement::Token(token)),
        Some(SyntaxElement::Token(self.scalar_token)),
        Some(SyntaxElement::Token(self.type_token)),
        Some(SyntaxElement::Node(self.name.into_syntax())),
        self
          .extending
          .map(|token| SyntaxElement::Node(token.into_syntax())),
        Some(SyntaxElement::Node(self.body.into_syntax())),
      ],
    ))
  }
}
pub fn sdl_schema_constrain_param(
  name: UnqualifiedName,
  name_token: SyntaxToken,
  param_type: TypeExpression,
) -> SdlSchemaConstrainParam {
  SdlSchemaConstrainParam::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::SDL_SCHEMA_CONSTRAIN_PARAM,
    [
      Some(SyntaxElement::Node(name.into_syntax())),
      Some(SyntaxElement::Token(name_token)),
      Some(SyntaxElement::Node(param_type.into_syntax())),
    ],
  ))
}
pub fn sdl_schema_constraint(
  abstract_token: SyntaxToken,
  constraint_token: SyntaxToken,
  name: Name,
  body: SdlSchemaConstraintBody,
) -> SdlSchemaConstraintBuilder {
  SdlSchemaConstraintBuilder {
    abstract_token,
    constraint_token,
    name,
    body,
    params: None,
    subject: None,
    extending: None,
  }
}
pub struct SdlSchemaConstraintBuilder {
  abstract_token: SyntaxToken,
  constraint_token: SyntaxToken,
  name: Name,
  body: SdlSchemaConstraintBody,
  params: Option<SdlSchemaConstraintArgs>,
  subject: Option<SdlConstraintSubjectExpression>,
  extending: Option<SdlExtending>,
}
impl SdlSchemaConstraintBuilder {
  pub fn with_params(mut self, params: SdlSchemaConstraintArgs) -> Self {
    self.params = Some(params);
    self
  }

  pub fn with_subject(mut self, subject: SdlConstraintSubjectExpression) -> Self {
    self.subject = Some(subject);
    self
  }

  pub fn with_extending(mut self, extending: SdlExtending) -> Self {
    self.extending = Some(extending);
    self
  }

  pub fn build(self) -> SdlSchemaConstraint {
    SdlSchemaConstraint::unwrap_cast(SyntaxNode::new_detached(
      EqlSyntaxKind::SDL_SCHEMA_CONSTRAINT,
      [
        Some(SyntaxElement::Token(self.abstract_token)),
        Some(SyntaxElement::Token(self.constraint_token)),
        Some(SyntaxElement::Node(self.name.into_syntax())),
        self
          .params
          .map(|token| SyntaxElement::Node(token.into_syntax())),
        self
          .subject
          .map(|token| SyntaxElement::Node(token.into_syntax())),
        self
          .extending
          .map(|token| SyntaxElement::Node(token.into_syntax())),
        Some(SyntaxElement::Node(self.body.into_syntax())),
      ],
    ))
  }
}
pub fn sdl_schema_constraint_args(
  open_paren_token: SyntaxToken,
  params: SdlSchemaConstrainParamList,
  close_paren_token: SyntaxToken,
) -> SdlSchemaConstraintArgs {
  SdlSchemaConstraintArgs::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::SDL_SCHEMA_CONSTRAINT_ARGS,
    [
      Some(SyntaxElement::Token(open_paren_token)),
      Some(SyntaxElement::Node(params.into_syntax())),
      Some(SyntaxElement::Token(close_paren_token)),
    ],
  ))
}
pub fn sdl_schema_constraint_block(
  open_curly_token: SyntaxToken,
  using: Expression,
  annotations: SdlAnnotationList,
  close_curly_token: SyntaxToken,
) -> SdlSchemaConstraintBlockBuilder {
  SdlSchemaConstraintBlockBuilder {
    open_curly_token,
    using,
    annotations,
    close_curly_token,
    error_message: None,
    semicolon_token: None,
  }
}
pub struct SdlSchemaConstraintBlockBuilder {
  open_curly_token: SyntaxToken,
  using: Expression,
  annotations: SdlAnnotationList,
  close_curly_token: SyntaxToken,
  error_message: Option<SdlConstraintErrorMessage>,
  semicolon_token: Option<SyntaxToken>,
}
impl SdlSchemaConstraintBlockBuilder {
  pub fn with_error_message(mut self, error_message: SdlConstraintErrorMessage) -> Self {
    self.error_message = Some(error_message);
    self
  }

  pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
    self.semicolon_token = Some(semicolon_token);
    self
  }

  pub fn build(self) -> SdlSchemaConstraintBlock {
    SdlSchemaConstraintBlock::unwrap_cast(SyntaxNode::new_detached(
      EqlSyntaxKind::SDL_SCHEMA_CONSTRAINT_BLOCK,
      [
        Some(SyntaxElement::Token(self.open_curly_token)),
        Some(SyntaxElement::Node(self.using.into_syntax())),
        Some(SyntaxElement::Node(self.annotations.into_syntax())),
        self
          .error_message
          .map(|token| SyntaxElement::Node(token.into_syntax())),
        Some(SyntaxElement::Token(self.close_curly_token)),
        self
          .semicolon_token
          .map(|token| SyntaxElement::Token(token)),
      ],
    ))
  }
}
pub fn sequence_type(sequence_token: SyntaxToken) -> SequenceType {
  SequenceType::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::SEQUENCE_TYPE,
    [Some(SyntaxElement::Token(sequence_token))],
  ))
}
pub fn string_type(str_token: SyntaxToken) -> StringType {
  StringType::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::STRING_TYPE,
    [Some(SyntaxElement::Token(str_token))],
  ))
}
pub fn tuple_type(
  tuple_token: SyntaxToken,
  less_token: SyntaxToken,
  members: TupleTypeMembers,
  greater_token: SyntaxToken,
) -> TupleType {
  TupleType::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::TUPLE_TYPE,
    [
      Some(SyntaxElement::Token(tuple_token)),
      Some(SyntaxElement::Token(less_token)),
      Some(SyntaxElement::Node(members.into_syntax())),
      Some(SyntaxElement::Token(greater_token)),
    ],
  ))
}
pub fn type_cast_expression(
  less_token: SyntaxToken,
  ty: TypeExpression,
  greater_token: SyntaxToken,
  target: TypeCastTarget,
) -> TypeCastExpression {
  TypeCastExpression::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::TYPE_CAST_EXPRESSION,
    [
      Some(SyntaxElement::Token(less_token)),
      Some(SyntaxElement::Node(ty.into_syntax())),
      Some(SyntaxElement::Token(greater_token)),
      Some(SyntaxElement::Node(target.into_syntax())),
    ],
  ))
}
pub fn unqualified_name(ident_token: SyntaxToken) -> UnqualifiedName {
  UnqualifiedName::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::UNQUALIFIED_NAME,
    [Some(SyntaxElement::Token(ident_token))],
  ))
}
pub fn uuid_type(uuid_token: SyntaxToken) -> UuidType {
  UuidType::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::UUID_TYPE,
    [Some(SyntaxElement::Token(uuid_token))],
  ))
}
pub fn sdl_annotation_list<I>(items: I) -> SdlAnnotationList
where
  I: IntoIterator<Item = SdlAnnotation>,
  I::IntoIter: ExactSizeIterator,
{
  SdlAnnotationList::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::SDL_ANNOTATION_LIST,
    items
      .into_iter()
      .map(|item| Some(item.into_syntax().into())),
  ))
}
pub fn sdl_constraint_arg_list<I>(items: I) -> SdlConstraintArgList
where
  I: IntoIterator<Item = SdlConstraintArg>,
  I::IntoIter: ExactSizeIterator,
{
  SdlConstraintArgList::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::SDL_CONSTRAINT_ARG_LIST,
    items
      .into_iter()
      .map(|item| Some(item.into_syntax().into())),
  ))
}
pub fn sdl_constraint_list<I>(items: I) -> SdlConstraintList
where
  I: IntoIterator<Item = SdlConstraint>,
  I::IntoIter: ExactSizeIterator,
{
  SdlConstraintList::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::SDL_CONSTRAINT_LIST,
    items
      .into_iter()
      .map(|item| Some(item.into_syntax().into())),
  ))
}
pub fn sdl_enum_declaration_members<I, S>(items: I, separators: S) -> SdlEnumDeclarationMembers
where
  I: IntoIterator<Item = UnqualifiedName>,
  I::IntoIter: ExactSizeIterator,
  S: IntoIterator<Item = EqlSyntaxToken>,
  S::IntoIter: ExactSizeIterator,
{
  let mut items = items.into_iter();
  let mut separators = separators.into_iter();
  let length = items.len() + separators.len();
  SdlEnumDeclarationMembers::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::SDL_ENUM_DECLARATION_MEMBERS,
    (0..length).map(|index| {
      if index % 2 == 0 {
        Some(items.next()?.into_syntax().into())
      } else {
        Some(separators.next()?.into())
      }
    }),
  ))
}
pub fn sdl_extending_names<I, S>(items: I, separators: S) -> SdlExtendingNames
where
  I: IntoIterator<Item = Name>,
  I::IntoIter: ExactSizeIterator,
  S: IntoIterator<Item = EqlSyntaxToken>,
  S::IntoIter: ExactSizeIterator,
{
  let mut items = items.into_iter();
  let mut separators = separators.into_iter();
  let length = items.len() + separators.len();
  SdlExtendingNames::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::SDL_EXTENDING_NAMES,
    (0..length).map(|index| {
      if index % 2 == 0 {
        Some(items.next()?.into_syntax().into())
      } else {
        Some(separators.next()?.into())
      }
    }),
  ))
}
pub fn sdl_index_list<I>(items: I) -> SdlIndexList
where
  I: IntoIterator<Item = SdlIndex>,
  I::IntoIter: ExactSizeIterator,
{
  SdlIndexList::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::SDL_INDEX_LIST,
    items
      .into_iter()
      .map(|item| Some(item.into_syntax().into())),
  ))
}
pub fn sdl_link_list<I>(items: I) -> SdlLinkList
where
  I: IntoIterator<Item = SdlLink>,
  I::IntoIter: ExactSizeIterator,
{
  SdlLinkList::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::SDL_LINK_LIST,
    items
      .into_iter()
      .map(|item| Some(item.into_syntax().into())),
  ))
}
pub fn sdl_property_list<I>(items: I) -> SdlPropertyList
where
  I: IntoIterator<Item = SdlProperty>,
  I::IntoIter: ExactSizeIterator,
{
  SdlPropertyList::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::SDL_PROPERTY_LIST,
    items
      .into_iter()
      .map(|item| Some(item.into_syntax().into())),
  ))
}
pub fn sdl_schema_constrain_param_list<I>(items: I) -> SdlSchemaConstrainParamList
where
  I: IntoIterator<Item = SdlSchemaConstrainParam>,
  I::IntoIter: ExactSizeIterator,
{
  SdlSchemaConstrainParamList::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::SDL_SCHEMA_CONSTRAIN_PARAM_LIST,
    items
      .into_iter()
      .map(|item| Some(item.into_syntax().into())),
  ))
}
pub fn sdl_schema_statments<I>(items: I) -> SdlSchemaStatments
where
  I: IntoIterator<Item = SdlSchema>,
  I::IntoIter: ExactSizeIterator,
{
  SdlSchemaStatments::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::SDL_SCHEMA_STATMENTS,
    items
      .into_iter()
      .map(|item| Some(item.into_syntax().into())),
  ))
}
pub fn tuple_type_members<I, S>(items: I, separators: S) -> TupleTypeMembers
where
  I: IntoIterator<Item = TypeExpression>,
  I::IntoIter: ExactSizeIterator,
  S: IntoIterator<Item = EqlSyntaxToken>,
  S::IntoIter: ExactSizeIterator,
{
  let mut items = items.into_iter();
  let mut separators = separators.into_iter();
  let length = items.len() + separators.len();
  TupleTypeMembers::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::TUPLE_TYPE_MEMBERS,
    (0..length).map(|index| {
      if index % 2 == 0 {
        Some(items.next()?.into_syntax().into())
      } else {
        Some(separators.next()?.into())
      }
    }),
  ))
}
pub fn sdl_unknown<I>(slots: I) -> SdlUnknown
where
  I: IntoIterator<Item = Option<SyntaxElement>>,
  I::IntoIter: ExactSizeIterator,
{
  SdlUnknown::unwrap_cast(SyntaxNode::new_detached(EqlSyntaxKind::SDL_UNKNOWN, slots))
}
pub fn sdl_unknown_expression<I>(slots: I) -> SdlUnknownExpression
where
  I: IntoIterator<Item = Option<SyntaxElement>>,
  I::IntoIter: ExactSizeIterator,
{
  SdlUnknownExpression::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::SDL_UNKNOWN_EXPRESSION,
    slots,
  ))
}
pub fn sdl_unknown_schema<I>(slots: I) -> SdlUnknownSchema
where
  I: IntoIterator<Item = Option<SyntaxElement>>,
  I::IntoIter: ExactSizeIterator,
{
  SdlUnknownSchema::unwrap_cast(SyntaxNode::new_detached(
    EqlSyntaxKind::SDL_UNKNOWN_SCHEMA,
    slots,
  ))
}
pub fn unknown<I>(slots: I) -> Unknown
where
  I: IntoIterator<Item = Option<SyntaxElement>>,
  I::IntoIter: ExactSizeIterator,
{
  Unknown::unwrap_cast(SyntaxNode::new_detached(EqlSyntaxKind::UNKNOWN, slots))
}
