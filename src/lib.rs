//! puball is a library to help you get rid of some situation that you have to endlessly
//! add `pub` keyword to a huge struct.
//!
//! # Example
//!
//! ```toml
//! # Cargo.toml
//!
//! [dependencies]
//! puball = "0.1"
//! ```
//!
//! ```rust
//!
//! mod child {
//!     use puball::pub_all;
//!
//!     pub_all!{
//!         pub struct NoPrivacy {
//!             a: i32,
//!             b: String,
//!             c: bool,
//!         }
//!     }
//! }
//!
//! use child::NoPrivacy;
//!
//! let np = NoPrivacy {
//!     a: 1,
//!     b: String::new(),
//!     c: true,
//! };
//!
//! assert_eq!(1, np.a);
//! assert!(np.c);

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    braced, parse::Parse, parse::ParseStream, parse_macro_input, punctuated::Punctuated,
    Ident, Token, Type, Visibility,
};

/// Datastructure that represent a struct field be like.
struct Field {
    name: Ident,
    _c: Token![:],
    ty: Type,
}

impl Parse for Field {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            _c: input.parse()?,
            ty: input.parse()?,
        })
    }
}

/// Datastructure that represent a struct be like.
struct PublicAll {
    vis: Visibility,
    name: Ident,
    fields: Punctuated<Field, Token![,]>,
}

impl Parse for PublicAll {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let vis: Visibility = input.parse()?;
        input.parse::<Token![struct]>()?;
        let name: Ident = input.parse()?;

        let content;
        braced!(content in input);
        let fields = content.parse_terminated(Field::parse)?;

        Ok(Self { vis, name, fields })
    }
}

/// Transforming a non-public struct to a public struct.
/// Require normal struct declaration syntax.
///
/// ```
/// pub_all!{
///   pub struct Inner {
///     a: i32,
///     b: String,
///     c: bool,
///   }
/// }
/// ```
#[proc_macro]
pub fn pub_all(input: TokenStream) -> TokenStream {
    let PublicAll { vis, name, fields } = parse_macro_input!(input as PublicAll);

    let recurse = fields.iter().map(|field| {
        let name = &field.name;
        let ty = &field.ty;
        quote!(
            pub #name: #ty
        )
    });

    let expanded = quote! {
        #vis struct #name {
            #(#recurse,)*
        }
    };

    TokenStream::from(expanded)
}
