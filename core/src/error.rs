pub(crate) use error_macros::syn_error;

mod error_macros {
    macro_rules! syn_error {
        ($lit: literal $(, $expr: expr)* $(,)?) => {
            return Err(
                syn::Error::new(
                    proc_macro2::Span::call_site(),
                    &format!($lit $(, $expr)*)
                )
            )
        };
        ($span: expr, $lit: literal $(, $expr: expr)* $(,)?) => {
            return Err(
                syn::Error::new(
                    $span,
                    &format!($lit $(, $expr)*)
                )
            )
        };
    }
    pub(crate) use syn_error;
}
