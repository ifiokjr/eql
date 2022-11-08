use std::hash::Hash;

use indexmap::IndexMap;
use strum::Display;
use strum::EnumString;
use strum::IntoStaticStr;

/// Text abstracts over types that hold a string value.
/// It is used to make the AST generic over the string type.
pub trait Text<'a>: 'a {
  type Value: 'a
    + From<&'a str>
    + AsRef<str>
    + std::borrow::Borrow<str>
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + std::fmt::Debug
    + Clone
    + Hash;
}

impl<'a> Text<'a> for &'a str {
  type Value = Self;
}

impl<'a> Text<'a> for String {
  type Value = String;
}

impl<'a> Text<'a> for std::borrow::Cow<'a, str> {
  type Value = Self;
}

/// Abstract parent for all query expressions.
#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a, T: Text<'a>> {
  Placeholder(Placeholder<'a, T>),
  Anchor(Anchor),
}

/// An interpolation placeholder used in expression templates.
#[derive(Debug, Clone, PartialEq)]
pub struct Placeholder<'a, T: Text<'a>> {
  name: T::Value,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OffsetLimitMixin<'a, T: Text<'a>> {
  offset: Option<Expression<'a, T>>,
  limit: Option<Expression<'a, T>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OrderByMixin<'a, T: Text<'a>> {
  order_by: Option<Expression<'a, T>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FilterMixin<'a, T: Text<'a>> {
  where_filter: Option<Expression<'a, T>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OptionValue<'a, T: Text<'a>> {
  Flag(Flag<'a, T>),
}

/// An interpolation placeholder used in expression templates.
#[derive(Debug, Clone, PartialEq)]
pub struct Flag<'a, T: Text<'a>> {
  name: T::Value,
  value: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Options<'a, T: Text<'a>> {
  options: IndexMap<T::Value, OptionValue<'a, T>>,
}

#[derive(Debug, Clone, PartialEq, Display, EnumString, IntoStaticStr)]
pub enum AccessKind {
  Select,
  UpdateRead,
  UpdateWrite,
  Delete,
  Insert,
}

impl AccessKind {
  pub fn is_data_check(&self) -> bool {
    use self::AccessKind::*;
    matches!(self, Insert | UpdateWrite)
  }
}

#[derive(Debug, Clone, PartialEq, Display, EnumString, IntoStaticStr)]
pub enum SchemaObject {
  #[strum(serialize = "ACCESS_POLICY")]
  AccessPolicy,
  #[strum(serialize = "ALIAS")]
  Alias,
  #[strum(serialize = "ANNOTATION")]
  Annotation,
  #[strum(serialize = "ARRAY TYPE")]
  ArrayType,
  #[strum(serialize = "CAST")]
  Cast,
  #[strum(serialize = "CONSTRAINT")]
  Constraint,
  #[strum(serialize = "DATABASE")]
  Database,
  #[strum(serialize = "EXTENSION")]
  Extension,
  #[strum(serialize = "EXTENSION PACKAGE")]
  ExtensionPackage,
  #[strum(serialize = "FUTURE")]
  Future,
  #[strum(serialize = "FUNCTION")]
  Function,
  #[strum(serialize = "GLOBAL")]
  Global,
  #[strum(serialize = "INDEX")]
  Index,
  #[strum(serialize = "LINK")]
  Link,
  #[strum(serialize = "MIGRATION")]
  Migration,
  #[strum(serialize = "MODULE")]
  Module,
  #[strum(serialize = "OPERATOR")]
  Operator,
  #[strum(serialize = "PARAMETER")]
  Parameter,
  #[strum(serialize = "PROPERTY")]
  Property,
  #[strum(serialize = "PSEUDO TYPE")]
  PseudoType,
  #[strum(serialize = "RANGE TYPE")]
  RangeType,
  #[strum(serialize = "ROLE")]
  Role,
  #[strum(serialize = "SCALAR TYPE")]
  ScalarType,
  #[strum(serialize = "TUPLE TYPE")]
  TupleType,
  #[strum(serialize = "TYPE")]
  Type,
}

#[derive(Debug, Clone, PartialEq, Display, EnumString, IntoStaticStr)]
pub enum Anchor {
  #[strum(serialize = "__special__")]
  Special,
  #[strum(serialize = "__source__")]
  Source,
  #[strum(serialize = "__subject__")]
  Subject,
}

#[derive(Debug, Clone, PartialEq, Display, EnumString, IntoStaticStr)]
pub enum SortOrder {
  #[strum(serialize = "ASC")]
  Ascending,
  #[strum(serialize = "DESC")]
  Descending,
}

#[derive(Debug, Clone, PartialEq, Display, EnumString, IntoStaticStr)]
pub enum NonesOrder {
  #[strum(serialize = "first")]
  First,
  #[strum(serialize = "last")]
  Last,
}

#[derive(Debug, Clone, PartialEq, Display, EnumString, IntoStaticStr)]
pub enum CardinalityModifier {
  #[strum(serialize = "OPTIONAL")]
  Optional,
  #[strum(serialize = "REQUIRED")]
  Required,
}

#[derive(Debug, Clone, PartialEq, Display, EnumString, IntoStaticStr)]
pub enum DescribeGlobal {
  #[strum(serialize = "SCHEMA")]
  Schema,
  #[strum(serialize = "DATABASE CONFIG")]
  DatabaseConfig,
  #[strum(serialize = "INSTANCE CONFIG")]
  InstanceConfig,
  #[strum(serialize = "ROLES")]
  Roles,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubExpression<'a, T: Text<'a>> {
  expression: Expression<'a, T>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SortExpression<'a, T: Text<'a>> {
  path: Expression<'a, T>,
  direction: Option<SortOrder>,
  nones_order: Option<NonesOrder>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AliasedExpression<'a, T: Text<'a>> {
  expression: Expression<'a, T>,
  alias: T::Value,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OptionallyAliasedExpression<'a, T: Text<'a>> {
  expression: Expression<'a, T>,
  alias: Option<T::Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModuleAliasDeclaration<'a, T: Text<'a>> {
  module: T::Value,
  alias: Option<T::Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectReference<'a, T: Text<'a>> {
  name: T::Value,
  module: Option<T::Value>,
  itemclass: Option<SchemaObject>,
}
