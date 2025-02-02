//! Rust macro to parse a JSON file at compile time and compile it into the
//! program as a `serde_json::Value`.
//!
//! Example &mdash; supplying a JSON file as context inside a [MiniJinja]
//! template:
//!
//! [MiniJinja]: https://github.com/mitsuhiko/minijinja
//!
//! ```rust
//! # macro_rules! include_json {
//! #     (concat!(env!("CARGO_MANIFEST_DIR"), $path:expr)) => {
//! #         ::include_json::include_json!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples", $path))
//! #     };
//! # }
//! #
//! # macro_rules! include_str {
//! #     ($path:expr) => {
//! #         ::std::include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/", $path))
//! #     };
//! # }
//! #
//! use include_json::include_json;
//!
//! fn main() {
//!     let pkg = include_json!(concat!(env!("CARGO_MANIFEST_DIR"), "/package.json"));
//!
//!     let mut env = minijinja::Environment::new();
//!     env.add_template("example", include_str!("example.jinja")).unwrap();
//!     let tmpl = env.get_template("example").unwrap();
//!     println!("{}", tmpl.render(minijinja::context!(pkg)).unwrap());
//! }
//! ```

#![doc(html_root_url = "https://docs.rs/include_json/0.0.0")]

use macro_string::MacroString;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use serde_json::Value;
use std::fs;
use std::path::Path;
use syn::{parse_macro_input, Error, Result};

#[proc_macro]
pub fn include_json(input: TokenStream) -> TokenStream {
    let MacroString(path) = parse_macro_input!(input);
    do_include_json(&path)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn do_include_json(path: &str) -> Result<TokenStream2> {
    let path = Path::new(path);
    if path.is_relative() {
        return Err(Error::new(
            Span::call_site(),
            "a relative path is not supported; use `include_json!(concat!(env!(\"CARGO_MANIFEST_DIR\"), ...))`",
        ));
    }

    let content = match fs::read(path) {
        Ok(content) => content,
        Err(err) => {
            return Err(Error::new(
                Span::call_site(),
                format!("{} {}", err, path.display()),
            ));
        }
    };

    let json: Value = match serde_json::from_slice(&content) {
        Ok(json) => json,
        Err(err) => return Err(Error::new(Span::call_site(), err)),
    };

    Ok(PrintValue(&json).into_token_stream())
}

struct PrintValue<'a>(&'a Value);

impl ToTokens for PrintValue<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self.0 {
            Value::Null => tokens.extend(quote!(::serde_json::Value::Null)),
            Value::Bool(b) => tokens.extend(quote!(::serde_json::Value::Bool(#b))),
            Value::Number(n) => {
                let repr = n.to_string();
                tokens.extend(quote! {
                    ::serde_json::Value::Number(
                        ::core::str::FromStr::from_str(#repr).unwrap()
                    )
                });
            }
            Value::String(s) => {
                tokens.extend(quote! {
                    ::serde_json::Value::String(::core::convert::From::from(#s))
                });
            }
            Value::Array(vec) => {
                if vec.is_empty() {
                    tokens.extend(quote! {
                        ::serde_json::Value::Array(::core::default::Default::default())
                    });
                } else {
                    let elements = vec.iter().map(PrintValue);
                    tokens.extend(quote! {
                        ::serde_json::Value::Array(vec![#(#elements),*])
                    });
                }
            }
            Value::Object(map) => {
                if map.is_empty() {
                    tokens.extend(quote! {
                        ::serde_json::Value::Object(::serde_json::Map::new())
                    });
                } else {
                    let len = map.len();
                    let keys = map.keys();
                    let values = map.values().map(PrintValue);
                    tokens.extend(quote! {
                        ::serde_json::Value::Object({
                            let mut object = ::serde_json::Map::with_capacity(#len);
                            #(
                                let _ = object.insert(::core::convert::From::from(#keys), #values);
                            )*
                            object
                        })
                    });
                }
            }
        }
    }
}
