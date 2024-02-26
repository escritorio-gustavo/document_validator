use syn::{
    parse::{Parse, ParseStream},
    Error, Lit, Result, Token,
};

pub mod document;

pub fn parse_assign_str(input: ParseStream) -> Result<String> {
    input.parse::<Token![=]>()?;
    match Lit::parse(input)? {
        Lit::Str(string) => Ok(string.value()),
        other => Err(Error::new(other.span(), "expected string")),
    }
}
