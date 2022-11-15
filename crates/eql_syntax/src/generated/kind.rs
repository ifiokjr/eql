// Generated file, do not edit by hand, see `xtask/src/codegen`

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
  BACKTICK,
  DOLLAR,
  HASH,
  ABSTRACT_KW,
  ACCESS_KW,
  ALIAS_KW,
  ALL_KW,
  ALLOW_KW,
  AND_KW,
  ANYTUPLE_KW,
  ANYTYPE_KW,
  ARRAY_KW,
  AS_KW,
  B_KW,
  BIGINT_KW,
  BOOL_KW,
  BY_KW,
  BYTES_KW,
  COMMIT_KW,
  CONFIGURE_KW,
  DATETIME_KW,
  DDL_KW,
  DECIMAL_KW,
  DELETE_KW,
  DESCRIBE_KW,
  DETACHED_KW,
  DISTINCT_KW,
  DURATION_KW,
  ELSE_KW,
  ENUM_KW,
  ERRMESSAGE_KW,
  EXISTS_KW,
  EXTENDING_KW,
  FALSE_KW,
  FILTER_KW,
  FLOAT32_KW,
  FLOAT64_KW,
  FOR_KW,
  GLOBAL_KW,
  GROUP_KW,
  IF_KW,
  ILIKE_KW,
  IN_KW,
  INDEX_KW,
  INSERT_KW,
  INT16_KW,
  INT32_KW,
  INT64_KW,
  INTROSPECT_KW,
  IS_KW,
  JSON_KW,
  LIKE_KW,
  LIMIT_KW,
  MODULE_KW,
  NOT_KW,
  OFFSET_KW,
  ON_KW,
  OPTIONAL_KW,
  OR_KW,
  POLICY_KW,
  R_KW,
  RANGE_KW,
  SDL_KW,
  SELECT_KW,
  SEQUENCE_KW,
  SET_KW,
  SINGLE_KW,
  START_KW,
  STD_KW,
  STR_KW,
  TO_KW,
  TRUE_KW,
  TUPLE_KW,
  TYPEOF_KW,
  UPDATE_KW,
  UUID_KW,
  VARIADIC_KW,
  WITH_KW,
  RESERVED_ABORT_KW,
  RESERVED_APPLIED_KW,
  RESERVED_ASSIGNMENT_KW,
  RESERVED_CARDINALITY_KW,
  RESERVED_CAST_KW,
  RESERVED_COMMITTED_KW,
  RESERVED_CONFIG_KW,
  RESERVED_CONFLICT_KW,
  RESERVED_CONSTRAINT_KW,
  RESERVED_CUBE_KW,
  RESERVED_CURRENT_KW,
  RESERVED_DATABASE_KW,
  RESERVED_DECLARE_KW,
  RESERVED_DEFAULT_KW,
  RESERVED_DEFERRABLE_KW,
  RESERVED_DEFERRED_KW,
  RESERVED_DELEGATED_KW,
  RESERVED_DENY_KW,
  RESERVED_EMPTY_KW,
  RESERVED_EXCEPT_KW,
  RESERVED_EXPRESSION_KW,
  RESERVED_EXTENSION_KW,
  RESERVED_FINAL_KW,
  RESERVED_FROM_KW,
  RESERVED_FUNCTION_KW,
  RESERVED_FUTURE_KW,
  RESERVED_IMPLICIT_KW,
  RESERVED_INFIX_KW,
  RESERVED_INHERITABLE_KW,
  RESERVED_INSTANCE_KW,
  RESERVED_INTO_KW,
  RESERVED_ISOLATION_KW,
  RESERVED_MIGRATION_KW,
  RESERVED_NAMED_KW,
  RESERVED_OBJECT_KW,
  RESERVED_OF_KW,
  RESERVED_ONLY_KW,
  RESERVED_ONTO_KW,
  RESERVED_OPERATOR_KW,
  RESERVED_OPTIONALITY_KW,
  RESERVED_ORDER_KW,
  RESERVED_ORPHAN_KW,
  RESERVED_OVERLOADED_KW,
  RESERVED_OWNED_KW,
  RESERVED_PACKAGE_KW,
  RESERVED_POPULATE_KW,
  RESERVED_POSTFIX_KW,
  RESERVED_PREFIX_KW,
  RESERVED_PROPERTY_KW,
  RESERVED_PROPOSED_KW,
  RESERVED_PSEUDO_KW,
  RESERVED_READ_KW,
  RESERVED_REJECT_KW,
  RESERVED_RELEASE_KW,
  RESERVED_RENAME_KW,
  RESERVED_REQUIRED_KW,
  RESERVED_RESET_KW,
  RESERVED_RESTRICT_KW,
  RESERVED_REWRITE_KW,
  RESERVED_ROLE_KW,
  RESERVED_ROLES_KW,
  RESERVED_ROLLUP_KW,
  RESERVED_SCALAR_KW,
  RESERVED_SCHEMA_KW,
  RESERVED_SERIALIZABLE_KW,
  RESERVED_SESSION_KW,
  RESERVED_SOURCE_KW,
  RESERVED_SUPERUSER_KW,
  RESERVED_SYSTEM_KW,
  RESERVED_TARGET_KW,
  RESERVED_TERNARY_KW,
  RESERVED_TEXT_KW,
  RESERVED_THEN_KW,
  RESERVED_TRANSACTION_KW,
  RESERVED_UNLESS_KW,
  RESERVED_VERBOSE_KW,
  RESERVED_VERSION_KW,
  RESERVED_VIEW_KW,
  RESERVED_WRITE_KW,
  RESERVED_ANALYZE_KW,
  RESERVED_ANYARRAY_KW,
  RESERVED_BEGIN_KW,
  RESERVED_CASE_KW,
  RESERVED_CHECK_KW,
  RESERVED_DEALLOCATE_KW,
  RESERVED_DISCARD_KW,
  RESERVED_DO_KW,
  RESERVED_END_KW,
  RESERVED_EXECUTE_KW,
  RESERVED_EXPLAIN_KW,
  RESERVED_FETCH_KW,
  RESERVED_GET_KW,
  RESERVED_GRANT_KW,
  RESERVED_IMPORT_KW,
  RESERVED_LISTEN_KW,
  RESERVED_LOAD_KW,
  RESERVED_LOCK_KW,
  RESERVED_MATCH_KW,
  RESERVED_MOVE_KW,
  RESERVED_NOTIFY_KW,
  RESERVED_OVER_KW,
  RESERVED_PREPARE_KW,
  RESERVED_PARTITION_KW,
  RESERVED_RAISE_KW,
  RESERVED_REFRESH_KW,
  RESERVED_REINDEX_KW,
  RESERVED_REVOKE_KW,
  RESERVED_WHEN_KW,
  RESERVED_WINDOW_KW,
  RESERVED_NEVER_KW,
  EDGE_ASC_KW,
  EDGE_DESC_KW,
  EDGE_UNION_KW,
  EDGE_SAVEPOINT_KW,
  EDGE_ROLLBACK_KW,
  SDL_ANNOTATION_KW,
  SDL_LINK_KW,
  SDL_MULTI_KW,
  SDL_TYPE_KW,
  SDL_USING_KW,
  SDL_VOLATILITY_KW,
  DDL_AFTER_KW,
  DDL_ALTER_KW,
  DDL_BEFORE_KW,
  DDL_CREATE_KW,
  DDL_DROP_KW,
  DDL_FIRST_KW,
  DDL_LAST_KW,
  INT_LITERAL,
  FLOAT_LITERAL,
  BIG_INT_LITERAL,
  DECIMAL_LITERAL,
  STRING_LITERAL,
  BYTE_LITERAL,
  ERROR,
  PLAIN_IDENT,
  QUOTED_IDENT,
  NEWLINE,
  WHITESPACE,
  COMMENT,
  QUERY_PARAMETER,
  ANY_LITERAL_EXPRESSION,
  ANY_ROOT,
  ANY_UNKNOWN,
  ARRAY_TYPE,
  BARE_BYTES_LITERAL_EXPRESSION,
  BARE_STRING_LITERAL_EXPRESSION,
  BIG_INT_LITERAL_EXPRESSION,
  BIG_INT_TYPE,
  BOOLEAN_LITERAL_EXPRESSION,
  BOOLEAN_TYPE,
  BYTES_LITERAL_EXPRESSION,
  BYTES_TYPE,
  DATE_TIME_TYPE,
  DECIMAL_LITERAL_EXPRESSION,
  DECIMAL_TYPE,
  DOT_REFERENCE_NAME,
  DURATION_TYPE,
  EMPTY_STATEMENT,
  EQL_ROOT,
  EQL_ROOT_ITEM,
  EQL_ROOT_ITEM_LIST,
  EXPRESSION,
  FLOAT_LITERAL_EXPRESSION,
  FLOAT_SIXTY_FOUR_TYPE,
  FLOAT_THIRTY_TWO_TYPE,
  IDENT,
  INT_LITERAL_EXPRESSION,
  INT_SIXTEEN_TYPE,
  INT_SIXTY_FOUR_TYPE,
  INT_THIRTY_TWO_TYPE,
  JSON_TYPE,
  NAME,
  OUTGOING_PATH_STEP,
  PARAMETER_NAME,
  PATH_STEP,
  PRIMITIVE_TYPE,
  QUALIFIED_NAME,
  RANGE_TYPE,
  RANGE_TYPE_MEMBER,
  RAW_BYTES_LITERAL_EXPRESSION,
  RAW_STRING_LITERAL_EXPRESSION,
  SDL_ANNOTATION,
  SDL_ANNOTATION_LIST,
  SDL_ANNOTATION_SCHEMA,
  SDL_ANNOTATION_SCHEMA_BLOCK,
  SDL_ANNOTATION_SCHEMA_BODY,
  SDL_CONSTRAINT,
  SDL_CONSTRAINT_ARG,
  SDL_CONSTRAINT_ARGS,
  SDL_CONSTRAINT_ARG_LIST,
  SDL_CONSTRAINT_BLOCK,
  SDL_CONSTRAINT_BODY,
  SDL_CONSTRAINT_ERROR_MESSAGE,
  SDL_CONSTRAINT_EXCEPT,
  SDL_CONSTRAINT_LIST,
  SDL_CONSTRAINT_SUBJECT_EXPRESSION,
  SDL_ENUM_DECLARATION,
  SDL_ENUM_DECLARATION_MEMBERS,
  SDL_EXTENDING,
  SDL_EXTENDING_NAMES,
  SDL_INDEX,
  SDL_INDEX_LIST,
  SDL_LINK,
  SDL_LINK_LIST,
  SDL_MODULE,
  SDL_OBJECT_BLOCK,
  SDL_OBJECT_BODY,
  SDL_OBJECT_SCHEMA,
  SDL_PROPERTY,
  SDL_PROPERTY_LIST,
  SDL_ROOT,
  SDL_ROOT_ITEM,
  SDL_ROOT_ITEM_LIST,
  SDL_SCALAR_BLOCK,
  SDL_SCALAR_BODY,
  SDL_SCALAR_EXTENDING,
  SDL_SCALAR_EXTENDING_ENUM,
  SDL_SCALAR_EXTENDING_TYPE,
  SDL_SCHEMA,
  SDL_SCHEMA_CONSTRAINT,
  SDL_SCHEMA_CONSTRAINT_ARGS,
  SDL_SCHEMA_CONSTRAINT_BLOCK,
  SDL_SCHEMA_CONSTRAINT_BODY,
  SDL_SCHEMA_CONSTRAIN_PARAM,
  SDL_SCHEMA_CONSTRAIN_PARAM_LIST,
  SDL_SCHEMA_LIST,
  SDL_SCHEMA_SCALAR,
  SDL_UNKNOWN,
  SDL_UNKNOWN_EXPRESSION,
  SDL_UNKNOWN_SCHEMA,
  SEQUENCE_TYPE,
  STATEMENT,
  STRING_LITERAL_EXPRESSION,
  STRING_TYPE,
  SYNTAX_ELEMENT,
  TUPLE_TYPE,
  TUPLE_TYPE_MEMBERS,
  TYPE_CAST_EXPRESSION,
  TYPE_CAST_TARGET,
  TYPE_EXPRESSION,
  UNKNOWN_EXPRESSION,
  UNKNOWN_STATEMENT,
  UNQUALIFIED_NAME,
  UUID_TYPE,
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
      | MODULO | POW | LESS | GREATER | EQUAL | AMPERSAND | PIPE | AT | BACKTICK | DOLLAR
      | HASH => true,
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
      EQL_ROOT_ITEM_LIST
      | SDL_ANNOTATION_LIST
      | SDL_CONSTRAINT_ARG_LIST
      | SDL_CONSTRAINT_LIST
      | SDL_INDEX_LIST
      | SDL_LINK_LIST
      | SDL_PROPERTY_LIST
      | SDL_ROOT_ITEM_LIST
      | SDL_SCHEMA_CONSTRAIN_PARAM_LIST
      | SDL_SCHEMA_LIST => true,
      _ => false,
    }
  }

  pub fn from_keyword(ident: &str) -> Option<EqlSyntaxKind> {
    let kw = match ident {
      "abstract" => ABSTRACT_KW,
      "access" => ACCESS_KW,
      "alias" => ALIAS_KW,
      "all" => ALL_KW,
      "allow" => ALLOW_KW,
      "and" => AND_KW,
      "anytuple" => ANYTUPLE_KW,
      "anytype" => ANYTYPE_KW,
      "array" => ARRAY_KW,
      "as" => AS_KW,
      "b" => B_KW,
      "bigint" => BIGINT_KW,
      "bool" => BOOL_KW,
      "by" => BY_KW,
      "bytes" => BYTES_KW,
      "commit" => COMMIT_KW,
      "configure" => CONFIGURE_KW,
      "datetime" => DATETIME_KW,
      "ddl" => DDL_KW,
      "decimal" => DECIMAL_KW,
      "delete" => DELETE_KW,
      "describe" => DESCRIBE_KW,
      "detached" => DETACHED_KW,
      "distinct" => DISTINCT_KW,
      "duration" => DURATION_KW,
      "else" => ELSE_KW,
      "enum" => ENUM_KW,
      "errmessage" => ERRMESSAGE_KW,
      "exists" => EXISTS_KW,
      "extending" => EXTENDING_KW,
      "false" => FALSE_KW,
      "filter" => FILTER_KW,
      "float32" => FLOAT32_KW,
      "float64" => FLOAT64_KW,
      "for" => FOR_KW,
      "global" => GLOBAL_KW,
      "group" => GROUP_KW,
      "if" => IF_KW,
      "ilike" => ILIKE_KW,
      "in" => IN_KW,
      "index" => INDEX_KW,
      "insert" => INSERT_KW,
      "int16" => INT16_KW,
      "int32" => INT32_KW,
      "int64" => INT64_KW,
      "introspect" => INTROSPECT_KW,
      "is" => IS_KW,
      "json" => JSON_KW,
      "like" => LIKE_KW,
      "limit" => LIMIT_KW,
      "module" => MODULE_KW,
      "not" => NOT_KW,
      "offset" => OFFSET_KW,
      "on" => ON_KW,
      "optional" => OPTIONAL_KW,
      "or" => OR_KW,
      "policy" => POLICY_KW,
      "r" => R_KW,
      "range" => RANGE_KW,
      "sdl" => SDL_KW,
      "select" => SELECT_KW,
      "sequence" => SEQUENCE_KW,
      "set" => SET_KW,
      "single" => SINGLE_KW,
      "start" => START_KW,
      "std" => STD_KW,
      "str" => STR_KW,
      "to" => TO_KW,
      "true" => TRUE_KW,
      "tuple" => TUPLE_KW,
      "typeof" => TYPEOF_KW,
      "update" => UPDATE_KW,
      "uuid" => UUID_KW,
      "variadic" => VARIADIC_KW,
      "with" => WITH_KW,
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
      "serializable" => RESERVED_SERIALIZABLE_KW,
      "session" => RESERVED_SESSION_KW,
      "source" => RESERVED_SOURCE_KW,
      "superuser" => RESERVED_SUPERUSER_KW,
      "system" => RESERVED_SYSTEM_KW,
      "target" => RESERVED_TARGET_KW,
      "ternary" => RESERVED_TERNARY_KW,
      "text" => RESERVED_TEXT_KW,
      "then" => RESERVED_THEN_KW,
      "transaction" => RESERVED_TRANSACTION_KW,
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
      "type" => SDL_TYPE_KW,
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
      BACKTICK => "`",
      DOLLAR => "$",
      HASH => "#",
      ABSTRACT_KW => "abstract",
      ACCESS_KW => "access",
      ALIAS_KW => "alias",
      ALL_KW => "all",
      ALLOW_KW => "allow",
      AND_KW => "and",
      ANYTUPLE_KW => "anytuple",
      ANYTYPE_KW => "anytype",
      ARRAY_KW => "array",
      AS_KW => "as",
      B_KW => "b",
      BIGINT_KW => "bigint",
      BOOL_KW => "bool",
      BY_KW => "by",
      BYTES_KW => "bytes",
      COMMIT_KW => "commit",
      CONFIGURE_KW => "configure",
      DATETIME_KW => "datetime",
      DDL_KW => "ddl",
      DECIMAL_KW => "decimal",
      DELETE_KW => "delete",
      DESCRIBE_KW => "describe",
      DETACHED_KW => "detached",
      DISTINCT_KW => "distinct",
      DURATION_KW => "duration",
      ELSE_KW => "else",
      ENUM_KW => "enum",
      ERRMESSAGE_KW => "errmessage",
      EXISTS_KW => "exists",
      EXTENDING_KW => "extending",
      FALSE_KW => "false",
      FILTER_KW => "filter",
      FLOAT32_KW => "float32",
      FLOAT64_KW => "float64",
      FOR_KW => "for",
      GLOBAL_KW => "global",
      GROUP_KW => "group",
      IF_KW => "if",
      ILIKE_KW => "ilike",
      IN_KW => "in",
      INDEX_KW => "index",
      INSERT_KW => "insert",
      INT16_KW => "int16",
      INT32_KW => "int32",
      INT64_KW => "int64",
      INTROSPECT_KW => "introspect",
      IS_KW => "is",
      JSON_KW => "json",
      LIKE_KW => "like",
      LIMIT_KW => "limit",
      MODULE_KW => "module",
      NOT_KW => "not",
      OFFSET_KW => "offset",
      ON_KW => "on",
      OPTIONAL_KW => "optional",
      OR_KW => "or",
      POLICY_KW => "policy",
      R_KW => "r",
      RANGE_KW => "range",
      SDL_KW => "sdl",
      SELECT_KW => "select",
      SEQUENCE_KW => "sequence",
      SET_KW => "set",
      SINGLE_KW => "single",
      START_KW => "start",
      STD_KW => "std",
      STR_KW => "str",
      TO_KW => "to",
      TRUE_KW => "true",
      TUPLE_KW => "tuple",
      TYPEOF_KW => "typeof",
      UPDATE_KW => "update",
      UUID_KW => "uuid",
      VARIADIC_KW => "variadic",
      WITH_KW => "with",
      RESERVED_ABORT_KW => "abort",
      RESERVED_APPLIED_KW => "applied",
      RESERVED_ASSIGNMENT_KW => "assignment",
      RESERVED_CARDINALITY_KW => "cardinality",
      RESERVED_CAST_KW => "cast",
      RESERVED_COMMITTED_KW => "committed",
      RESERVED_CONFIG_KW => "config",
      RESERVED_CONFLICT_KW => "conflict",
      RESERVED_CONSTRAINT_KW => "constraint",
      RESERVED_CUBE_KW => "cube",
      RESERVED_CURRENT_KW => "current",
      RESERVED_DATABASE_KW => "database",
      RESERVED_DECLARE_KW => "declare",
      RESERVED_DEFAULT_KW => "default",
      RESERVED_DEFERRABLE_KW => "deferrable",
      RESERVED_DEFERRED_KW => "deferred",
      RESERVED_DELEGATED_KW => "delegated",
      RESERVED_DENY_KW => "deny",
      RESERVED_EMPTY_KW => "empty",
      RESERVED_EXCEPT_KW => "except",
      RESERVED_EXPRESSION_KW => "expression",
      RESERVED_EXTENSION_KW => "extension",
      RESERVED_FINAL_KW => "final",
      RESERVED_FROM_KW => "from",
      RESERVED_FUNCTION_KW => "function",
      RESERVED_FUTURE_KW => "future",
      RESERVED_IMPLICIT_KW => "implicit",
      RESERVED_INFIX_KW => "infix",
      RESERVED_INHERITABLE_KW => "inheritable",
      RESERVED_INSTANCE_KW => "instance",
      RESERVED_INTO_KW => "into",
      RESERVED_ISOLATION_KW => "isolation",
      RESERVED_MIGRATION_KW => "migration",
      RESERVED_NAMED_KW => "named",
      RESERVED_OBJECT_KW => "object",
      RESERVED_OF_KW => "of",
      RESERVED_ONLY_KW => "only",
      RESERVED_ONTO_KW => "onto",
      RESERVED_OPERATOR_KW => "operator",
      RESERVED_OPTIONALITY_KW => "optionality",
      RESERVED_ORDER_KW => "order",
      RESERVED_ORPHAN_KW => "orphan",
      RESERVED_OVERLOADED_KW => "overloaded",
      RESERVED_OWNED_KW => "owned",
      RESERVED_PACKAGE_KW => "package",
      RESERVED_POPULATE_KW => "populate",
      RESERVED_POSTFIX_KW => "postfix",
      RESERVED_PREFIX_KW => "prefix",
      RESERVED_PROPERTY_KW => "property",
      RESERVED_PROPOSED_KW => "proposed",
      RESERVED_PSEUDO_KW => "pseudo",
      RESERVED_READ_KW => "read",
      RESERVED_REJECT_KW => "reject",
      RESERVED_RELEASE_KW => "release",
      RESERVED_RENAME_KW => "rename",
      RESERVED_REQUIRED_KW => "required",
      RESERVED_RESET_KW => "reset",
      RESERVED_RESTRICT_KW => "restrict",
      RESERVED_REWRITE_KW => "rewrite",
      RESERVED_ROLE_KW => "role",
      RESERVED_ROLES_KW => "roles",
      RESERVED_ROLLUP_KW => "rollup",
      RESERVED_SCALAR_KW => "scalar",
      RESERVED_SCHEMA_KW => "schema",
      RESERVED_SERIALIZABLE_KW => "serializable",
      RESERVED_SESSION_KW => "session",
      RESERVED_SOURCE_KW => "source",
      RESERVED_SUPERUSER_KW => "superuser",
      RESERVED_SYSTEM_KW => "system",
      RESERVED_TARGET_KW => "target",
      RESERVED_TERNARY_KW => "ternary",
      RESERVED_TEXT_KW => "text",
      RESERVED_THEN_KW => "then",
      RESERVED_TRANSACTION_KW => "transaction",
      RESERVED_UNLESS_KW => "unless",
      RESERVED_VERBOSE_KW => "verbose",
      RESERVED_VERSION_KW => "version",
      RESERVED_VIEW_KW => "view",
      RESERVED_WRITE_KW => "write",
      RESERVED_ANALYZE_KW => "analyze",
      RESERVED_ANYARRAY_KW => "anyarray",
      RESERVED_BEGIN_KW => "begin",
      RESERVED_CASE_KW => "case",
      RESERVED_CHECK_KW => "check",
      RESERVED_DEALLOCATE_KW => "deallocate",
      RESERVED_DISCARD_KW => "discard",
      RESERVED_DO_KW => "do",
      RESERVED_END_KW => "end",
      RESERVED_EXECUTE_KW => "execute",
      RESERVED_EXPLAIN_KW => "explain",
      RESERVED_FETCH_KW => "fetch",
      RESERVED_GET_KW => "get",
      RESERVED_GRANT_KW => "grant",
      RESERVED_IMPORT_KW => "import",
      RESERVED_LISTEN_KW => "listen",
      RESERVED_LOAD_KW => "load",
      RESERVED_LOCK_KW => "lock",
      RESERVED_MATCH_KW => "match",
      RESERVED_MOVE_KW => "move",
      RESERVED_NOTIFY_KW => "notify",
      RESERVED_OVER_KW => "over",
      RESERVED_PREPARE_KW => "prepare",
      RESERVED_PARTITION_KW => "partition",
      RESERVED_RAISE_KW => "raise",
      RESERVED_REFRESH_KW => "refresh",
      RESERVED_REINDEX_KW => "reindex",
      RESERVED_REVOKE_KW => "revoke",
      RESERVED_WHEN_KW => "when",
      RESERVED_WINDOW_KW => "window",
      RESERVED_NEVER_KW => "never",
      EDGE_ASC_KW => "asc",
      EDGE_DESC_KW => "desc",
      EDGE_UNION_KW => "union",
      EDGE_SAVEPOINT_KW => "savepoint",
      EDGE_ROLLBACK_KW => "rollback",
      SDL_ANNOTATION_KW => "annotation",
      SDL_LINK_KW => "link",
      SDL_MULTI_KW => "multi",
      SDL_TYPE_KW => "type",
      SDL_USING_KW => "using",
      SDL_VOLATILITY_KW => "volatility",
      DDL_AFTER_KW => "after",
      DDL_ALTER_KW => "alter",
      DDL_BEFORE_KW => "before",
      DDL_CREATE_KW => "create",
      DDL_DROP_KW => "drop",
      DDL_FIRST_KW => "first",
      DDL_LAST_KW => "last",
      STRING_LITERAL => "string literal",
      _ => return None,
    };
    Some(tok)
  }
}
impl From<&'_ [u8]> for EqlSyntaxKind {
  fn from(slice: &'_ [u8]) -> Self {
    match slice {
      b"abstract" => ABSTRACT_KW,
      b"access" => ACCESS_KW,
      b"alias" => ALIAS_KW,
      b"all" => ALL_KW,
      b"allow" => ALLOW_KW,
      b"and" => AND_KW,
      b"anytuple" => ANYTUPLE_KW,
      b"anytype" => ANYTYPE_KW,
      b"array" => ARRAY_KW,
      b"as" => AS_KW,
      b"b" => B_KW,
      b"bigint" => BIGINT_KW,
      b"bool" => BOOL_KW,
      b"by" => BY_KW,
      b"bytes" => BYTES_KW,
      b"commit" => COMMIT_KW,
      b"configure" => CONFIGURE_KW,
      b"datetime" => DATETIME_KW,
      b"ddl" => DDL_KW,
      b"decimal" => DECIMAL_KW,
      b"delete" => DELETE_KW,
      b"describe" => DESCRIBE_KW,
      b"detached" => DETACHED_KW,
      b"distinct" => DISTINCT_KW,
      b"duration" => DURATION_KW,
      b"else" => ELSE_KW,
      b"enum" => ENUM_KW,
      b"errmessage" => ERRMESSAGE_KW,
      b"exists" => EXISTS_KW,
      b"extending" => EXTENDING_KW,
      b"false" => FALSE_KW,
      b"filter" => FILTER_KW,
      b"float32" => FLOAT32_KW,
      b"float64" => FLOAT64_KW,
      b"for" => FOR_KW,
      b"global" => GLOBAL_KW,
      b"group" => GROUP_KW,
      b"if" => IF_KW,
      b"ilike" => ILIKE_KW,
      b"in" => IN_KW,
      b"index" => INDEX_KW,
      b"insert" => INSERT_KW,
      b"int16" => INT16_KW,
      b"int32" => INT32_KW,
      b"int64" => INT64_KW,
      b"introspect" => INTROSPECT_KW,
      b"is" => IS_KW,
      b"json" => JSON_KW,
      b"like" => LIKE_KW,
      b"limit" => LIMIT_KW,
      b"module" => MODULE_KW,
      b"not" => NOT_KW,
      b"offset" => OFFSET_KW,
      b"on" => ON_KW,
      b"optional" => OPTIONAL_KW,
      b"or" => OR_KW,
      b"policy" => POLICY_KW,
      b"r" => R_KW,
      b"range" => RANGE_KW,
      b"sdl" => SDL_KW,
      b"select" => SELECT_KW,
      b"sequence" => SEQUENCE_KW,
      b"set" => SET_KW,
      b"single" => SINGLE_KW,
      b"start" => START_KW,
      b"std" => STD_KW,
      b"str" => STR_KW,
      b"to" => TO_KW,
      b"true" => TRUE_KW,
      b"tuple" => TUPLE_KW,
      b"typeof" => TYPEOF_KW,
      b"update" => UPDATE_KW,
      b"uuid" => UUID_KW,
      b"variadic" => VARIADIC_KW,
      b"with" => WITH_KW,
      b"abort" => RESERVED_ABORT_KW,
      b"applied" => RESERVED_APPLIED_KW,
      b"assignment" => RESERVED_ASSIGNMENT_KW,
      b"cardinality" => RESERVED_CARDINALITY_KW,
      b"cast" => RESERVED_CAST_KW,
      b"committed" => RESERVED_COMMITTED_KW,
      b"config" => RESERVED_CONFIG_KW,
      b"conflict" => RESERVED_CONFLICT_KW,
      b"constraint" => RESERVED_CONSTRAINT_KW,
      b"cube" => RESERVED_CUBE_KW,
      b"current" => RESERVED_CURRENT_KW,
      b"database" => RESERVED_DATABASE_KW,
      b"declare" => RESERVED_DECLARE_KW,
      b"default" => RESERVED_DEFAULT_KW,
      b"deferrable" => RESERVED_DEFERRABLE_KW,
      b"deferred" => RESERVED_DEFERRED_KW,
      b"delegated" => RESERVED_DELEGATED_KW,
      b"deny" => RESERVED_DENY_KW,
      b"empty" => RESERVED_EMPTY_KW,
      b"except" => RESERVED_EXCEPT_KW,
      b"expression" => RESERVED_EXPRESSION_KW,
      b"extension" => RESERVED_EXTENSION_KW,
      b"final" => RESERVED_FINAL_KW,
      b"from" => RESERVED_FROM_KW,
      b"function" => RESERVED_FUNCTION_KW,
      b"future" => RESERVED_FUTURE_KW,
      b"implicit" => RESERVED_IMPLICIT_KW,
      b"infix" => RESERVED_INFIX_KW,
      b"inheritable" => RESERVED_INHERITABLE_KW,
      b"instance" => RESERVED_INSTANCE_KW,
      b"into" => RESERVED_INTO_KW,
      b"isolation" => RESERVED_ISOLATION_KW,
      b"migration" => RESERVED_MIGRATION_KW,
      b"named" => RESERVED_NAMED_KW,
      b"object" => RESERVED_OBJECT_KW,
      b"of" => RESERVED_OF_KW,
      b"only" => RESERVED_ONLY_KW,
      b"onto" => RESERVED_ONTO_KW,
      b"operator" => RESERVED_OPERATOR_KW,
      b"optionality" => RESERVED_OPTIONALITY_KW,
      b"order" => RESERVED_ORDER_KW,
      b"orphan" => RESERVED_ORPHAN_KW,
      b"overloaded" => RESERVED_OVERLOADED_KW,
      b"owned" => RESERVED_OWNED_KW,
      b"package" => RESERVED_PACKAGE_KW,
      b"populate" => RESERVED_POPULATE_KW,
      b"postfix" => RESERVED_POSTFIX_KW,
      b"prefix" => RESERVED_PREFIX_KW,
      b"property" => RESERVED_PROPERTY_KW,
      b"proposed" => RESERVED_PROPOSED_KW,
      b"pseudo" => RESERVED_PSEUDO_KW,
      b"read" => RESERVED_READ_KW,
      b"reject" => RESERVED_REJECT_KW,
      b"release" => RESERVED_RELEASE_KW,
      b"rename" => RESERVED_RENAME_KW,
      b"required" => RESERVED_REQUIRED_KW,
      b"reset" => RESERVED_RESET_KW,
      b"restrict" => RESERVED_RESTRICT_KW,
      b"rewrite" => RESERVED_REWRITE_KW,
      b"role" => RESERVED_ROLE_KW,
      b"roles" => RESERVED_ROLES_KW,
      b"rollup" => RESERVED_ROLLUP_KW,
      b"scalar" => RESERVED_SCALAR_KW,
      b"schema" => RESERVED_SCHEMA_KW,
      b"serializable" => RESERVED_SERIALIZABLE_KW,
      b"session" => RESERVED_SESSION_KW,
      b"source" => RESERVED_SOURCE_KW,
      b"superuser" => RESERVED_SUPERUSER_KW,
      b"system" => RESERVED_SYSTEM_KW,
      b"target" => RESERVED_TARGET_KW,
      b"ternary" => RESERVED_TERNARY_KW,
      b"text" => RESERVED_TEXT_KW,
      b"then" => RESERVED_THEN_KW,
      b"transaction" => RESERVED_TRANSACTION_KW,
      b"unless" => RESERVED_UNLESS_KW,
      b"verbose" => RESERVED_VERBOSE_KW,
      b"version" => RESERVED_VERSION_KW,
      b"view" => RESERVED_VIEW_KW,
      b"write" => RESERVED_WRITE_KW,
      b"analyze" => RESERVED_ANALYZE_KW,
      b"anyarray" => RESERVED_ANYARRAY_KW,
      b"begin" => RESERVED_BEGIN_KW,
      b"case" => RESERVED_CASE_KW,
      b"check" => RESERVED_CHECK_KW,
      b"deallocate" => RESERVED_DEALLOCATE_KW,
      b"discard" => RESERVED_DISCARD_KW,
      b"do" => RESERVED_DO_KW,
      b"end" => RESERVED_END_KW,
      b"execute" => RESERVED_EXECUTE_KW,
      b"explain" => RESERVED_EXPLAIN_KW,
      b"fetch" => RESERVED_FETCH_KW,
      b"get" => RESERVED_GET_KW,
      b"grant" => RESERVED_GRANT_KW,
      b"import" => RESERVED_IMPORT_KW,
      b"listen" => RESERVED_LISTEN_KW,
      b"load" => RESERVED_LOAD_KW,
      b"lock" => RESERVED_LOCK_KW,
      b"match" => RESERVED_MATCH_KW,
      b"move" => RESERVED_MOVE_KW,
      b"notify" => RESERVED_NOTIFY_KW,
      b"over" => RESERVED_OVER_KW,
      b"prepare" => RESERVED_PREPARE_KW,
      b"partition" => RESERVED_PARTITION_KW,
      b"raise" => RESERVED_RAISE_KW,
      b"refresh" => RESERVED_REFRESH_KW,
      b"reindex" => RESERVED_REINDEX_KW,
      b"revoke" => RESERVED_REVOKE_KW,
      b"when" => RESERVED_WHEN_KW,
      b"window" => RESERVED_WINDOW_KW,
      b"never" => RESERVED_NEVER_KW,
      b"asc" => EDGE_ASC_KW,
      b"desc" => EDGE_DESC_KW,
      b"union" => EDGE_UNION_KW,
      b"savepoint" => EDGE_SAVEPOINT_KW,
      b"rollback" => EDGE_ROLLBACK_KW,
      b"annotation" => SDL_ANNOTATION_KW,
      b"link" => SDL_LINK_KW,
      b"multi" => SDL_MULTI_KW,
      b"type" => SDL_TYPE_KW,
      b"using" => SDL_USING_KW,
      b"volatility" => SDL_VOLATILITY_KW,
      b"after" => DDL_AFTER_KW,
      b"alter" => DDL_ALTER_KW,
      b"before" => DDL_BEFORE_KW,
      b"create" => DDL_CREATE_KW,
      b"drop" => DDL_DROP_KW,
      b"first" => DDL_FIRST_KW,
      b"last" => DDL_LAST_KW,
      _ => IDENT,
    }
  }
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules! T {
  [:=] => { $ crate::EqlSyntaxKind::ASSIGN };
  [+=] => { $ crate::EqlSyntaxKind::ADD_ASSIGN };
  [-=] => { $ crate::EqlSyntaxKind::SUB_ASSIGN };
  [->] => { $ crate::EqlSyntaxKind::ARROW };
  [??] => { $ crate::EqlSyntaxKind::COALESCE };
  [::] => { $ crate::EqlSyntaxKind::NAMESPACE };
  [.<] => { $ crate::EqlSyntaxKind::BACKWARD_LINK };
  ["//"] => { $ crate::EqlSyntaxKind::FLOOR_DIV };
  [++] => { $ crate::EqlSyntaxKind::CONCAT };
  [>=] => { $ crate::EqlSyntaxKind::GREATER_EQUAL };
  [<=] => { $ crate::EqlSyntaxKind::LESS_EQUAL };
  [!=] => { $ crate::EqlSyntaxKind::NOT_EQUAL };
  [?=] => { $ crate::EqlSyntaxKind::NOT_DISTINCT_FROM };
  [?!=] => { $ crate::EqlSyntaxKind::DISTINCT_FROM };
  [,] => { $ crate::EqlSyntaxKind::COMMA };
  ['('] => { $ crate::EqlSyntaxKind::OPEN_PAREN };
  [')'] => { $ crate::EqlSyntaxKind::CLOSE_PAREN };
  ['['] => { $ crate::EqlSyntaxKind::OPEN_SQUARE };
  [']'] => { $ crate::EqlSyntaxKind::CLOSE_SQUARE };
  ['{'] => { $ crate::EqlSyntaxKind::OPEN_CURLY };
  ['}'] => { $ crate::EqlSyntaxKind::CLOSE_CURLY };
  [.] => { $ crate::EqlSyntaxKind::DOT };
  [;] => { $ crate::EqlSyntaxKind::SEMICOLON };
  [:] => { $ crate::EqlSyntaxKind::COLON };
  [+] => { $ crate::EqlSyntaxKind::ADD };
  [-] => { $ crate::EqlSyntaxKind::SUBTRACT };
  [*] => { $ crate::EqlSyntaxKind::MULTIPLY };
  [/] => { $ crate::EqlSyntaxKind::DIVIDE };
  [%] => { $ crate::EqlSyntaxKind::MODULO };
  [^] => { $ crate::EqlSyntaxKind::POW };
  [<] => { $ crate::EqlSyntaxKind::LESS };
  [>] => { $ crate::EqlSyntaxKind::GREATER };
  [=] => { $ crate::EqlSyntaxKind::EQUAL };
  [&] => { $ crate::EqlSyntaxKind::AMPERSAND };
  [|] => { $ crate::EqlSyntaxKind::PIPE };
  [@] => { $ crate::EqlSyntaxKind::AT };
  ['`'] => { $ crate::EqlSyntaxKind::BACKTICK };
  ['$'] => { $ crate::EqlSyntaxKind::DOLLAR };
  [#] => { $ crate::EqlSyntaxKind::HASH };
  [abstract] => { $ crate::EqlSyntaxKind::ABSTRACT_KW };
  [access] => { $ crate::EqlSyntaxKind::ACCESS_KW };
  [alias] => { $ crate::EqlSyntaxKind::ALIAS_KW };
  [all] => { $ crate::EqlSyntaxKind::ALL_KW };
  [allow] => { $ crate::EqlSyntaxKind::ALLOW_KW };
  [and] => { $ crate::EqlSyntaxKind::AND_KW };
  [anytuple] => { $ crate::EqlSyntaxKind::ANYTUPLE_KW };
  [anytype] => { $ crate::EqlSyntaxKind::ANYTYPE_KW };
  [array] => { $ crate::EqlSyntaxKind::ARRAY_KW };
  [as] => { $ crate::EqlSyntaxKind::AS_KW };
  [b] => { $ crate::EqlSyntaxKind::B_KW };
  [bigint] => { $ crate::EqlSyntaxKind::BIGINT_KW };
  [bool] => { $ crate::EqlSyntaxKind::BOOL_KW };
  [by] => { $ crate::EqlSyntaxKind::BY_KW };
  [bytes] => { $ crate::EqlSyntaxKind::BYTES_KW };
  [commit] => { $ crate::EqlSyntaxKind::COMMIT_KW };
  [configure] => { $ crate::EqlSyntaxKind::CONFIGURE_KW };
  [datetime] => { $ crate::EqlSyntaxKind::DATETIME_KW };
  [ddl] => { $ crate::EqlSyntaxKind::DDL_KW };
  [decimal] => { $ crate::EqlSyntaxKind::DECIMAL_KW };
  [delete] => { $ crate::EqlSyntaxKind::DELETE_KW };
  [describe] => { $ crate::EqlSyntaxKind::DESCRIBE_KW };
  [detached] => { $ crate::EqlSyntaxKind::DETACHED_KW };
  [distinct] => { $ crate::EqlSyntaxKind::DISTINCT_KW };
  [duration] => { $ crate::EqlSyntaxKind::DURATION_KW };
  [else] => { $ crate::EqlSyntaxKind::ELSE_KW };
  [enum] => { $ crate::EqlSyntaxKind::ENUM_KW };
  [errmessage] => { $ crate::EqlSyntaxKind::ERRMESSAGE_KW };
  [exists] => { $ crate::EqlSyntaxKind::EXISTS_KW };
  [extending] => { $ crate::EqlSyntaxKind::EXTENDING_KW };
  [false] => { $ crate::EqlSyntaxKind::FALSE_KW };
  [filter] => { $ crate::EqlSyntaxKind::FILTER_KW };
  [float32] => { $ crate::EqlSyntaxKind::FLOAT32_KW };
  [float64] => { $ crate::EqlSyntaxKind::FLOAT64_KW };
  [for] => { $ crate::EqlSyntaxKind::FOR_KW };
  [global] => { $ crate::EqlSyntaxKind::GLOBAL_KW };
  [group] => { $ crate::EqlSyntaxKind::GROUP_KW };
  [if] => { $ crate::EqlSyntaxKind::IF_KW };
  [ilike] => { $ crate::EqlSyntaxKind::ILIKE_KW };
  [in] => { $ crate::EqlSyntaxKind::IN_KW };
  [index] => { $ crate::EqlSyntaxKind::INDEX_KW };
  [insert] => { $ crate::EqlSyntaxKind::INSERT_KW };
  [int16] => { $ crate::EqlSyntaxKind::INT16_KW };
  [int32] => { $ crate::EqlSyntaxKind::INT32_KW };
  [int64] => { $ crate::EqlSyntaxKind::INT64_KW };
  [introspect] => { $ crate::EqlSyntaxKind::INTROSPECT_KW };
  [is] => { $ crate::EqlSyntaxKind::IS_KW };
  [json] => { $ crate::EqlSyntaxKind::JSON_KW };
  [like] => { $ crate::EqlSyntaxKind::LIKE_KW };
  [limit] => { $ crate::EqlSyntaxKind::LIMIT_KW };
  [module] => { $ crate::EqlSyntaxKind::MODULE_KW };
  [not] => { $ crate::EqlSyntaxKind::NOT_KW };
  [offset] => { $ crate::EqlSyntaxKind::OFFSET_KW };
  [on] => { $ crate::EqlSyntaxKind::ON_KW };
  [optional] => { $ crate::EqlSyntaxKind::OPTIONAL_KW };
  [or] => { $ crate::EqlSyntaxKind::OR_KW };
  [policy] => { $ crate::EqlSyntaxKind::POLICY_KW };
  [r] => { $ crate::EqlSyntaxKind::R_KW };
  [range] => { $ crate::EqlSyntaxKind::RANGE_KW };
  [sdl] => { $ crate::EqlSyntaxKind::SDL_KW };
  [select] => { $ crate::EqlSyntaxKind::SELECT_KW };
  [sequence] => { $ crate::EqlSyntaxKind::SEQUENCE_KW };
  [set] => { $ crate::EqlSyntaxKind::SET_KW };
  [single] => { $ crate::EqlSyntaxKind::SINGLE_KW };
  [start] => { $ crate::EqlSyntaxKind::START_KW };
  [std] => { $ crate::EqlSyntaxKind::STD_KW };
  [str] => { $ crate::EqlSyntaxKind::STR_KW };
  [to] => { $ crate::EqlSyntaxKind::TO_KW };
  [true] => { $ crate::EqlSyntaxKind::TRUE_KW };
  [tuple] => { $ crate::EqlSyntaxKind::TUPLE_KW };
  [typeof] => { $ crate::EqlSyntaxKind::TYPEOF_KW };
  [update] => { $ crate::EqlSyntaxKind::UPDATE_KW };
  [uuid] => { $ crate::EqlSyntaxKind::UUID_KW };
  [variadic] => { $ crate::EqlSyntaxKind::VARIADIC_KW };
  [with] => { $ crate::EqlSyntaxKind::WITH_KW };
  [abort] => { $ crate::EqlSyntaxKind::RESERVED_ABORT_KW };
  [applied] => { $ crate::EqlSyntaxKind::RESERVED_APPLIED_KW };
  [assignment] => { $ crate::EqlSyntaxKind::RESERVED_ASSIGNMENT_KW };
  [cardinality] => { $ crate::EqlSyntaxKind::RESERVED_CARDINALITY_KW };
  [cast] => { $ crate::EqlSyntaxKind::RESERVED_CAST_KW };
  [committed] => { $ crate::EqlSyntaxKind::RESERVED_COMMITTED_KW };
  [config] => { $ crate::EqlSyntaxKind::RESERVED_CONFIG_KW };
  [conflict] => { $ crate::EqlSyntaxKind::RESERVED_CONFLICT_KW };
  [constraint] => { $ crate::EqlSyntaxKind::RESERVED_CONSTRAINT_KW };
  [cube] => { $ crate::EqlSyntaxKind::RESERVED_CUBE_KW };
  [current] => { $ crate::EqlSyntaxKind::RESERVED_CURRENT_KW };
  [database] => { $ crate::EqlSyntaxKind::RESERVED_DATABASE_KW };
  [declare] => { $ crate::EqlSyntaxKind::RESERVED_DECLARE_KW };
  [default] => { $ crate::EqlSyntaxKind::RESERVED_DEFAULT_KW };
  [deferrable] => { $ crate::EqlSyntaxKind::RESERVED_DEFERRABLE_KW };
  [deferred] => { $ crate::EqlSyntaxKind::RESERVED_DEFERRED_KW };
  [delegated] => { $ crate::EqlSyntaxKind::RESERVED_DELEGATED_KW };
  [deny] => { $ crate::EqlSyntaxKind::RESERVED_DENY_KW };
  [empty] => { $ crate::EqlSyntaxKind::RESERVED_EMPTY_KW };
  [except] => { $ crate::EqlSyntaxKind::RESERVED_EXCEPT_KW };
  [expression] => { $ crate::EqlSyntaxKind::RESERVED_EXPRESSION_KW };
  [extension] => { $ crate::EqlSyntaxKind::RESERVED_EXTENSION_KW };
  [final] => { $ crate::EqlSyntaxKind::RESERVED_FINAL_KW };
  [from] => { $ crate::EqlSyntaxKind::RESERVED_FROM_KW };
  [function] => { $ crate::EqlSyntaxKind::RESERVED_FUNCTION_KW };
  [future] => { $ crate::EqlSyntaxKind::RESERVED_FUTURE_KW };
  [implicit] => { $ crate::EqlSyntaxKind::RESERVED_IMPLICIT_KW };
  [infix] => { $ crate::EqlSyntaxKind::RESERVED_INFIX_KW };
  [inheritable] => { $ crate::EqlSyntaxKind::RESERVED_INHERITABLE_KW };
  [instance] => { $ crate::EqlSyntaxKind::RESERVED_INSTANCE_KW };
  [into] => { $ crate::EqlSyntaxKind::RESERVED_INTO_KW };
  [isolation] => { $ crate::EqlSyntaxKind::RESERVED_ISOLATION_KW };
  [migration] => { $ crate::EqlSyntaxKind::RESERVED_MIGRATION_KW };
  [named] => { $ crate::EqlSyntaxKind::RESERVED_NAMED_KW };
  [object] => { $ crate::EqlSyntaxKind::RESERVED_OBJECT_KW };
  [of] => { $ crate::EqlSyntaxKind::RESERVED_OF_KW };
  [only] => { $ crate::EqlSyntaxKind::RESERVED_ONLY_KW };
  [onto] => { $ crate::EqlSyntaxKind::RESERVED_ONTO_KW };
  [operator] => { $ crate::EqlSyntaxKind::RESERVED_OPERATOR_KW };
  [optionality] => { $ crate::EqlSyntaxKind::RESERVED_OPTIONALITY_KW };
  [order] => { $ crate::EqlSyntaxKind::RESERVED_ORDER_KW };
  [orphan] => { $ crate::EqlSyntaxKind::RESERVED_ORPHAN_KW };
  [overloaded] => { $ crate::EqlSyntaxKind::RESERVED_OVERLOADED_KW };
  [owned] => { $ crate::EqlSyntaxKind::RESERVED_OWNED_KW };
  [package] => { $ crate::EqlSyntaxKind::RESERVED_PACKAGE_KW };
  [populate] => { $ crate::EqlSyntaxKind::RESERVED_POPULATE_KW };
  [postfix] => { $ crate::EqlSyntaxKind::RESERVED_POSTFIX_KW };
  [prefix] => { $ crate::EqlSyntaxKind::RESERVED_PREFIX_KW };
  [property] => { $ crate::EqlSyntaxKind::RESERVED_PROPERTY_KW };
  [proposed] => { $ crate::EqlSyntaxKind::RESERVED_PROPOSED_KW };
  [pseudo] => { $ crate::EqlSyntaxKind::RESERVED_PSEUDO_KW };
  [read] => { $ crate::EqlSyntaxKind::RESERVED_READ_KW };
  [reject] => { $ crate::EqlSyntaxKind::RESERVED_REJECT_KW };
  [release] => { $ crate::EqlSyntaxKind::RESERVED_RELEASE_KW };
  [rename] => { $ crate::EqlSyntaxKind::RESERVED_RENAME_KW };
  [required] => { $ crate::EqlSyntaxKind::RESERVED_REQUIRED_KW };
  [reset] => { $ crate::EqlSyntaxKind::RESERVED_RESET_KW };
  [restrict] => { $ crate::EqlSyntaxKind::RESERVED_RESTRICT_KW };
  [rewrite] => { $ crate::EqlSyntaxKind::RESERVED_REWRITE_KW };
  [role] => { $ crate::EqlSyntaxKind::RESERVED_ROLE_KW };
  [roles] => { $ crate::EqlSyntaxKind::RESERVED_ROLES_KW };
  [rollup] => { $ crate::EqlSyntaxKind::RESERVED_ROLLUP_KW };
  [scalar] => { $ crate::EqlSyntaxKind::RESERVED_SCALAR_KW };
  [schema] => { $ crate::EqlSyntaxKind::RESERVED_SCHEMA_KW };
  [serializable] => { $ crate::EqlSyntaxKind::RESERVED_SERIALIZABLE_KW };
  [session] => { $ crate::EqlSyntaxKind::RESERVED_SESSION_KW };
  [source] => { $ crate::EqlSyntaxKind::RESERVED_SOURCE_KW };
  [superuser] => { $ crate::EqlSyntaxKind::RESERVED_SUPERUSER_KW };
  [system] => { $ crate::EqlSyntaxKind::RESERVED_SYSTEM_KW };
  [target] => { $ crate::EqlSyntaxKind::RESERVED_TARGET_KW };
  [ternary] => { $ crate::EqlSyntaxKind::RESERVED_TERNARY_KW };
  [text] => { $ crate::EqlSyntaxKind::RESERVED_TEXT_KW };
  [then] => { $ crate::EqlSyntaxKind::RESERVED_THEN_KW };
  [transaction] => { $ crate::EqlSyntaxKind::RESERVED_TRANSACTION_KW };
  [unless] => { $ crate::EqlSyntaxKind::RESERVED_UNLESS_KW };
  [verbose] => { $ crate::EqlSyntaxKind::RESERVED_VERBOSE_KW };
  [version] => { $ crate::EqlSyntaxKind::RESERVED_VERSION_KW };
  [view] => { $ crate::EqlSyntaxKind::RESERVED_VIEW_KW };
  [write] => { $ crate::EqlSyntaxKind::RESERVED_WRITE_KW };
  [analyze] => { $ crate::EqlSyntaxKind::RESERVED_ANALYZE_KW };
  [anyarray] => { $ crate::EqlSyntaxKind::RESERVED_ANYARRAY_KW };
  [begin] => { $ crate::EqlSyntaxKind::RESERVED_BEGIN_KW };
  [case] => { $ crate::EqlSyntaxKind::RESERVED_CASE_KW };
  [check] => { $ crate::EqlSyntaxKind::RESERVED_CHECK_KW };
  [deallocate] => { $ crate::EqlSyntaxKind::RESERVED_DEALLOCATE_KW };
  [discard] => { $ crate::EqlSyntaxKind::RESERVED_DISCARD_KW };
  [do] => { $ crate::EqlSyntaxKind::RESERVED_DO_KW };
  [end] => { $ crate::EqlSyntaxKind::RESERVED_END_KW };
  [execute] => { $ crate::EqlSyntaxKind::RESERVED_EXECUTE_KW };
  [explain] => { $ crate::EqlSyntaxKind::RESERVED_EXPLAIN_KW };
  [fetch] => { $ crate::EqlSyntaxKind::RESERVED_FETCH_KW };
  [get] => { $ crate::EqlSyntaxKind::RESERVED_GET_KW };
  [grant] => { $ crate::EqlSyntaxKind::RESERVED_GRANT_KW };
  [import] => { $ crate::EqlSyntaxKind::RESERVED_IMPORT_KW };
  [listen] => { $ crate::EqlSyntaxKind::RESERVED_LISTEN_KW };
  [load] => { $ crate::EqlSyntaxKind::RESERVED_LOAD_KW };
  [lock] => { $ crate::EqlSyntaxKind::RESERVED_LOCK_KW };
  [match] => { $ crate::EqlSyntaxKind::RESERVED_MATCH_KW };
  [move] => { $ crate::EqlSyntaxKind::RESERVED_MOVE_KW };
  [notify] => { $ crate::EqlSyntaxKind::RESERVED_NOTIFY_KW };
  [over] => { $ crate::EqlSyntaxKind::RESERVED_OVER_KW };
  [prepare] => { $ crate::EqlSyntaxKind::RESERVED_PREPARE_KW };
  [partition] => { $ crate::EqlSyntaxKind::RESERVED_PARTITION_KW };
  [raise] => { $ crate::EqlSyntaxKind::RESERVED_RAISE_KW };
  [refresh] => { $ crate::EqlSyntaxKind::RESERVED_REFRESH_KW };
  [reindex] => { $ crate::EqlSyntaxKind::RESERVED_REINDEX_KW };
  [revoke] => { $ crate::EqlSyntaxKind::RESERVED_REVOKE_KW };
  [when] => { $ crate::EqlSyntaxKind::RESERVED_WHEN_KW };
  [window] => { $ crate::EqlSyntaxKind::RESERVED_WINDOW_KW };
  [never] => { $ crate::EqlSyntaxKind::RESERVED_NEVER_KW };
  [asc] => { $ crate::EqlSyntaxKind::EDGE_ASC_KW };
  [desc] => { $ crate::EqlSyntaxKind::EDGE_DESC_KW };
  [union] => { $ crate::EqlSyntaxKind::EDGE_UNION_KW };
  [savepoint] => { $ crate::EqlSyntaxKind::EDGE_SAVEPOINT_KW };
  [rollback] => { $ crate::EqlSyntaxKind::EDGE_ROLLBACK_KW };
  [annotation] => { $ crate::EqlSyntaxKind::SDL_ANNOTATION_KW };
  [link] => { $ crate::EqlSyntaxKind::SDL_LINK_KW };
  [multi] => { $ crate::EqlSyntaxKind::SDL_MULTI_KW };
  [type] => { $ crate::EqlSyntaxKind::SDL_TYPE_KW };
  [using] => { $ crate::EqlSyntaxKind::SDL_USING_KW };
  [volatility] => { $ crate::EqlSyntaxKind::SDL_VOLATILITY_KW };
  [after] => { $ crate::EqlSyntaxKind::DDL_AFTER_KW };
  [alter] => { $ crate::EqlSyntaxKind::DDL_ALTER_KW };
  [before] => { $ crate::EqlSyntaxKind::DDL_BEFORE_KW };
  [create] => { $ crate::EqlSyntaxKind::DDL_CREATE_KW };
  [drop] => { $ crate::EqlSyntaxKind::DDL_DROP_KW };
  [first] => { $ crate::EqlSyntaxKind::DDL_FIRST_KW };
  [last] => { $ crate::EqlSyntaxKind::DDL_LAST_KW };
  [ident] => { $ crate::EqlSyntaxKind::IDENT };
  [EOF] => { $ crate::EqlSyntaxKind::EOF };
  [#] => { $ crate::EqlSyntaxKind::HASH };
}
