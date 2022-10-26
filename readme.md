# kickjump_rust

This is an exploration of whether kickjump can be built primarily in Rust.

In order for this to be explored the following packages will need to be created.

- [ ] `skribble` - parse a rust code base for all calls to skribble and generate the css class names
- [ ] `skribble_macro` - a macro which runs at compile time to extract the class names

### `maso`

Short for masochist. This is a runtime framework for generating application backends and frontends with rust.

- pages support (src/pages) should transform to multiple entrypoints each with a single page yew router component
- code splitting is difficult with
- what if each page was generated as totally seperate wasm bundle and then the main router loads the components async (is that possible)
- for now it seems simpler to just have a single entrypoint
- the server implementation (http2) either tokio or actix. actix seems to be the most popular.

## What is KickJump?

Open source provides value to the world. Almost everything that exists today is touched at some point by open source libraries.

Our impact on productivity and opening up new frontiers to those without resources has been one of the greatest achievements of the 21st century.

The problem is that while open source delivers vast benefits to the world, it captures very little of the value it creates.

This means developers are working on solutions and libraries which make society wealthier without seeing any direct wealth returns. Indirectly, open source devlopers may boost their profiles and land lucrative roles via exposure, but the actual high value projects don't generate anything of monetary value.

This leads to the following problems:

- In order to succeed in open source, developers often juggle two full time roles. Employee work and open source work. This is unsustainable and leads to burnout.
- Projects become overwhelming for maintainers and they often make the perfectly reasonable decision to step away.
- The draw of open source work is the ability to own a project and invest coding time into that project. However, in order to support this ideal, projects have to find other avenues for income generation.

These problems can be solved by highly entrepreneurial developers who embody the characteristics of a perfectly balanced Vitruvian individual. However, this is a rare breed of developer. And by limiting open source work to companies with resources to spare, and quant level developers, we are limiting the potential of open source.

So how to we solve these problems.

The first step is to think deeply about the value proposition of open source projects and break them into different categories.

This is a rudimentary list of the different types of open source projects.

-
