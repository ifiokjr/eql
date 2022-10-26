//! Filesystem-based routing for Yew.
//!
//! Simply create a `src/pages/` folder and place rust source files inside it at
//! the desired path. Each source file must expose a component called `Page`
//! that will be rendered at that path. URL parameters will be passed
//! automatically as props.
//!
//! Some examples:
//!
//! * `pages/index.rs` will be served at `/`
//! * `pages/foo.rs` will be served at `/foo`
//! * `pages/blog/index.rs` will be served at `/blog`
//! * `pages/blog/[id].rs` will be served at `/blog/<any string>` and the page
//!   component will receive an `id` prop of type `String`.
//! * `pages/blog/[id:u64].rs` will be served at `/blog/<any u64>` and the page
//!   component will receive an `id` prop of type `u64`.
//!
//! Typed parameters work with anythring that implements `FromStr`. If the
//! conversion fails, the route will not match.
//!
//! TODO:
//!
//! * Implement not found handling.

// Taken from: https://docs.rs/crate/yew-fs-router/0.4.0/source/src/lib.rs

use std::env;
use std::fs;
use std::path::PathBuf;

/// Use in the root of your crate in order to include all the modules and
/// generate the router.
///
/// It takes either no arguments, or a single type for a component that will be
/// used to wrap all pages.
///
/// The router is a component in `crate::router::Router` with no props.
pub use maso_router_macro::router;

/// Call in `build.rs`. Needed to make proper change detection work when a new
/// page is created.
pub fn change_detection() {
  println!("cargo:rerun-if-changed=src/pages");

  let out_dir: PathBuf = env::var("OUT_DIR")
    .expect("Cargo should set CARGO_MANIFEST_DIR")
    .into();

  let changed_path = out_dir.join("maso_router_changed.txt");

  fs::write(changed_path, "changed").unwrap();
}

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub children: Children,
}

#[function_component(BlankLayout)]
pub fn blank_layout(props: &Props) -> Html {
  html! {
      {for props.children.iter()}
  }
}
