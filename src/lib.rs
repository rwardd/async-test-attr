use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

/// Generates the appropriate async test attribute based on feature flags:
/// - If "tokio" feature is enabled: generates `#[tokio::test]`
/// - If "futures" feature is enabled: generates `#[apply(test!)]`
#[proc_macro_attribute]
pub fn async_test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let output = quote! {
        #[cfg_attr(feature = "tokio", tokio::test)]
        #[cfg_attr(all(any(feature = "futures"), not(feature = "tokio")), apply(test!))]
        #input
    };
    output.into()
}

/// Generates the appropriate async main executor attribute based on feature flags:
/// - If "tokio" feature is enabled: generates `#[tokio::main]`
/// - If "futures" feature is enabled: generates `#[apply(main!)]`
#[proc_macro_attribute]
pub fn async_executor(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let output = quote! {
        #[cfg_attr(feature = "tokio", tokio::main)]
        #[cfg_attr(all(any(feature = "futures"), not(feature = "tokio")), macro_rules_attribute::apply(smol_macros::main!))]
        #input
    };
    output.into()
}
