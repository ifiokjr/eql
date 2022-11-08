//! Defines input for code generation process.

use std::collections::btree_map;
use std::collections::BTreeMap;

use quote::format_ident;

use super::nodes::NODES;

const LANGUAGE_PREFIXES: [&str; 3] = ["eql_", "sdl_", "ddl_"];

pub struct AstKinds<'a> {
  pub punct: &'a [(&'a str, &'a str)],
  pub literals: &'a [&'a str],
  pub tokens: &'a [&'a str],
  pub nodes: &'a [&'a str],
  pub keywords: &'a [&'a str],
  pub edgeql_keywords: &'a [&'a str],
  pub sdl_keywords: &'a [&'a str],
  pub ddl_keywords: &'a [&'a str],
  pub reserved_keywords: &'a [&'a str],
}

const PUNCTUATION: &'_ [(&'_ str, &'_ str)] = &[
  (":=", "ASSIGN"),
  ("+=", "ADD_ASSIGN"),
  ("-=", "SUB_ASSIGN"),
  ("->", "ARROW"),
  ("??", "COALESCE"),
  ("::", "NAMESPACE"),
  (".<", "BACKWARD_LINK"),
  ("//", "FLOOR_DIV"),
  ("++", "CONCAT"),
  (">=", "GREATER_EQUAL"),
  ("<=", "LESS_EQUAL"),
  ("!=", "NOT_EQUAL"),
  ("?=", "NOT_DISTINCT_FROM"),
  ("?!=", "DISTINCT_FROM"),
  (",", "COMMA"),
  ("(", "OPEN_PAREN"),
  (")", "CLOSE_PAREN"),
  ("[", "OPEN_SQUARE"),
  ("]", "CLOSE_SQUARE"),
  ("{", "OPEN_CURLY"),
  ("}", "CLOSE_CURLY"),
  (".", "DOT"),
  (";", "SEMICOLON"),
  (":", "COLON"),
  ("+", "ADD"),
  ("-", "SUBTRACT"),
  ("*", "MULTIPLY"),
  ("/", "DIVIDE"),
  ("%", "MODULO"),
  ("^", "POW"),
  ("<", "LESS"),
  (">", "GREATER"),
  ("=", "EQUAL"),
  ("&", "AMPERSAND"),
  ("|", "PIPE"),
  ("@", "AT"),
];

const LITERALS: &'_ [&'_ str] = &[
  "INT_LITERAL",
  "FLOAT_LITERAL",
  "BIG_INT_LITERAL",
  "DECIMAL_LITERAL",
  "STRING_LITERAL",
  "BYTE_LITERAL",
];

const TOKENS: &'_ [&'_ str] = &["ERROR", "IDENT", "NEWLINE", "WHITESPACE", "COMMENT"];
const SHARED_KEYWORDS: &'_ [&'_ str] = &[
  "__source__",
  "__subject__",
  "__type__",
  "__std__",
  "__edgedbsys__",
  "__edgedbtpl__",
  "abstract",
  "access",
  "alias",
  "all",
  "allow",
  "and",
  "anytuple",
  "anytype",
  "as",
  "by",
  "commit",
  "configure",
  "delete",
  "describe",
  "detached",
  "distinct",
  "else",
  "exists",
  "extending",
  "false",
  "filter",
  "for",
  "global",
  "group",
  "if",
  "ilike",
  "in",
  "index",
  "insert",
  "introspect",
  "is",
  "like",
  "limit",
  "module",
  "not",
  "offset",
  "on",
  "optional",
  "or",
  "policy",
  "select",
  "set",
  "single",
  "start",
  "std",
  "true",
  "typeof",
  "update",
  "variadic",
  "with",
];

const EDGEQL_KEYWORDS: &'_ [&'_ str] = &["asc", "desc", "union", "savepoint", "rollback"];
// const EDGEQL_KEYWORDS: &'_ [&'_ str] = const_join!(&'_ str, SHARED_KEYWORDS,
// EDGEQL_KEYWORDS_ONLY);

const SDL_KEYWORDS: &'_ [&'_ str] = &[
  "annotation",
  "link",
  "multi",
  "sequence",
  "using",
  "volatility",
];
// const SDL_KEYWORDS: &'_ [&'_ str] = const_join!(&'_ str, SHARED_KEYWORDS,
// SDL_KEYWORDS_ONLY);

const DDL_KEYWORDS: &'_ [&'_ str] = &[
  "after", "alter", "before", "create", "drop", "first", "last",
];
// const DDL_KEYWORDS: &'_ [&'_ str] = const_join!(&'_ str, SDL_KEYWORDS,
// DDL_KEYWORDS_ONLY);

const RESERVED_KEYWORDS: &'_ [&'_ str] = &[
  // Other (requires more understanding of the grammar)
  "abort",
  "applied",
  "assignment",
  "cardinality",
  "cast",
  "committed",
  "config",
  "conflict",
  "constraint",
  "cube",
  "current",
  "database",
  "ddl",
  "declare",
  "default",
  "deferrable",
  "deferred",
  "delegated",
  "deny",
  "empty",
  "except",
  "expression",
  "extension",
  "final",
  "from",
  "function",
  "future",
  "implicit",
  "infix",
  "inheritable",
  "instance",
  "into",
  "isolation",
  "json",
  "migration",
  "named",
  "object",
  "of",
  "only",
  "onto",
  "operator",
  "optionality",
  "order",
  "orphan",
  "overloaded",
  "owned",
  "package",
  "populate",
  "postfix",
  "prefix",
  "property",
  "proposed",
  "pseudo",
  "read",
  "reject",
  "release",
  "rename",
  "required",
  "reset",
  "restrict",
  "rewrite",
  "role",
  "roles",
  "rollup",
  "scalar",
  "schema",
  "sdl",
  "serializable",
  "session",
  "source",
  "superuser",
  "system",
  "target",
  "ternary",
  "text",
  "then",
  "to",
  "transaction",
  "type",
  "unless",
  "verbose",
  "version",
  "view",
  "write",
  // Future Keywords
  "analyze",
  "anyarray",
  "begin",
  "case",
  "check",
  "deallocate",
  "discard",
  "do",
  "end",
  "execute",
  "explain",
  "fetch",
  "get",
  "grant",
  "import",
  "listen",
  "load",
  "lock",
  "match",
  "move",
  "notify",
  "over",
  "prepare",
  "partition",
  "raise",
  "refresh",
  "reindex",
  "revoke",
  "when",
  "window",
  "never",
];

pub const KINDS: AstKinds<'_> = AstKinds {
  punct: PUNCTUATION,
  literals: LITERALS,
  tokens: TOKENS,
  nodes: NODES,
  keywords: SDL_KEYWORDS,
  edgeql_keywords: EDGEQL_KEYWORDS,
  sdl_keywords: SDL_KEYWORDS,
  ddl_keywords: DDL_KEYWORDS,
  reserved_keywords: RESERVED_KEYWORDS,
};

#[derive(Default, Debug)]
pub struct AstSrc {
  pub nodes: Vec<AstNodeSrc>,
  pub unions: Vec<AstEnumSrc>,
  pub lists: BTreeMap<String, AstListSrc>,
  pub unknowns: Vec<String>,
}

impl AstSrc {
  pub fn push_list(&mut self, name: &str, src: AstListSrc) {
    self.lists.insert(String::from(name), src);
  }

  pub fn lists(&self) -> btree_map::Iter<String, AstListSrc> {
    self.lists.iter()
  }

  pub fn is_list(&self, name: &str) -> bool {
    self.lists.contains_key(name)
  }

  /// Sorts all nodes, enums, etc. for a stable code gen result
  pub fn sort(&mut self) {
    // No need to sort lists, they're stored in a btree
    self.nodes.sort_unstable_by(|a, b| a.name.cmp(&b.name));
    self.unions.sort_unstable_by(|a, b| a.name.cmp(&b.name));
    self.unknowns.sort_unstable();

    for union in self.unions.iter_mut() {
      union.variants.sort_unstable();
    }
  }
}

#[derive(Debug)]
pub struct AstListSrc {
  pub element_name: String,
  pub separator: Option<AstListSeparatorConfiguration>,
}

#[derive(Debug)]
pub struct AstListSeparatorConfiguration {
  /// Name of the separator token
  pub separator_token: String,
  /// Whatever the list allows a trailing comma or not
  pub allow_trailing: bool,
}

#[derive(Debug)]
pub struct AstNodeSrc {
  pub documentation: Vec<String>,
  pub name: String,
  // pub traits: Vec<String>,
  pub fields: Vec<Field>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum TokenKind {
  Single(String),
  Many(Vec<String>),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Field {
  Token {
    name: String,
    kind: TokenKind,
    optional: bool,
  },
  Node {
    name: String,
    ty: String,
    optional: bool,
  },
}

impl Field {
  pub fn method_name(&self) -> proc_macro2::Ident {
    match self {
      Field::Token { name, .. } => {
        let name = match name.as_str() {
          ":=" => "assign",
          "+=" => "add_assign",
          "-=" => "sub_assign",
          "->" => "arrow",
          "??" => "coalesce",
          "::" => "namespace",
          ".<" => "backward_link",
          "//" => "floor_div",
          "++" => "concat",
          ">=" => "greater_equal",
          "<=" => "less_equal",
          "!=" => "not_equal",
          "?=" => "not_distinct_from",
          "?!=" => "distinct_from",
          "," => "comma",
          "'('" => "open_paren",
          "')'" => "close_paren",
          "'['" => "open_square",
          "']'" => "close_square",
          "'{'" => "open_curly",
          "'}'" => "close_curly",
          "." => "dot",
          ";" => "semicolon",
          ":" => "colon",
          "+" => "add",
          "-" => "subtract",
          "*" => "multiply",
          "/" => "divide",
          "%" => "modulo",
          "^" => "pow",
          "<" => "less",
          ">" => "greater",
          "=" => "equal",
          "&" => "ampersand",
          "|" => "pipe",
          "@" => "at",
          "$" => "dollar",
          _ => name,
        };
        println!("The name: {}_token", name);
        format_ident!("{}_token", name)
      }
      Field::Node { name, .. } => {
        let name = name;
        let (prefix, tail) = name.split_once('_').unwrap_or(("", name));
        let final_name = if LANGUAGE_PREFIXES.contains(&prefix) {
          tail
        } else {
          name.as_str()
        };

        // this check here is to avoid emitting methods called "type()",
        // where "type" is a reserved word
        if final_name == "type" {
          format_ident!("ty")
        } else {
          format_ident!("{}", final_name)
        }
      }
    }
  }

  #[allow(dead_code)]
  pub fn ty(&self) -> proc_macro2::Ident {
    match self {
      Field::Token { .. } => format_ident!("SyntaxToken"),
      Field::Node { ty, .. } => format_ident!("{}", ty),
    }
  }

  pub fn is_optional(&self) -> bool {
    match self {
      Field::Node { optional, .. } => *optional,
      Field::Token { optional, .. } => *optional,
    }
  }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Cardinality {
  Optional,
  Many,
}

#[derive(Debug, Clone)]
pub struct AstEnumSrc {
  pub documentation: Vec<String>,
  pub name: String,
  // pub traits: Vec<String>,
  pub variants: Vec<String>,
}
