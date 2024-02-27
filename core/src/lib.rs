#![deny(clippy::pedantic, clippy::nursery)]

use std::convert::identity;

use attr::document::DocAttr;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, Error, Fields, FieldsUnnamed, ItemStruct, Result};

use crate::error::syn_error;

mod attr;
mod error;

pub fn derive_document2(input: TokenStream) -> TokenStream {
    expand_derive_document(input).map_or_else(Error::into_compile_error, identity)
}

fn expand_derive_document(input: TokenStream) -> Result<TokenStream> {
    let item = parse2::<ItemStruct>(input)?;
    let name = &item.ident;

    let Some(document_attr) = item
        .attrs
        .iter()
        .filter(|x| x.path().is_ident("document"))
        .map(DocAttr::try_from)
        .next()
    else {
        syn_error!(r#"The #[document(...)] attribute is required"#);
    };

    let DocAttr {
        validator: Some(validator),
        crate_path,
        formatter,
    } = document_attr?
    else {
        syn_error!(r#"The #[document(validator = "...")] attribute is required"#);
    };

    let Fields::Unnamed(FieldsUnnamed {
        unnamed: ref fields,
        ..
    }) = item.fields
    else {
        syn_error!("This trait may only be derived by a newtype struct");
    };

    if fields.len() != 1 {
        syn_error!("This trait may only be derived by a newtype struct");
    }

    let formatted_document = formatter.map_or(quote![document], |fmt| quote![#fmt(document)]);

    Ok(quote! {
        impl #crate_path::Document for #name {
            fn new(document: &str) -> Option<Self> where Self: Sized {
                Self::validate(document).then(|| Self(#formatted_document.into()))
            }

            fn validate(document: &str) -> bool where Self: Sized {
                #validator(document)
            }

            fn as_str(&self) -> &str {
                &self.0
            }
        }
    })
}
