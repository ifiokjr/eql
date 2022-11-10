// Generated file, do not edit by hand, see `xtask/src/codegen`

use eql_syntax::EqlSyntaxKind;
use eql_syntax::EqlSyntaxKind::*;
use eql_syntax::T;
use eql_syntax::*;
use rome_rowan::AstNode;
use rome_rowan::ParsedChildren;
use rome_rowan::RawNodeSlots;
use rome_rowan::RawSyntaxNode;
use rome_rowan::SyntaxFactory;
use rome_rowan::SyntaxKind;
#[derive(Debug)]
pub struct EqlSyntaxFactory;
impl SyntaxFactory for EqlSyntaxFactory {
  type Kind = EqlSyntaxKind;

  #[allow(unused_mut)]
  fn make_syntax(
    kind: Self::Kind,
    children: ParsedChildren<Self::Kind>,
  ) -> RawSyntaxNode<Self::Kind> {
    match kind {
      SDL_UNKNOWN | SDL_UNKNOWN_EXPRESSION | SDL_UNKNOWN_SCHEMA | UNKNOWN => {
        RawSyntaxNode::new(kind, children.into_iter().map(Some))
      }
      ARRAY_TYPE => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T![array] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T ! [<] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if PrimitiveType::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T ! [>] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(ARRAY_TYPE.to_unknown(), children.into_iter().map(Some));
        }
        slots.into_node(ARRAY_TYPE, children)
      }
      BARE_BYTES_LITERAL_EXPRESSION => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T![r] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T![b] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if BareStringLiteralExpression::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            BARE_BYTES_LITERAL_EXPRESSION.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(BARE_BYTES_LITERAL_EXPRESSION, children)
      }
      BARE_STRING_LITERAL_EXPRESSION => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == STRING_LITERAL {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            BARE_STRING_LITERAL_EXPRESSION.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(BARE_STRING_LITERAL_EXPRESSION, children)
      }
      BIG_INT_LITERAL_EXPRESSION => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == BIG_INT_LITERAL {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            BIG_INT_LITERAL_EXPRESSION.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(BIG_INT_LITERAL_EXPRESSION, children)
      }
      BIG_INT_TYPE => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T![bigint] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(BIG_INT_TYPE.to_unknown(), children.into_iter().map(Some));
        }
        slots.into_node(BIG_INT_TYPE, children)
      }
      BOOLEAN_LITERAL_EXPRESSION => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if matches!(element.kind(), T![true] | T![false]) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            BOOLEAN_LITERAL_EXPRESSION.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(BOOLEAN_LITERAL_EXPRESSION, children)
      }
      BOOLEAN_TYPE => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T![bool] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(BOOLEAN_TYPE.to_unknown(), children.into_iter().map(Some));
        }
        slots.into_node(BOOLEAN_TYPE, children)
      }
      BYTES_TYPE => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T![bytes] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(BYTES_TYPE.to_unknown(), children.into_iter().map(Some));
        }
        slots.into_node(BYTES_TYPE, children)
      }
      DATE_TIME_TYPE => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T![datetime] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(DATE_TIME_TYPE.to_unknown(), children.into_iter().map(Some));
        }
        slots.into_node(DATE_TIME_TYPE, children)
      }
      DECIMAL_LITERAL_EXPRESSION => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == DECIMAL_LITERAL {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            DECIMAL_LITERAL_EXPRESSION.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(DECIMAL_LITERAL_EXPRESSION, children)
      }
      DECIMAL_TYPE => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T![decimal] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(DECIMAL_TYPE.to_unknown(), children.into_iter().map(Some));
        }
        slots.into_node(DECIMAL_TYPE, children)
      }
      DOT_REFERENCE_NAME => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == IDENT {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T ! [.] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == IDENT {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            DOT_REFERENCE_NAME.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(DOT_REFERENCE_NAME, children)
      }
      DURATION_TYPE => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T![duration] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(DURATION_TYPE.to_unknown(), children.into_iter().map(Some));
        }
        slots.into_node(DURATION_TYPE, children)
      }
      EMPTY_STATEMENT => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T ! [;] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(EMPTY_STATEMENT.to_unknown(), children.into_iter().map(Some));
        }
        slots.into_node(EMPTY_STATEMENT, children)
      }
      EXPRESSION => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if Unknown::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(EXPRESSION.to_unknown(), children.into_iter().map(Some));
        }
        slots.into_node(EXPRESSION, children)
      }
      FLOAT_LITERAL_EXPRESSION => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == FLOAT_LITERAL {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            FLOAT_LITERAL_EXPRESSION.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(FLOAT_LITERAL_EXPRESSION, children)
      }
      FLOAT_SIXTY_FOUR_TYPE => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T![float64] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            FLOAT_SIXTY_FOUR_TYPE.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(FLOAT_SIXTY_FOUR_TYPE, children)
      }
      FLOAT_THIRTY_TWO_TYPE => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T![float32] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            FLOAT_THIRTY_TWO_TYPE.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(FLOAT_THIRTY_TWO_TYPE, children)
      }
      INT_LITERAL_EXPRESSION => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == INT_LITERAL {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            INT_LITERAL_EXPRESSION.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(INT_LITERAL_EXPRESSION, children)
      }
      INT_SIXTEEN_TYPE => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T![int16] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            INT_SIXTEEN_TYPE.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(INT_SIXTEEN_TYPE, children)
      }
      INT_SIXTY_FOUR_TYPE => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T![int64] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            INT_SIXTY_FOUR_TYPE.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(INT_SIXTY_FOUR_TYPE, children)
      }
      INT_THIRTY_TWO_TYPE => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T![int32] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            INT_THIRTY_TWO_TYPE.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(INT_THIRTY_TWO_TYPE, children)
      }
      JSON_TYPE => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T![json] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(JSON_TYPE.to_unknown(), children.into_iter().map(Some));
        }
        slots.into_node(JSON_TYPE, children)
      }
      PARAMETER_NAME => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T ! [$] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == IDENT {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(PARAMETER_NAME.to_unknown(), children.into_iter().map(Some));
        }
        slots.into_node(PARAMETER_NAME, children)
      }
      QUALIFIED_NAME => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == IDENT {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T ! [::] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == IDENT {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(QUALIFIED_NAME.to_unknown(), children.into_iter().map(Some));
        }
        slots.into_node(QUALIFIED_NAME, children)
      }
      RANGE_TYPE => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T![range] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T ! [<] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if RangeTypeMember::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T ! [>] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(RANGE_TYPE.to_unknown(), children.into_iter().map(Some));
        }
        slots.into_node(RANGE_TYPE, children)
      }
      RAW_BYTES_LITERAL_EXPRESSION => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T![b] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if RawStringLiteralExpression::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            RAW_BYTES_LITERAL_EXPRESSION.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(RAW_BYTES_LITERAL_EXPRESSION, children)
      }
      RAW_STRING_LITERAL_EXPRESSION => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T![r] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == STRING_LITERAL {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            RAW_STRING_LITERAL_EXPRESSION.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(RAW_STRING_LITERAL_EXPRESSION, children)
      }
      SDL_ANNOTATION => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<5usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T![annotation] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if UnqualifiedName::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T ! [:=] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if StringLiteralExpression::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if EmptyStatement::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(SDL_ANNOTATION.to_unknown(), children.into_iter().map(Some));
        }
        slots.into_node(SDL_ANNOTATION, children)
      }
      SDL_ANNOTATION_SCHEMA => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<5usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T![abstract] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T![inheritable] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T![annotation] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if Name::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if SdlAnnotationSchemaBody::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            SDL_ANNOTATION_SCHEMA.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(SDL_ANNOTATION_SCHEMA, children)
      }
      SDL_ANNOTATION_SCHEMA_BLOCK => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T!['{'] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if SdlAnnotationList::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T!['}'] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T ! [;] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            SDL_ANNOTATION_SCHEMA_BLOCK.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(SDL_ANNOTATION_SCHEMA_BLOCK, children)
      }
      SDL_CONSTRAINT => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<7usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T![delegated] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T![constraint] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if Name::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if SdlConstraintArgs::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if SdlConstraintSubjectExpression::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if SdlConstraintExcept::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if SdlConstraintBody::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(SDL_CONSTRAINT.to_unknown(), children.into_iter().map(Some));
        }
        slots.into_node(SDL_CONSTRAINT, children)
      }
      SDL_CONSTRAINT_ARG => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if UnqualifiedName::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T ! [:] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if Expression::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            SDL_CONSTRAINT_ARG.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(SDL_CONSTRAINT_ARG, children)
      }
      SDL_CONSTRAINT_ARGS => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T!['('] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if SdlConstraintArgList::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T![')'] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            SDL_CONSTRAINT_ARGS.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(SDL_CONSTRAINT_ARGS, children)
      }
      SDL_CONSTRAINT_BLOCK => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<5usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T!['{'] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if SdlAnnotationList::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if SdlConstraintErrorMessage::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T!['}'] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T ! [;] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            SDL_CONSTRAINT_BLOCK.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(SDL_CONSTRAINT_BLOCK, children)
      }
      SDL_CONSTRAINT_ERROR_MESSAGE => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T![errmessage] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T ! [:=] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if StringLiteralExpression::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if EmptyStatement::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            SDL_CONSTRAINT_ERROR_MESSAGE.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(SDL_CONSTRAINT_ERROR_MESSAGE, children)
      }
      SDL_CONSTRAINT_EXCEPT => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T![except] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if Expression::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            SDL_CONSTRAINT_EXCEPT.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(SDL_CONSTRAINT_EXCEPT, children)
      }
      SDL_CONSTRAINT_SUBJECT_EXPRESSION => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T![on] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T!['('] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if Expression::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T![')'] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            SDL_CONSTRAINT_SUBJECT_EXPRESSION.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(SDL_CONSTRAINT_SUBJECT_EXPRESSION, children)
      }
      SDL_ENUM_DECLARATION => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T![enum] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T ! [<] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if SdlEnumDeclarationMembers::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T ! [>] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            SDL_ENUM_DECLARATION.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(SDL_ENUM_DECLARATION, children)
      }
      SDL_EXTENDING => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T![extending] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if SdlExtendingNames::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(SDL_EXTENDING.to_unknown(), children.into_iter().map(Some));
        }
        slots.into_node(SDL_EXTENDING, children)
      }
      SDL_INDEX => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if SdlUnknown::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(SDL_INDEX.to_unknown(), children.into_iter().map(Some));
        }
        slots.into_node(SDL_INDEX, children)
      }
      SDL_LINK => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if SdlUnknown::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(SDL_LINK.to_unknown(), children.into_iter().map(Some));
        }
        slots.into_node(SDL_LINK, children)
      }
      SDL_MODULE => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<6usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T![module] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if UnqualifiedName::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T!['{'] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if SdlSchemaStatements::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T!['}'] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T ! [;] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(SDL_MODULE.to_unknown(), children.into_iter().map(Some));
        }
        slots.into_node(SDL_MODULE, children)
      }
      SDL_OBJECT_BLOCK => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<8usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T!['{'] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if SdlAnnotationList::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if SdlPropertyList::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if SdlLinkList::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if SdlConstraintList::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if SdlIndexList::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T!['}'] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T ! [;] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            SDL_OBJECT_BLOCK.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(SDL_OBJECT_BLOCK, children)
      }
      SDL_OBJECT_SCHEMA => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<5usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T![abstract] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T![type] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if Name::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if SdlExtending::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if SdlObjectBody::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            SDL_OBJECT_SCHEMA.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(SDL_OBJECT_SCHEMA, children)
      }
      SDL_PROPERTY => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if SdlUnknown::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(SDL_PROPERTY.to_unknown(), children.into_iter().map(Some));
        }
        slots.into_node(SDL_PROPERTY, children)
      }
      SDL_SCALAR_BLOCK => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<5usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T!['{'] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if SdlAnnotationList::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if SdlConstraintList::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T!['}'] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T ! [;] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            SDL_SCALAR_BLOCK.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(SDL_SCALAR_BLOCK, children)
      }
      SDL_SCALAR_EXTENDING_ENUM => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T![extending] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if SdlEnumDeclaration::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            SDL_SCALAR_EXTENDING_ENUM.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(SDL_SCALAR_EXTENDING_ENUM, children)
      }
      SDL_SCALAR_EXTENDING_TYPE => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T![extending] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if SdlExtendingNames::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            SDL_SCALAR_EXTENDING_TYPE.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(SDL_SCALAR_EXTENDING_TYPE, children)
      }
      SDL_SCALAR_SCHEMA => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<6usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T![abstract] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T![scalar] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T![type] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if Name::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if SdlScalarExtending::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if SdlScalarBody::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            SDL_SCALAR_SCHEMA.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(SDL_SCALAR_SCHEMA, children)
      }
      SDL_SCHEMA_CONSTRAIN_PARAM => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if UnqualifiedName::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T ! [:] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if TypeExpression::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            SDL_SCHEMA_CONSTRAIN_PARAM.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(SDL_SCHEMA_CONSTRAIN_PARAM, children)
      }
      SDL_SCHEMA_CONSTRAINT => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<7usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T![abstract] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T![constraint] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if Name::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if SdlSchemaConstraintArgs::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if SdlConstraintSubjectExpression::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if SdlExtending::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if SdlSchemaConstraintBody::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            SDL_SCHEMA_CONSTRAINT.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(SDL_SCHEMA_CONSTRAINT, children)
      }
      SDL_SCHEMA_CONSTRAINT_ARGS => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T!['('] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if SdlSchemaConstrainParamList::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T![')'] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            SDL_SCHEMA_CONSTRAINT_ARGS.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(SDL_SCHEMA_CONSTRAINT_ARGS, children)
      }
      SDL_SCHEMA_CONSTRAINT_BLOCK => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<6usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T!['{'] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if Expression::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if SdlAnnotationList::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if SdlConstraintErrorMessage::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T!['}'] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T ! [;] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            SDL_SCHEMA_CONSTRAINT_BLOCK.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(SDL_SCHEMA_CONSTRAINT_BLOCK, children)
      }
      SEQUENCE_TYPE => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T![sequence] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(SEQUENCE_TYPE.to_unknown(), children.into_iter().map(Some));
        }
        slots.into_node(SEQUENCE_TYPE, children)
      }
      STRING_TYPE => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T![str] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(STRING_TYPE.to_unknown(), children.into_iter().map(Some));
        }
        slots.into_node(STRING_TYPE, children)
      }
      TUPLE_TYPE => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T![tuple] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T ! [<] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if TupleTypeMembers::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T ! [>] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(TUPLE_TYPE.to_unknown(), children.into_iter().map(Some));
        }
        slots.into_node(TUPLE_TYPE, children)
      }
      TYPE_CAST_EXPRESSION => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T ! [<] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if TypeExpression::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if element.kind() == T ! [>] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if let Some(element) = &current_element {
          if TypeCastTarget::can_cast(element.kind()) {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            TYPE_CAST_EXPRESSION.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(TYPE_CAST_EXPRESSION, children)
      }
      UNQUALIFIED_NAME => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == IDENT {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(
            UNQUALIFIED_NAME.to_unknown(),
            children.into_iter().map(Some),
          );
        }
        slots.into_node(UNQUALIFIED_NAME, children)
      }
      UUID_TYPE => {
        let mut elements = (&children).into_iter();
        let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
        let mut current_element = elements.next();
        if let Some(element) = &current_element {
          if element.kind() == T![uuid] {
            slots.mark_present();
            current_element = elements.next();
          }
        }
        slots.next_slot();
        if current_element.is_some() {
          return RawSyntaxNode::new(UUID_TYPE.to_unknown(), children.into_iter().map(Some));
        }
        slots.into_node(UUID_TYPE, children)
      }
      SDL_ANNOTATION_LIST => Self::make_node_list_syntax(kind, children, SdlAnnotation::can_cast),
      SDL_CONSTRAINT_ARG_LIST => {
        Self::make_node_list_syntax(kind, children, SdlConstraintArg::can_cast)
      }
      SDL_CONSTRAINT_LIST => Self::make_node_list_syntax(kind, children, SdlConstraint::can_cast),
      SDL_ENUM_DECLARATION_MEMBERS => {
        Self::make_separated_list_syntax(kind, children, UnqualifiedName::can_cast, T ! [,], false)
      }
      SDL_EXTENDING_NAMES => {
        Self::make_separated_list_syntax(kind, children, Name::can_cast, T ! [,], false)
      }
      SDL_INDEX_LIST => Self::make_node_list_syntax(kind, children, SdlIndex::can_cast),
      SDL_LINK_LIST => Self::make_node_list_syntax(kind, children, SdlLink::can_cast),
      SDL_PROPERTY_LIST => Self::make_node_list_syntax(kind, children, SdlProperty::can_cast),
      SDL_SCHEMA_CONSTRAIN_PARAM_LIST => {
        Self::make_node_list_syntax(kind, children, SdlSchemaConstrainParam::can_cast)
      }
      SDL_SCHEMA_STATEMENTS => Self::make_node_list_syntax(kind, children, SdlSchema::can_cast),
      TUPLE_TYPE_MEMBERS => {
        Self::make_separated_list_syntax(kind, children, TypeExpression::can_cast, T ! [,], false)
      }
      _ => unreachable!("Is {:?} a token?", kind),
    }
  }
}
