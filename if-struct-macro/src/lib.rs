use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{Attribute, Item};

/// Conditionally applies `serde(default)` only to structs when the "serde" feature is enabled
#[proc_macro_attribute]
pub fn if_struct_serde_default(args: TokenStream, input: TokenStream) -> TokenStream {
    let input2 = proc_macro2::TokenStream::from(input);
    let args2 = proc_macro2::TokenStream::from(args);

    match if_struct_serde_default_impl(args2, input2) {
        Ok(output) => TokenStream::from(output),
        Err(error) => TokenStream::from(error.to_compile_error()),
    }
}

fn if_struct_serde_default_impl(
    _args: TokenStream2,
    input: TokenStream2,
) -> syn::Result<TokenStream2> {
    let item: Item = syn::parse2(input)?;

    match item {
        Item::Struct(mut item_struct) => {
            // Only add serde(default) to structs when serde feature is enabled
            add_if_struct_serde_default(&mut item_struct.attrs);
            Ok(item_struct.into_token_stream())
        }
        other => Ok(other.into_token_stream()),
    }
}

fn add_if_struct_serde_default(attrs: &mut Vec<Attribute>) {
    let new_attr: Attribute = syn::parse_quote! {
        #[cfg_attr(feature = "serde", serde(default))]
    };
    attrs.push(new_attr);
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn test_struct_gets_default() {
        let input = quote! {
            struct TestStruct {
                field: String,
            }
        };

        let result = if_struct_serde_default_impl(quote! {}, input).unwrap();
        let result_str = result.to_string().replace(" ", "");

        assert!(result_str.contains("serde(default)"));
    }

    #[test]
    fn test_enum_no_default() {
        let input = quote! {
            enum TestEnum {
                Variant1,
                Variant2,
            }
        };

        let result = if_struct_serde_default_impl(quote! {}, input).unwrap();
        let result_str = result.to_string();

        // Should not contain default for enums
        assert!(!result_str.contains("serde(default)"));
    }
}
