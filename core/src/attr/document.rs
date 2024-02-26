use quote::quote;
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    parse2, parse_str, Attribute, Error, Ident, Path, Result, Token,
};

use super::parse_assign_str;

pub struct DocAttr {
    pub validator: Path,
    pub crate_path: Path,
    pub formatter: Option<Path>,
}

impl TryFrom<&Attribute> for DocAttr {
    type Error = Error;

    fn try_from(attr: &Attribute) -> Result<Self> {
        attr.parse_args()
    }
}

impl Default for DocAttr {
    fn default() -> Self {
        Self {
            validator: parse2::<Path>(quote!(crate)).expect("valid path"),
            crate_path: parse2::<Path>(quote!(document_validator)).expect("valid path"),
            formatter: None,
        }
    }
}

impl Parse for DocAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut out = Self::default();
        loop {
            let span = input.span();
            let key: Ident = input.call(IdentExt::parse_any)?;
            match key.to_string().as_str() {
                "validator" => {
                    out.validator = parse_str::<Path>(&parse_assign_str(input)?)
                        .map_err(|e| Error::new(span, e.to_string()))?;
                }
                "crate" => {
                    out.crate_path = parse_str::<Path>(&parse_assign_str(input)?)
                        .map_err(|e| Error::new(span, e.to_string()))?;
                }
                "formatter" => {
                    out.formatter = Some(
                        parse_str::<Path>(&parse_assign_str(input)?)
                            .map_err(|e| Error::new(span, e.to_string()))?,
                    );
                }
                x => {
                    return Err(Error::new(
                        input.span(),
                        format!("unexpected attribute {x}"),
                    ))
                }
            }

            if input.is_empty() {
                break;
            }

            input.parse::<Token![,]>()?;
        }

        Ok(out)
    }
}
