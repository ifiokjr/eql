# eql

Edge Query Language (eql) is currently an experimental library adding language server protocol support to EdgeDB.

The **lsp** is based on the same architecture used for [`rust-analyzer`](https://github.com/rust-lang/rust-analyzer).

There are two variants of **lsp** support.

- `.esdl` files which define the **EdgeDB Schema Definition Library**.
- `.edgeql` files which define the **EdgeDB Query Language**.

The public facing libraries in this repository are:

- `crates/eql_lsp` - the CLI which launches the language server protocol.
- `crates/eql_derive` - provides the `eql! { }` macro for inline EdgeQL statements.
- `crates/eql` - exports `eql_derive::eql` and `edgedb_client::*`.
- `vscode-eql` - the vscode extension which supports syntax highlighting, code completion, full intellisense

By default the vscode extension `eql` reads the schema at `dbschema/*.esdl` file and uses this to provide schema.
