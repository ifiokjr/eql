use crate::shared::ast::ObjectReference;
use crate::shared::ast::SchemaObject;
use crate::shared::ast::Text;

#[derive(Debug, Clone, PartialEq)]
pub struct ModuleDeclaration<'a, T: Text<'a>> {
  name: ObjectReference<'a, T>,
  declarations: Vec<Declaration<'a, T>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration<'a, T: Text<'a>> {}

#[derive(Debug, Clone, PartialEq)]
pub struct Schema<'a, T: Text<'a>> {
  declarations: Vec<()>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamedModuleDeclaration<'a, T: Text<'a>> {
  module: ModuleDeclaration<'a, T>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NamedDDL<'a, T: Text<'a>> {
  CreateObject(CreateObject<'a, T>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateObject<'a, T: Text<'a>> {
  name: ObjectReference<'a, T>,
  object_class: SchemaObject,
  abstract_: bool,
  sdl_alter_if_exists: bool,
  create_if_not_exists: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AlterObject<'a, T: Text<'a>> {
  name: ObjectReference<'a, T>,
  object_class: SchemaObject,
}
