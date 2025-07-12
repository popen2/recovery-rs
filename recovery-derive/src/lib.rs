use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, parse_macro_input};

#[proc_macro_derive(Recovery, attributes(recovery))]
pub fn recovery_derive(tokens: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs, ident, data, ..
    } = parse_macro_input!(tokens);

    let syn::Data::Enum(data) = data else {
        panic!("Recovery only supports enums");
    };

    let default_recovery = find_recovery_attr(&attrs);

    let mut variants = vec![];
    for variant in data.variants {
        let syn::Variant {
            attrs,
            ident,
            fields,
            ..
        } = variant;

        let recovery_strategy = find_recovery_attr(&attrs)
            .or_else(|| default_recovery.clone())
            .unwrap_or_else(|| {
                panic!("Please add #[recovery(...)] for {ident} or an enum-level default")
            });

        variants.push(match fields {
            syn::Fields::Named(_) => {
                quote! { Self::#ident{..} => #recovery_strategy }
            }
            syn::Fields::Unnamed(_) => {
                quote! { Self::#ident(..) => #recovery_strategy }
            }
            syn::Fields::Unit => {
                quote! { Self::#ident => #recovery_strategy }
            }
        });
    }

    quote! {
        impl recovery::Recovery for #ident {
            fn recovery(&self) -> recovery::RecoveryStrategy {
                match self {
                    #( #variants, )*
                }
            }
        }
    }
    .into()
}

fn find_recovery_attr(attrs: &[Attribute]) -> Option<proc_macro2::TokenStream> {
    let attr = attrs.iter().find(|attr| attr.path().is_ident("recovery"))?;

    let mut result = quote! {};
    attr.parse_nested_meta(|meta| match meta.path.get_ident() {
        Some(ident) if ident == "auto" => {
            result = quote! { recovery::RecoveryStrategy::Auto };
            Ok(())
        }
        Some(ident) if ident == "manual" => {
            result = quote! { recovery::RecoveryStrategy::Manual };
            Ok(())
        }
        Some(ident) if ident == "never" => {
            result = quote! { recovery::RecoveryStrategy::Never };
            Ok(())
        }
        Some(_) | None => Err(meta.error("One of \"auto\", \"manual\" or \"never\" is required")),
    })
    .unwrap();
    Some(result)
}
