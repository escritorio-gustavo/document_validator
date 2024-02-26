//! A small library for document validation. It includes a trait called
//! `Document`, which can be derived by giving its derive attribute helper
//! the path to a validator function

#![deny(clippy::pedantic, clippy::nursery)]

#[cfg(feature = "derive")]
pub use document_validator_macros::*;

pub mod cnpj;
pub mod cpf;

/// This trait indicates that a type is a document with a defined validation
/// scheme. Such types should ideally be a newtype struct over a type that
/// implements `Deref::<str>`
pub trait Document {
    /// Creates a new valid document. This method should call `Self::validate(document)`
    /// and return `None` if validation fails.
    ///
    /// If validation succeeds this method should return `Some(Self(format_logic(document).into()))`
    fn new(document: &str) -> Option<Self>
    where
        Self: Sized;

    /// Verifies if a given string is a valid document acording to some
    /// validation logic
    fn validate(document: &str) -> bool
    where
        Self: Sized;

    /// Returns a reference to the string representation of the document
    fn as_str(&self) -> &str;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore = "This test doesn't need to run, it just needs to compile"]
    fn trait_must_be_object_safe() {
        struct Foo(Box<str>);

        impl Document for Foo {
            fn new(document: &str) -> Option<Self>
            where
                Self: Sized,
            {
                Self::validate(document).then(|| Self(document.into()))
            }

            fn validate(_: &str) -> bool {
                true
            }

            fn as_str(&self) -> &str {
                &self.0
            }
        }

        let _foo = Box::new(Foo("".into())) as Box<dyn Document>;
    }
}
