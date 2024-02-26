use document_validator_core::derive_document2;
use proc_macro::TokenStream;

#[proc_macro_derive(Document, attributes(document))]
pub fn derive_document(input: TokenStream) -> TokenStream {
    derive_document2(input.into()).into()
}
