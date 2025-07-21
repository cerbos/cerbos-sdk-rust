use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{Attribute, Item};

/// Conditionally applies `serde(default)` only to structs when the "serde" feature is enabled
#[proc_macro_attribute]
pub fn serde_default(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input2 = proc_macro2::TokenStream::from(input);

    match serde_default_impl(input2) {
        Ok(output) => TokenStream::from(output),
        Err(error) => TokenStream::from(error.to_compile_error()),
    }
}

fn serde_default_impl(input: TokenStream2) -> syn::Result<TokenStream2> {
    let item: Item = syn::parse2(input)?;

    if let Item::Struct(mut item_struct) = item {
        item_struct.attrs.push(get_serde_de());
        item_struct.attrs.push(get_serde_default());
        Ok(item_struct.into_token_stream())
    } else {
        Ok(item.into_token_stream())
    }
}

fn get_serde_de() -> Attribute {
    syn::parse_quote! {
        #[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(rename_all = "camelCase"))]
    }
}

fn get_serde_default() -> Attribute {
    syn::parse_quote! {
        #[cfg_attr(feature = "serde", serde(default))]
    }
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

        let result = serde_default_impl(input).unwrap();
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

        let result = serde_default_impl(input).unwrap();
        let result_str = result.to_string();

        // Should not contain default for enums
        assert!(!result_str.contains("serde(default)"));
    }
}
