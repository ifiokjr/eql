// Generated file, do not edit by hand, see `xtask/src/codegen`

#[doc = r" Reconstruct an AstNode from a SyntaxNode"]
#[doc = r""]
#[doc = r" This macros performs a match over the [kind](rome_rowan::SyntaxNode::kind)"]
#[doc = r" of the provided [rome_rowan::SyntaxNode] and constructs the appropriate"]
#[doc = r" AstNode type for it, then execute the provided expression over it."]
#[doc = r""]
#[doc = r" # Examples"]
#[doc = r""]
#[doc = r" ```ignore"]
#[doc = r" map_syntax_node!(syntax_node, node => node.format())"]
#[doc = r" ```"]
#[macro_export]
macro_rules! map_syntax_node {
  ($node:expr, $pattern:pat => $body:expr) => {
    match $node {
      node => {
        match $crate::EqlSyntaxNode::kind(&node) {
          $crate::EqlSyntaxKind::ARRAY_TYPE => {
            let $pattern = unsafe { $crate::ArrayType::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::BARE_BYTES_LITERAL_EXPRESSION => {
            let $pattern = unsafe { $crate::BareBytesLiteralExpression::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::BARE_STRING_LITERAL_EXPRESSION => {
            let $pattern = unsafe { $crate::BareStringLiteralExpression::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::BIG_INT_LITERAL_EXPRESSION => {
            let $pattern = unsafe { $crate::BigIntLiteralExpression::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::BIG_INT_TYPE => {
            let $pattern = unsafe { $crate::BigIntType::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::BOOLEAN_LITERAL_EXPRESSION => {
            let $pattern = unsafe { $crate::BooleanLiteralExpression::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::BOOLEAN_TYPE => {
            let $pattern = unsafe { $crate::BooleanType::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::BYTES_TYPE => {
            let $pattern = unsafe { $crate::BytesType::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::DATE_TIME_TYPE => {
            let $pattern = unsafe { $crate::DateTimeType::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::DECIMAL_LITERAL_EXPRESSION => {
            let $pattern = unsafe { $crate::DecimalLiteralExpression::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::DECIMAL_TYPE => {
            let $pattern = unsafe { $crate::DecimalType::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::DOT_REFERENCE_NAME => {
            let $pattern = unsafe { $crate::DotReferenceName::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::DURATION_TYPE => {
            let $pattern = unsafe { $crate::DurationType::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::EMPTY_STATEMENT => {
            let $pattern = unsafe { $crate::EmptyStatement::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::EXPRESSION => {
            let $pattern = unsafe { $crate::Expression::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::FLOAT_LITERAL_EXPRESSION => {
            let $pattern = unsafe { $crate::FloatLiteralExpression::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::FLOAT_SIXTY_FOUR_TYPE => {
            let $pattern = unsafe { $crate::FloatSixtyFourType::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::FLOAT_THIRTY_TWO_TYPE => {
            let $pattern = unsafe { $crate::FloatThirtyTwoType::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::INT_LITERAL_EXPRESSION => {
            let $pattern = unsafe { $crate::IntLiteralExpression::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::INT_SIXTEEN_TYPE => {
            let $pattern = unsafe { $crate::IntSixteenType::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::INT_SIXTY_FOUR_TYPE => {
            let $pattern = unsafe { $crate::IntSixtyFourType::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::INT_THIRTY_TWO_TYPE => {
            let $pattern = unsafe { $crate::IntThirtyTwoType::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::JSON_TYPE => {
            let $pattern = unsafe { $crate::JsonType::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::PARAMETER_NAME => {
            let $pattern = unsafe { $crate::ParameterName::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::QUALIFIED_NAME => {
            let $pattern = unsafe { $crate::QualifiedName::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::RANGE_TYPE => {
            let $pattern = unsafe { $crate::RangeType::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::RAW_BYTES_LITERAL_EXPRESSION => {
            let $pattern = unsafe { $crate::RawBytesLiteralExpression::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::RAW_STRING_LITERAL_EXPRESSION => {
            let $pattern = unsafe { $crate::RawStringLiteralExpression::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_ANNOTATION => {
            let $pattern = unsafe { $crate::SdlAnnotation::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_ANNOTATION_SCHEMA => {
            let $pattern = unsafe { $crate::SdlAnnotationSchema::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_ANNOTATION_SCHEMA_BLOCK => {
            let $pattern = unsafe { $crate::SdlAnnotationSchemaBlock::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_CONSTRAINT => {
            let $pattern = unsafe { $crate::SdlConstraint::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_CONSTRAINT_ARG => {
            let $pattern = unsafe { $crate::SdlConstraintArg::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_CONSTRAINT_ARGS => {
            let $pattern = unsafe { $crate::SdlConstraintArgs::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_CONSTRAINT_BLOCK => {
            let $pattern = unsafe { $crate::SdlConstraintBlock::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_CONSTRAINT_ERROR_MESSAGE => {
            let $pattern = unsafe { $crate::SdlConstraintErrorMessage::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_CONSTRAINT_EXCEPT => {
            let $pattern = unsafe { $crate::SdlConstraintExcept::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_CONSTRAINT_SUBJECT_EXPRESSION => {
            let $pattern = unsafe { $crate::SdlConstraintSubjectExpression::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_ENUM_DECLARATION => {
            let $pattern = unsafe { $crate::SdlEnumDeclaration::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_EXTENDING => {
            let $pattern = unsafe { $crate::SdlExtending::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_INDEX => {
            let $pattern = unsafe { $crate::SdlIndex::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_LINK => {
            let $pattern = unsafe { $crate::SdlLink::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_MODULE => {
            let $pattern = unsafe { $crate::SdlModule::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_OBJECT_BLOCK => {
            let $pattern = unsafe { $crate::SdlObjectBlock::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_OBJECT_SCHEMA => {
            let $pattern = unsafe { $crate::SdlObjectSchema::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_PROPERTY => {
            let $pattern = unsafe { $crate::SdlProperty::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_SCALAR_BLOCK => {
            let $pattern = unsafe { $crate::SdlScalarBlock::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_SCALAR_EXTENDING_ENUM => {
            let $pattern = unsafe { $crate::SdlScalarExtendingEnum::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_SCALAR_EXTENDING_TYPE => {
            let $pattern = unsafe { $crate::SdlScalarExtendingType::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_SCALAR_SCHEMA => {
            let $pattern = unsafe { $crate::SdlScalarSchema::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_SCHEMA_CONSTRAIN_PARAM => {
            let $pattern = unsafe { $crate::SdlSchemaConstrainParam::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_SCHEMA_CONSTRAINT => {
            let $pattern = unsafe { $crate::SdlSchemaConstraint::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_SCHEMA_CONSTRAINT_ARGS => {
            let $pattern = unsafe { $crate::SdlSchemaConstraintArgs::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_SCHEMA_CONSTRAINT_BLOCK => {
            let $pattern = unsafe { $crate::SdlSchemaConstraintBlock::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SEQUENCE_TYPE => {
            let $pattern = unsafe { $crate::SequenceType::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::STRING_TYPE => {
            let $pattern = unsafe { $crate::StringType::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::TUPLE_TYPE => {
            let $pattern = unsafe { $crate::TupleType::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::TYPE_CAST_EXPRESSION => {
            let $pattern = unsafe { $crate::TypeCastExpression::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::UNQUALIFIED_NAME => {
            let $pattern = unsafe { $crate::UnqualifiedName::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::UUID_TYPE => {
            let $pattern = unsafe { $crate::UuidType::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_UNKNOWN => {
            let $pattern = unsafe { $crate::SdlUnknown::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_UNKNOWN_EXPRESSION => {
            let $pattern = unsafe { $crate::SdlUnknownExpression::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_UNKNOWN_SCHEMA => {
            let $pattern = unsafe { $crate::SdlUnknownSchema::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::UNKNOWN => {
            let $pattern = unsafe { $crate::Unknown::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_ANNOTATION_LIST => {
            let $pattern = unsafe { $crate::SdlAnnotationList::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_CONSTRAINT_ARG_LIST => {
            let $pattern = unsafe { $crate::SdlConstraintArgList::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_CONSTRAINT_LIST => {
            let $pattern = unsafe { $crate::SdlConstraintList::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_ENUM_DECLARATION_MEMBERS => {
            let $pattern = unsafe { $crate::SdlEnumDeclarationMembers::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_EXTENDING_NAMES => {
            let $pattern = unsafe { $crate::SdlExtendingNames::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_INDEX_LIST => {
            let $pattern = unsafe { $crate::SdlIndexList::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_LINK_LIST => {
            let $pattern = unsafe { $crate::SdlLinkList::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_PROPERTY_LIST => {
            let $pattern = unsafe { $crate::SdlPropertyList::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_SCHEMA_CONSTRAIN_PARAM_LIST => {
            let $pattern = unsafe { $crate::SdlSchemaConstrainParamList::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::SDL_SCHEMA_STATEMENTS => {
            let $pattern = unsafe { $crate::SdlSchemaStatements::new_unchecked(node) };
            $body
          }
          $crate::EqlSyntaxKind::TUPLE_TYPE_MEMBERS => {
            let $pattern = unsafe { $crate::TupleTypeMembers::new_unchecked(node) };
            $body
          }
          _ => unreachable!(),
        }
      }
    }
  };
}
pub(crate) use map_syntax_node;
