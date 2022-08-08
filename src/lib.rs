//! puball is a library to help you get rid of some situation that you have to endlessly
//! add `pub` keyword to a huge struct.
//!
//! # Example
//!
//! ```toml
//! [dependencies]
//! puball = "0.1.1"
//! ```
//!
//! ```rust
//! mod iphone {
//!     use puball::pub_all;
//!
//!     pub_all!{
//!         struct NoPrivacy {
//!             a: i32,
//!             b: String,
//!             c: bool,
//!         }
//!     }
//! }
//!
//! use iphone::NoPrivacy;
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
    name: Ident,
    fields: Punctuated<Field, Token![,]>,
}

impl Parse for PublicAll {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Visibility>()?;
        input.parse::<Token![struct]>()?;
        let name: Ident = input.parse()?;

        let content;
        braced!(content in input);
        let fields = content.parse_terminated(Field::parse)?;

        Ok(Self { name, fields })
    }
}

/// Transforming a non-public struct to a public struct.
/// Require normal struct declaration syntax.
///
/// ## Example
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
    let PublicAll { name, fields } = parse_macro_input!(input as PublicAll);

    let recurse = fields.iter().map(|field| {
        let name = &field.name;
        let ty = &field.ty;
        quote!(
            pub #name: #ty
        )
    });

    let expanded = quote! {
        pub struct #name {
            #(#recurse,)*
        }
    };

    TokenStream::from(expanded)
}
