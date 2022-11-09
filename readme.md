# eql

> an experimental language service protocol for [`edgedb`](https://www.edgedb.com).

## Introduction

EdgeDB is a great database with a query language that makes a lot of sense to me. Right now though the editor tooling isn't mature.

This is my attempt at creating a language service protocol for EdgeDB which is seperate from their current tooling. One advantage of this separation is better language support without the need for running an edgedb instance in the background.

Also by using rust to build a parser, lexer and language model it should be possible to build some exciting tooling.

Some potential tools.

- full autocomplete for `.edgeql` queries derived from the workspaces `.esdl` schema files.
- full autocomplete for migration files to help with difficult migrations
-

## Contributing

Make sure [`deno`](https://deno.land/manual@v1.26.2/getting_started/installation), [`rustup`](https://rustup.rs/) and [`edgedb`](https://www.edgedb.com/docs/intro/quickstart#installation) are installed.
