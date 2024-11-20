use proc_macro::TokenStream;

mod input;

/// The conflicting macro lets you pass a set of feature gated expressions that transitively conflict with
/// each other. (ie. a feature conflicts with the powerset of all other features in the block)
///
/// You may have expressions gated by features a, b, and c, but theyre all mutually
/// exclusive. To handle this macro will throw an error at compile time. 
///
/// # Example
///
/// ```ignore
///     fn new() -> &'static str {
///         conflicting! {
///             a => {
///                 "a"
///             },
///             b => {
///                 "b"
///             },
///             c => {
///                 "c"
///             },
///         }
///     }
/// ```
///
/// This will generate something like:
/// ```ignore
///     fn new() -> &'static str {
///         #[cfg(feature = "a")]
///         {
///             "a"
///             
///             #[cfg(feature = "b")]
///             compile_error!("a conflicts with b");
///
///             #[cfg(feature = "c")]
///             compile_error!("a conflicts with c");
///         }
///
///         #[cfg(feature = "b")]
///         {
///             "b"

///             #[cfg(feature = "a")]
///             compile_error!("b conflicts with a");
///
///             #[cfg(feature = "c")]
///             compile_error!("b conflicts with c");
///         }
///
///         #[cfg(feature = "c")]
///         {
///             "c"
///
///             #[cfg(feature = "a")]
///             compile_error!("c conflicts with a");
///
///             #[cfg(feature = "b")]
///             compile_error!("c conflicts with b");
///         }
///     }
/// ```   
#[proc_macro]
pub fn conflicting(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as input::ConflictingInput);

    let out: proc_macro2::TokenStream = input.0.iter().map(|gated_expr| {
        let feature = &gated_expr.feature;
        let expr = &gated_expr.expr;

        let conflicting_cases = input.0.iter()
            .filter(|other| other.feature != *feature)
            .map(|other| {
                let other_feature = &other.feature;
                quote::quote! {
                    #[cfg(feature = #other_feature)]
                    {
                        compile_error!(concat!(stringify!(#feature), " conflicts with ", stringify!(#other_feature)));
                    }
                }
            });

        quote::quote! {
            #[cfg(feature = #feature)]
            {
                #expr 
                #(#conflicting_cases)*
            }
        }
    })
    .collect();

    out.into()
}
