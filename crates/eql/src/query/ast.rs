use crate::shared::ast::Text;

#[derive(Debug, Clone, PartialEq)]
pub struct SessionSetAliasDeclaration<'a, T: Text<'a>> {
  module: T::Value,
  alias: Option<T::Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SessionResetAliasDeclaration<'a, T: Text<'a>> {
  module: T::Value,
  alias: T::Value,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SessionResetModule;

#[derive(Debug, Clone, PartialEq)]
pub struct SessionResetAllAliases;
