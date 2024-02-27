use syn::{
    parse::{Parse, ParseStream},
    Error, Lit, Result, Token,
};

pub mod document;

pub fn parse_assign<T>(input: ParseStream) -> Result<T>
where
    T: Parse,
{
    input.parse::<Token![=]>()?;
    match Lit::parse(input)? {
        Lit::Str(string) => string.parse::<T>(),
        other => Err(Error::new(other.span(), "expected string")),
    }
}

pub(crate) use attr_macros::impl_parse;
mod attr_macros {
    macro_rules! impl_parse {
        ($ty: ident ($input: ident, $output: ident) { $($key: pat => $block: block),* $(,)? }) => {
            impl TryFrom<&syn::Attribute> for $ty {
                type Error = syn::Error;

                fn try_from(attr: &syn::Attribute) -> Result<Self, Self::Error> {
                    attr.parse_args()
                }
            }

            impl syn::parse::Parse for $ty {
                fn parse($input: syn::parse::ParseStream) -> syn::Result<Self> {
                    let mut $output = $ty::default();

                    loop {
                        let span = $input.span();
                        let key: syn::Ident = $input.call(syn::ext::IdentExt::parse_any)?;

                        match key.to_string().as_str() {
                            $($key => $block,)*
                            x => {
                                $crate::error::syn_error!(
                                    span,
                                    r#"Unknown attribute "{x}". Accepted attributes are: {}"#,
                                    [$(stringify!($key)),*].join(", "),
                                );
                            },
                        };

                        if $input.is_empty() {
                            break;
                        }

                        $input.parse::<syn::Token![,]>()?;
                    }

                    Ok($output)
                }
            }
        };
    }

    pub(crate) use impl_parse;
}
