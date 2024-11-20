use syn::parse::{Parse, ParseStream};

pub(crate) struct GatedExpr {
    pub(crate) feature: syn::LitStr,
    pub(crate) expr: syn::Expr,
}

pub(crate) struct ConflictingInput(pub(crate) Vec<GatedExpr>);

impl Parse for GatedExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let feature: syn::LitStr = input.parse()?;

        // Skip the fat arrow
        let _ = input.parse::<syn::Token![=>]>()?;
        let expr = input.parse()?;

        // Skip the comma
        match input.parse::<syn::Token![,]>() {
            Ok(_) => {}
            Err(_) => {
                // If were not at the last block lets throw for bad pattern.
                if !input.is_empty() {
                    return Err(syn::Error::new(
                        feature.span(),
                        "Expected comma after block.",
                    ));
                }
            }
        }

        Ok(GatedExpr { feature, expr })
    }
}

impl Parse for ConflictingInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut exprs = Vec::new();
        while !input.is_empty() {
            exprs.push(input.parse()?);
        }

        Ok(ConflictingInput(exprs))
    }
}
