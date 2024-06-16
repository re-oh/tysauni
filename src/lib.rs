// lib.rs

extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_attribute]
pub fn record(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    if let Data::Struct(_) = input.data {
    } else {
        panic!("You can only record structs");
    }

    let name = input.ident;

    let expanded = quote! {
        impl #name {
            pub fn register_get_ident() -> &'static str {
                stringify!(#name)
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn register(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let variants = if let Data::Enum(data_enum) = input.data {
        data_enum.variants
    } else {
        panic!("Registers can only be created from Enums.");
    };

    let wrap_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let variant_fields = match &variant.fields {
            Fields::Unnamed(fields) => fields,
            _ => panic!("A Register Enum should only contain tuple Enums."),
        };

        if variant_fields.unnamed.len() != 1 {
            panic!("Each enum variant should contain exactly 1 field");
        }

        let field_type = &variant_fields.unnamed[0].ty;
        quote! {
            if obj.register_get_ident() == stringify!(#variant_name) {
                return #name::#variant_name(obj);
            }
        }
    });

    let resolve_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        quote! {
            #name::#variant_name(value) => value,
        }
    });

    let expanded = quote! {
        impl #name {
            pub fn wrap<T: Any>(obj: T) -> Self {
                #( #wrap_arms )*
                panic!("Unexpected type")
            }

            pub fn resolve<T: 'static>(self) -> T {
                match self {
                    #( #resolve_arms )*
                    _ => panic!("Unexpected type conversion"),
                }
            }
        }
    };

    TokenStream::from(expanded)
}
