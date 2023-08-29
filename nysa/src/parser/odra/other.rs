use syn::parse_quote;

/// File level attributes to mute error while compiling a contract.
/// Generating code from Solidity may result in some unusual naming conventions
/// and syntax that linter does not like.
pub(super) fn attrs() -> Vec<syn::Attribute> {
    vec![parse_quote!(#![allow(unused_braces, non_snake_case)])]
}

/// Generates code that is not a direct derivative of Solidity code.
pub(super) fn other_code() -> Vec<syn::Item> {
    path_stack_default_impl()
}

/// Generates Odra-specific implementations for PathStack.
pub(super) fn path_stack_default_impl() -> Vec<syn::Item> {
    vec![
        parse_quote! {
            impl odra::types::contract_def::Node for PathStack {
                const COUNT: u32 = 0;
                const IS_LEAF: bool = false;
            }
        },
        parse_quote! {
            impl odra::OdraItem for PathStack {
                fn is_module() -> bool {
                    false
                }
            }
        },
        parse_quote! {
            impl odra::StaticInstance for PathStack {
                fn instance<'a>(keys: &'a [&'a str]) -> (Self, &'a [&'a str]) {
                    (PathStack::default(), keys)
                }
            }
        },
        parse_quote! {
            impl odra::DynamicInstance for PathStack {
                #[allow(unused_variables)]
                fn instance(namespace: &[u8]) -> Self {
                    PathStack::default()
                }
            }
        },
    ]
}
