use std::env;
use std::path::PathBuf;

use hex::ToHex;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use proc_macro2::Span;
use quote::quote;
use syn::parse_macro_input;
use syn::LitStr;
use syn::Type;
use walkdir::WalkDir;

enum PathComponent {
  Static(String),
  Param(String),
  TypedParam(String, String),
}

struct Page {
  components: Vec<PathComponent>,
  fs_path: PathBuf,
}

impl Page {
  fn mangled_mod_name(&self) -> Ident {
    let component_parts = self.components.iter().map(|component| {
      match component {
        PathComponent::Static(val) => format!("__s{}", val.encode_hex::<String>()),
        PathComponent::Param(name) => format!("__p{}", name.encode_hex::<String>()),
        PathComponent::TypedParam(name, ty) => {
          format!(
            "__t{}_{}",
            name.encode_hex::<String>(),
            ty.encode_hex::<String>()
          )
        }
      }
    });

    let ident = component_parts.collect::<Vec<_>>().join("");

    Ident::new(
      if ident == "" { "index" } else { &ident },
      Span::call_site(),
    )
  }

  fn path(&self) -> String {
    let path = self
      .components
      .iter()
      .map(|component| {
        match component {
          PathComponent::Static(val) => format!("/{}", val),
          PathComponent::Param(name) | PathComponent::TypedParam(name, _) => {
            format!("/:{}", name)
          }
        }
      })
      .collect::<String>();

    if path.is_empty() {
      "/".to_owned()
    } else {
      path
    }
  }
}

#[proc_macro]
pub fn router(input: TokenStream) -> TokenStream {
  let layout_section = if input.is_empty() {
    quote! {
        page
    }
  } else {
    let input = parse_macro_input!(input as Type);

    quote! {
        {
            use super::*;
            html! {
                < #input >
                    {page}
                </ #input >
            }
        }
    }
  };

  let (pages, has_404) = read_pages();
  let page_mods = pages.iter().map(|page| {
    let mangled = page.mangled_mod_name();
    let path = LitStr::new(&page.fs_path.to_string_lossy(), Span::call_site());
    quote! {
        #[path = #path]
        pub(crate) mod #mangled;
    }
  });

  let routes = pages.iter().map(|page| {
    let mangled = page.mangled_mod_name();
    // println!("{}", page.path());
    let path = LitStr::new(&page.path(), Span::call_site());
    let params = page.components.iter().filter_map(|component| {
      match component {
        PathComponent::Static(_) => None,
        PathComponent::Param(name) => {
          let ident = Ident::new(name, Span::call_site());

          Some(quote! {
              #ident : String,
          })
        }
        PathComponent::TypedParam(name, ty) => {
          let ident = Ident::new(name, Span::call_site());
          let ty = syn::parse_str::<syn::Type>(ty).unwrap();

          Some(quote! {
              #ident : #ty,
          })
        }
      }
    });

    quote! {
        #[at(#path)]
        #mangled {
            #(#params)*
        },
    }
  });

  let route_matchers = pages.iter().map(|page| {
    let mangled = page.mangled_mod_name();
    let param_idents = page
      .components
      .iter()
      .filter_map(|component| {
        match component {
          PathComponent::Static(_) => None,
          PathComponent::Param(name) => {
            let ident = Ident::new(name, Span::call_site());

            Some(quote! {
                #ident
            })
          }
          PathComponent::TypedParam(name, _) => {
            let ident = Ident::new(name, Span::call_site());

            Some(quote! {
                #ident
            })
          }
        }
      })
      .collect::<Vec<_>>();
    quote! {
        Route::#mangled {
            #(
                #param_idents
            ),*
        } => html! {
            <super::pages::#mangled::Page
                #(
                    #param_idents = { #param_idents .clone() }
                )*
            />
        },
    }
  });

  let mod_404 = if has_404 {
    quote! {
        #[path = "404.rs"]
        pub mod not_found;
    }
  } else {
    quote! {}
  };

  let route_404 = if has_404 {
    quote! {
        #[not_found]
        #[at("/404")]
        NotFound,
    }
  } else {
    quote! {}
  };

  let match_404 = if has_404 {
    quote! {
        Route::NotFound => html! {
            <super::pages::not_found::Page />
        },
    }
  } else {
    quote! {}
  };

  let tokens = quote! {
    mod pages {
      #(
        #page_mods
      )*

      #mod_404
    }

    mod router {
      use ::yew::prelude::*;
      use ::yew_router::prelude::*;

      mod route {
        use super::super::*;

        #[derive(Clone, yew_router::prelude::Routable, PartialEq)]
        pub enum Route {
          #(
              #routes
          )*

          #route_404
        }
      }

      use route::Route;

      fn switch(routes: &Route) -> Html {
        let page = match routes {
          #(
              #route_matchers
          )*
          #match_404
        };

        #layout_section
      }

      #[function_component(Router)]
      pub fn router() -> Html {
        html! {
          <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
          </BrowserRouter>
        }
      }

      const __DETECT_CHANGES: &'static str = include_str!(concat!(env!("OUT_DIR"), "/maso_router_changed.txt"));
    }
  };

  tokens.into()
}

fn read_pages() -> (Vec<Page>, bool) {
  let manifest_dir: PathBuf = env::var("CARGO_MANIFEST_DIR")
    .expect("Cargo should set CARGO_MANIFEST_DIR")
    .into();

  let pages_dir = manifest_dir.join("src/pages");

  let mut pages = Vec::new();
  let mut has_404 = false;

  for entry in WalkDir::new(&pages_dir) {
    let entry = entry.unwrap();

    if entry.file_type().is_file() {
      let file_path = entry
        .path()
        .strip_prefix(&pages_dir)
        .expect("Path should be inside pages directory");
      let mut components = file_path
        .components()
        .map(|component| {
          let component = component.as_os_str().to_string_lossy();
          let component = component.trim_end_matches(".rs");

          if component.starts_with('[') && component.ends_with(']') {
            let param_name = &component[1..(component.len() - 1)];

            match param_name.split_once(':') {
              Some((param_name, ty)) => {
                PathComponent::TypedParam(param_name.to_owned(), ty.to_owned())
              }
              None => PathComponent::Param(param_name.to_owned()),
            }
          } else {
            PathComponent::Static(component.to_string())
          }
        })
        .collect::<Vec<_>>();

      if let Some(PathComponent::Static(val)) = components.last() {
        if val == "index" {
          components.pop();
        }
      }

      if components.len() == 1 {
        if let PathComponent::Static(v) = &components[0] {
          if v == "404" {
            has_404 = true;
            continue;
          }
        }
      }

      pages.push(Page {
        components,
        fs_path: file_path.to_owned(),
      })
    }
  }

  (pages, has_404)
}
