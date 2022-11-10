use std::fmt::Display;
use std::path::PathBuf;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum Language {
  #[default]
  EdgeQL,

  /// The schema definition language.
  Sdl,

  /// The query language for defining the schema (used in migrations).
  Ddl,
}

impl Language {
  pub const fn is_edgeql(&self) -> bool {
    matches!(self, Language::EdgeQL)
  }

  pub const fn is_sdl(&self) -> bool {
    matches!(self, Language::Sdl)
  }

  pub const fn is_ddl(&self) -> bool {
    matches!(self, Language::Ddl)
  }
}

/// Errors around the construct of the source type
#[derive(Debug)]
pub enum SourceTypeError {
  /// The path has no file name
  MissingFileName(PathBuf),
  /// The path has no file extension
  MissingFileExtension(PathBuf),
  /// The source type is unknown
  UnknownExtension(String),
}

impl std::error::Error for SourceTypeError {}

impl Display for SourceTypeError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      SourceTypeError::MissingFileName(path) => {
        write!(f, "The path {path:?} has no file name")
      }
      SourceTypeError::MissingFileExtension(path) => {
        write!(f, "The path {path:?} has no file extension")
      }
      SourceTypeError::UnknownExtension(extension) => {
        write!(f, "The parser can't parse the extension '{extension}' yet")
      }
    }
  }
}

/// It deduce the [SourceType] from the file name and its extension
pub fn compute_language(extension: &str) -> Result<Language, SourceTypeError> {
  let language = match extension {
    "edgeql" => Language::EdgeQL,
    "sdl" => Language::Sdl,
    "ddl" => Language::Sdl,
    _ => return Err(SourceTypeError::UnknownExtension(extension.into())),
  };

  Ok(language)
}
