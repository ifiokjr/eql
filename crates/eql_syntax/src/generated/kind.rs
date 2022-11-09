#![allow(clippy::all)]
#![allow(bad_style, missing_docs, unreachable_pub)]
#[doc = r" The kind of syntax node, e.g. `IDENT`, `FUNCTION_KW`, or `FOR_STMT`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum EqlSyntaxKind {
  #[doc(hidden)]
  TOMBSTONE,
  #[doc = r" Marks the end of the file. May have trivia attached."]
  EOF,
  ASSIGN,
  ADD_ASSIGN,
  SUB_ASSIGN,
  ARROW,
  COALESCE,
  NAMESPACE,
  BACKWARD_LINK,
  FLOOR_DIV,
  CONCAT,
  GREATER_EQUAL,
  LESS_EQUAL,
  NOT_EQUAL,
  NOT_DISTINCT_FROM,
  DISTINCT_FROM,
  COMMA,
  OPEN_PAREN,
  CLOSE_PAREN,
  OPEN_SQUARE,
  CLOSE_SQUARE,
  OPEN_CURLY,
  CLOSE_CURLY,
  DOT,
  SEMICOLON,
  COLON,
  ADD,
  SUBTRACT,
  MULTIPLY,
  DIVIDE,
  MODULO,
  POW,
  LESS,
  GREATER,
  EQUAL,
  AMPERSAND,
  PIPE,
  AT,
  ANNOTATION_KW,
  LINK_KW,
  MULTI_KW,
  SEQUENCE_KW,
  USING_KW,
  VOLATILITY_KW,
  RESERVED_ANNOTATION_KW,
  RESERVED_LINK_KW,
  RESERVED_MULTI_KW,
  RESERVED_SEQUENCE_KW,
  RESERVED_USING_KW,
  RESERVED_VOLATILITY_KW,
  EDGE_ANNOTATION_KW,
  EDGE_LINK_KW,
  EDGE_MULTI_KW,
  EDGE_SEQUENCE_KW,
  EDGE_USING_KW,
  EDGE_VOLATILITY_KW,
  SDL_ANNOTATION_KW,
  SDL_LINK_KW,
  SDL_MULTI_KW,
  SDL_SEQUENCE_KW,
  SDL_USING_KW,
  SDL_VOLATILITY_KW,
  DDL_ANNOTATION_KW,
  DDL_LINK_KW,
  DDL_MULTI_KW,
  DDL_SEQUENCE_KW,
  DDL_USING_KW,
  DDL_VOLATILITY_KW,
  INT_LITERAL,
  FLOAT_LITERAL,
  BIG_INT_LITERAL,
  DECIMAL_LITERAL,
  STRING_LITERAL,
  BYTE_LITERAL,
  ERROR,
  IDENT,
  NEWLINE,
  WHITESPACE,
  COMMENT,
  #[doc(hidden)]
  __LAST,
}
use self::EqlSyntaxKind::*;
impl EqlSyntaxKind {
  pub const fn is_punct(self) -> bool {
    match self {
      ASSIGN | ADD_ASSIGN | SUB_ASSIGN | ARROW | COALESCE | NAMESPACE | BACKWARD_LINK
      | FLOOR_DIV | CONCAT | GREATER_EQUAL | LESS_EQUAL | NOT_EQUAL | NOT_DISTINCT_FROM
      | DISTINCT_FROM | COMMA | OPEN_PAREN | CLOSE_PAREN | OPEN_SQUARE | CLOSE_SQUARE
      | OPEN_CURLY | CLOSE_CURLY | DOT | SEMICOLON | COLON | ADD | SUBTRACT | MULTIPLY | DIVIDE
      | MODULO | POW | LESS | GREATER | EQUAL | AMPERSAND | PIPE | AT => true,
      _ => false,
    }
  }

  pub const fn is_literal(self) -> bool {
    match self {
      INT_LITERAL | FLOAT_LITERAL | BIG_INT_LITERAL | DECIMAL_LITERAL | STRING_LITERAL
      | BYTE_LITERAL => true,
      _ => false,
    }
  }

  pub const fn is_list(self) -> bool {
    match self {
      _ => false,
    }
  }

  pub fn from_keyword(ident: &str) -> Option<EqlSyntaxKind> {
    let kw = match ident {
      "annotation" => ANNOTATION_KW,
      "link" => LINK_KW,
      "multi" => MULTI_KW,
      "sequence" => SEQUENCE_KW,
      "using" => USING_KW,
      "volatility" => VOLATILITY_KW,
      "abort" => RESERVED_ABORT_KW,
      "applied" => RESERVED_APPLIED_KW,
      "assignment" => RESERVED_ASSIGNMENT_KW,
      "cardinality" => RESERVED_CARDINALITY_KW,
      "cast" => RESERVED_CAST_KW,
      "committed" => RESERVED_COMMITTED_KW,
      "config" => RESERVED_CONFIG_KW,
      "conflict" => RESERVED_CONFLICT_KW,
      "constraint" => RESERVED_CONSTRAINT_KW,
      "cube" => RESERVED_CUBE_KW,
      "current" => RESERVED_CURRENT_KW,
      "database" => RESERVED_DATABASE_KW,
      "ddl" => RESERVED_DDL_KW,
      "declare" => RESERVED_DECLARE_KW,
      "default" => RESERVED_DEFAULT_KW,
      "deferrable" => RESERVED_DEFERRABLE_KW,
      "deferred" => RESERVED_DEFERRED_KW,
      "delegated" => RESERVED_DELEGATED_KW,
      "deny" => RESERVED_DENY_KW,
      "empty" => RESERVED_EMPTY_KW,
      "except" => RESERVED_EXCEPT_KW,
      "expression" => RESERVED_EXPRESSION_KW,
      "extension" => RESERVED_EXTENSION_KW,
      "final" => RESERVED_FINAL_KW,
      "from" => RESERVED_FROM_KW,
      "function" => RESERVED_FUNCTION_KW,
      "future" => RESERVED_FUTURE_KW,
      "implicit" => RESERVED_IMPLICIT_KW,
      "infix" => RESERVED_INFIX_KW,
      "inheritable" => RESERVED_INHERITABLE_KW,
      "instance" => RESERVED_INSTANCE_KW,
      "into" => RESERVED_INTO_KW,
      "isolation" => RESERVED_ISOLATION_KW,
      "json" => RESERVED_JSON_KW,
      "migration" => RESERVED_MIGRATION_KW,
      "named" => RESERVED_NAMED_KW,
      "object" => RESERVED_OBJECT_KW,
      "of" => RESERVED_OF_KW,
      "only" => RESERVED_ONLY_KW,
      "onto" => RESERVED_ONTO_KW,
      "operator" => RESERVED_OPERATOR_KW,
      "optionality" => RESERVED_OPTIONALITY_KW,
      "order" => RESERVED_ORDER_KW,
      "orphan" => RESERVED_ORPHAN_KW,
      "overloaded" => RESERVED_OVERLOADED_KW,
      "owned" => RESERVED_OWNED_KW,
      "package" => RESERVED_PACKAGE_KW,
      "populate" => RESERVED_POPULATE_KW,
      "postfix" => RESERVED_POSTFIX_KW,
      "prefix" => RESERVED_PREFIX_KW,
      "property" => RESERVED_PROPERTY_KW,
      "proposed" => RESERVED_PROPOSED_KW,
      "pseudo" => RESERVED_PSEUDO_KW,
      "read" => RESERVED_READ_KW,
      "reject" => RESERVED_REJECT_KW,
      "release" => RESERVED_RELEASE_KW,
      "rename" => RESERVED_RENAME_KW,
      "required" => RESERVED_REQUIRED_KW,
      "reset" => RESERVED_RESET_KW,
      "restrict" => RESERVED_RESTRICT_KW,
      "rewrite" => RESERVED_REWRITE_KW,
      "role" => RESERVED_ROLE_KW,
      "roles" => RESERVED_ROLES_KW,
      "rollup" => RESERVED_ROLLUP_KW,
      "scalar" => RESERVED_SCALAR_KW,
      "schema" => RESERVED_SCHEMA_KW,
      "sdl" => RESERVED_SDL_KW,
      "serializable" => RESERVED_SERIALIZABLE_KW,
      "session" => RESERVED_SESSION_KW,
      "source" => RESERVED_SOURCE_KW,
      "superuser" => RESERVED_SUPERUSER_KW,
      "system" => RESERVED_SYSTEM_KW,
      "target" => RESERVED_TARGET_KW,
      "ternary" => RESERVED_TERNARY_KW,
      "text" => RESERVED_TEXT_KW,
      "then" => RESERVED_THEN_KW,
      "to" => RESERVED_TO_KW,
      "transaction" => RESERVED_TRANSACTION_KW,
      "type" => RESERVED_TYPE_KW,
      "unless" => RESERVED_UNLESS_KW,
      "verbose" => RESERVED_VERBOSE_KW,
      "version" => RESERVED_VERSION_KW,
      "view" => RESERVED_VIEW_KW,
      "write" => RESERVED_WRITE_KW,
      "analyze" => RESERVED_ANALYZE_KW,
      "anyarray" => RESERVED_ANYARRAY_KW,
      "begin" => RESERVED_BEGIN_KW,
      "case" => RESERVED_CASE_KW,
      "check" => RESERVED_CHECK_KW,
      "deallocate" => RESERVED_DEALLOCATE_KW,
      "discard" => RESERVED_DISCARD_KW,
      "do" => RESERVED_DO_KW,
      "end" => RESERVED_END_KW,
      "execute" => RESERVED_EXECUTE_KW,
      "explain" => RESERVED_EXPLAIN_KW,
      "fetch" => RESERVED_FETCH_KW,
      "get" => RESERVED_GET_KW,
      "grant" => RESERVED_GRANT_KW,
      "import" => RESERVED_IMPORT_KW,
      "listen" => RESERVED_LISTEN_KW,
      "load" => RESERVED_LOAD_KW,
      "lock" => RESERVED_LOCK_KW,
      "match" => RESERVED_MATCH_KW,
      "move" => RESERVED_MOVE_KW,
      "notify" => RESERVED_NOTIFY_KW,
      "over" => RESERVED_OVER_KW,
      "prepare" => RESERVED_PREPARE_KW,
      "partition" => RESERVED_PARTITION_KW,
      "raise" => RESERVED_RAISE_KW,
      "refresh" => RESERVED_REFRESH_KW,
      "reindex" => RESERVED_REINDEX_KW,
      "revoke" => RESERVED_REVOKE_KW,
      "when" => RESERVED_WHEN_KW,
      "window" => RESERVED_WINDOW_KW,
      "never" => RESERVED_NEVER_KW,
      "asc" => EDGE_ASC_KW,
      "desc" => EDGE_DESC_KW,
      "union" => EDGE_UNION_KW,
      "savepoint" => EDGE_SAVEPOINT_KW,
      "rollback" => EDGE_ROLLBACK_KW,
      "annotation" => SDL_ANNOTATION_KW,
      "link" => SDL_LINK_KW,
      "multi" => SDL_MULTI_KW,
      "sequence" => SDL_SEQUENCE_KW,
      "using" => SDL_USING_KW,
      "volatility" => SDL_VOLATILITY_KW,
      "after" => DDL_AFTER_KW,
      "alter" => DDL_ALTER_KW,
      "before" => DDL_BEFORE_KW,
      "create" => DDL_CREATE_KW,
      "drop" => DDL_DROP_KW,
      "first" => DDL_FIRST_KW,
      "last" => DDL_LAST_KW,
      _ => return None,
    };
    Some(kw)
  }

  pub const fn to_string(&self) -> Option<&'static str> {
    let tok = match self {
      ASSIGN => ":=",
      ADD_ASSIGN => "+=",
      SUB_ASSIGN => "-=",
      ARROW => "->",
      COALESCE => "??",
      NAMESPACE => "::",
      BACKWARD_LINK => ".<",
      FLOOR_DIV => "//",
      CONCAT => "++",
      GREATER_EQUAL => ">=",
      LESS_EQUAL => "<=",
      NOT_EQUAL => "!=",
      NOT_DISTINCT_FROM => "?=",
      DISTINCT_FROM => "?!=",
      COMMA => ",",
      OPEN_PAREN => "(",
      CLOSE_PAREN => ")",
      OPEN_SQUARE => "[",
      CLOSE_SQUARE => "]",
      OPEN_CURLY => "{",
      CLOSE_CURLY => "}",
      DOT => ".",
      SEMICOLON => ";",
      COLON => ":",
      ADD => "+",
      SUBTRACT => "-",
      MULTIPLY => "*",
      DIVIDE => "/",
      MODULO => "%",
      POW => "^",
      LESS => "<",
      GREATER => ">",
      EQUAL => "=",
      AMPERSAND => "&",
      PIPE => "|",
      AT => "@",
      ANNOTATION_KW => "annotation",
      LINK_KW => "link",
      MULTI_KW => "multi",
      SEQUENCE_KW => "sequence",
      USING_KW => "using",
      VOLATILITY_KW => "volatility",
      RESERVED_ANNOTATION_KW => "annotation",
      RESERVED_LINK_KW => "link",
      RESERVED_MULTI_KW => "multi",
      RESERVED_SEQUENCE_KW => "sequence",
      RESERVED_USING_KW => "using",
      RESERVED_VOLATILITY_KW => "volatility",
      EDGE_ANNOTATION_KW => "annotation",
      EDGE_LINK_KW => "link",
      EDGE_MULTI_KW => "multi",
      EDGE_SEQUENCE_KW => "sequence",
      EDGE_USING_KW => "using",
      EDGE_VOLATILITY_KW => "volatility",
      SDL_ANNOTATION_KW => "annotation",
      SDL_LINK_KW => "link",
      SDL_MULTI_KW => "multi",
      SDL_SEQUENCE_KW => "sequence",
      SDL_USING_KW => "using",
      SDL_VOLATILITY_KW => "volatility",
      DDL_ANNOTATION_KW => "annotation",
      DDL_LINK_KW => "link",
      DDL_MULTI_KW => "multi",
      DDL_SEQUENCE_KW => "sequence",
      DDL_USING_KW => "using",
      DDL_VOLATILITY_KW => "volatility",
      STRING_LITERAL => "string literal",
      _ => return None,
    };
    Some(tok)
  }
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules ! T { [:=] => { $ crate :: EqlSyntaxKind :: ASSIGN } ; [+=] => { $ crate :: EqlSyntaxKind :: ADD_ASSIGN } ; [-=] => { $ crate :: EqlSyntaxKind :: SUB_ASSIGN } ; [->] => { $ crate :: EqlSyntaxKind :: ARROW } ; [??] => { $ crate :: EqlSyntaxKind :: COALESCE } ; [::] => { $ crate :: EqlSyntaxKind :: NAMESPACE } ; [.<] => { $ crate :: EqlSyntaxKind :: BACKWARD_LINK } ; ["//"] => { $ crate :: EqlSyntaxKind :: FLOOR_DIV } ; [++] => { $ crate :: EqlSyntaxKind :: CONCAT } ; [>=] => { $ crate :: EqlSyntaxKind :: GREATER_EQUAL } ; [<=] => { $ crate :: EqlSyntaxKind :: LESS_EQUAL } ; [!=] => { $ crate :: EqlSyntaxKind :: NOT_EQUAL } ; [?=] => { $ crate :: EqlSyntaxKind :: NOT_DISTINCT_FROM } ; [?!=] => { $ crate :: EqlSyntaxKind :: DISTINCT_FROM } ; [,] => { $ crate :: EqlSyntaxKind :: COMMA } ; ['('] => { $ crate :: EqlSyntaxKind :: OPEN_PAREN } ; [')'] => { $ crate :: EqlSyntaxKind :: CLOSE_PAREN } ; ['['] => { $ crate :: EqlSyntaxKind :: OPEN_SQUARE } ; [']'] => { $ crate :: EqlSyntaxKind :: CLOSE_SQUARE } ; ['{'] => { $ crate :: EqlSyntaxKind :: OPEN_CURLY } ; ['}'] => { $ crate :: EqlSyntaxKind :: CLOSE_CURLY } ; [.] => { $ crate :: EqlSyntaxKind :: DOT } ; [;] => { $ crate :: EqlSyntaxKind :: SEMICOLON } ; [:] => { $ crate :: EqlSyntaxKind :: COLON } ; [+] => { $ crate :: EqlSyntaxKind :: ADD } ; [-] => { $ crate :: EqlSyntaxKind :: SUBTRACT } ; [*] => { $ crate :: EqlSyntaxKind :: MULTIPLY } ; [/] => { $ crate :: EqlSyntaxKind :: DIVIDE } ; [%] => { $ crate :: EqlSyntaxKind :: MODULO } ; [^] => { $ crate :: EqlSyntaxKind :: POW } ; [<] => { $ crate :: EqlSyntaxKind :: LESS } ; [>] => { $ crate :: EqlSyntaxKind :: GREATER } ; [=] => { $ crate :: EqlSyntaxKind :: EQUAL } ; [&] => { $ crate :: EqlSyntaxKind :: AMPERSAND } ; [|] => { $ crate :: EqlSyntaxKind :: PIPE } ; [@] => { $ crate :: EqlSyntaxKind :: AT } ; [annotation] => { $ crate :: EqlSyntaxKind :: ANNOTATION_KW } ; [link] => { $ crate :: EqlSyntaxKind :: LINK_KW } ; [multi] => { $ crate :: EqlSyntaxKind :: MULTI_KW } ; [sequence] => { $ crate :: EqlSyntaxKind :: SEQUENCE_KW } ; [using] => { $ crate :: EqlSyntaxKind :: USING_KW } ; [volatility] => { $ crate :: EqlSyntaxKind :: VOLATILITY_KW } ; [annotation] => { $ crate :: EqlSyntaxKind :: RESERVED_ANNOTATION_KW } ; [link] => { $ crate :: EqlSyntaxKind :: RESERVED_LINK_KW } ; [multi] => { $ crate :: EqlSyntaxKind :: RESERVED_MULTI_KW } ; [sequence] => { $ crate :: EqlSyntaxKind :: RESERVED_SEQUENCE_KW } ; [using] => { $ crate :: EqlSyntaxKind :: RESERVED_USING_KW } ; [volatility] => { $ crate :: EqlSyntaxKind :: RESERVED_VOLATILITY_KW } ; [annotation] => { $ crate :: EqlSyntaxKind :: EDGE_ANNOTATION_KW } ; [link] => { $ crate :: EqlSyntaxKind :: EDGE_LINK_KW } ; [multi] => { $ crate :: EqlSyntaxKind :: EDGE_MULTI_KW } ; [sequence] => { $ crate :: EqlSyntaxKind :: EDGE_SEQUENCE_KW } ; [using] => { $ crate :: EqlSyntaxKind :: EDGE_USING_KW } ; [volatility] => { $ crate :: EqlSyntaxKind :: EDGE_VOLATILITY_KW } ; [annotation] => { $ crate :: EqlSyntaxKind :: SDL_ANNOTATION_KW } ; [link] => { $ crate :: EqlSyntaxKind :: SDL_LINK_KW } ; [multi] => { $ crate :: EqlSyntaxKind :: SDL_MULTI_KW } ; [sequence] => { $ crate :: EqlSyntaxKind :: SDL_SEQUENCE_KW } ; [using] => { $ crate :: EqlSyntaxKind :: SDL_USING_KW } ; [volatility] => { $ crate :: EqlSyntaxKind :: SDL_VOLATILITY_KW } ; [annotation] => { $ crate :: EqlSyntaxKind :: DDL_ANNOTATION_KW } ; [link] => { $ crate :: EqlSyntaxKind :: DDL_LINK_KW } ; [multi] => { $ crate :: EqlSyntaxKind :: DDL_MULTI_KW } ; [sequence] => { $ crate :: EqlSyntaxKind :: DDL_SEQUENCE_KW } ; [using] => { $ crate :: EqlSyntaxKind :: DDL_USING_KW } ; [volatility] => { $ crate :: EqlSyntaxKind :: DDL_VOLATILITY_KW } ; [ident] => { $ crate :: EqlSyntaxKind :: IDENT } ; [EOF] => { $ crate :: EqlSyntaxKind :: EOF } ; [#] => { $ crate :: EqlSyntaxKind :: HASH } ; }
