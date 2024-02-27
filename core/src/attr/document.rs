use quote::quote;
use syn::{parse2, Path};

use super::{impl_parse, parse_assign};

pub struct DocAttr {
    pub validator: Option<Path>,
    pub crate_path: Path,
    pub formatter: Option<Path>,
}

impl Default for DocAttr {
    fn default() -> Self {
        Self {
            validator: None,
            crate_path: parse2::<Path>(quote!(document_validator)).expect("valid path"),
            formatter: None,
        }
    }
}

impl_parse! {
    DocAttr(input, output) {
        "validator" => {
            output.validator = Some(parse_assign::<Path>(input)?);
        },
        "crate" => {
            output.crate_path = parse_assign::<Path>(input)?;
        },
        "formatter" => {
            output.formatter = Some(parse_assign::<Path>(input)?);
        }
    }
}
