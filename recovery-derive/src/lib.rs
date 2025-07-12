use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, parse_macro_input};

#[proc_macro_derive(Recovery, attributes(recovery))]
pub fn recovery_derive(tokens: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs, ident, data, ..
    } = parse_macro_input!(tokens);

    let syn::Data::Enum(data) = data else {
        panic!("#[derive(Recovery)] only supports enums");
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
            .or(default_recovery)
            .unwrap_or_else(|| {
                panic!("Please add #[recovery(...)] for {ident} or an enum-level default")
            });

        variants.push(match (fields, recovery_strategy) {
            (syn::Fields::Named(_), DeriveStrategy::Transparent) => {
                panic!("#[recovery(transparent)] is not supported for variants with named fields");
            }
            (syn::Fields::Named(_), DeriveStrategy::Auto) => {
                quote! { Self::#ident{..} => recovery::RecoveryStrategy::Auto }
            }
            (syn::Fields::Named(_), DeriveStrategy::Manual) => {
                quote! { Self::#ident{..} => recovery::RecoveryStrategy::Manual }
            }
            (syn::Fields::Named(_), DeriveStrategy::Never) => {
                quote! { Self::#ident{..} => recovery::RecoveryStrategy::Never }
            }
            (syn::Fields::Unnamed(_), DeriveStrategy::Transparent) => {
                quote! { Self::#ident(field, ..) => field.recovery() }
            }
            (syn::Fields::Unnamed(_), DeriveStrategy::Auto) => {
                quote! { Self::#ident(..) => recovery::RecoveryStrategy::Auto }
            }
            (syn::Fields::Unnamed(_), DeriveStrategy::Manual) => {
                quote! { Self::#ident(..) => recovery::RecoveryStrategy::Manual }
            }
            (syn::Fields::Unnamed(_), DeriveStrategy::Never) => {
                quote! { Self::#ident(..) => recovery::RecoveryStrategy::Never }
            }
            (syn::Fields::Unit, DeriveStrategy::Transparent) => {
                panic!("#[recovery(transparent)] is not supported for unit variants");
            }
            (syn::Fields::Unit, DeriveStrategy::Auto) => {
                quote! { Self::#ident => recovery::RecoveryStrategy::Auto }
            }
            (syn::Fields::Unit, DeriveStrategy::Manual) => {
                quote! { Self::#ident => recovery::RecoveryStrategy::Manual }
            }
            (syn::Fields::Unit, DeriveStrategy::Never) => {
                quote! { Self::#ident => recovery::RecoveryStrategy::Never }
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

fn find_recovery_attr(attrs: &[Attribute]) -> Option<DeriveStrategy> {
    let attr = attrs.iter().find(|attr| attr.path().is_ident("recovery"))?;

    let mut result = None;
    attr.parse_nested_meta(|meta| match meta.path.get_ident() {
        Some(ident) if ident == "transparent" => {
            result = Some(DeriveStrategy::Transparent);
            Ok(())
        }
        Some(ident) if ident == "auto" => {
            result = Some(DeriveStrategy::Auto);
            Ok(())
        }
        Some(ident) if ident == "manual" => {
            result = Some(DeriveStrategy::Manual);
            Ok(())
        }
        Some(ident) if ident == "never" => {
            result = Some(DeriveStrategy::Never);
            Ok(())
        }
        Some(_) | None => Err(meta
            .error("One of these is required: \"auto\", \"manual\", \"never\", \"transparent\"")),
    })
    .unwrap();
    result
}

#[derive(Clone, Copy)]
enum DeriveStrategy {
    Transparent,
    Auto,
    Manual,
    Never,
}
