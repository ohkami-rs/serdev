mod target;
mod validate;

use self::target::Target;
use self::validate::Validate;

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Error, LitStr};


pub(super) fn Serialize(input: TokenStream) -> Result<TokenStream, Error> {
    let mut target = syn::parse2::<Target>(input.clone())?;
    Validate::take(target.attrs_mut())?;

    Ok(quote! {
        #[derive(::serdev::__private__::serde::Serialize)]
        #[serde(crate = "::serdev::__private__::serde")]
        #[::serdev::__private__::consume]
        #target
    })
}

pub(super) fn Deserialize(input: TokenStream) -> Result<TokenStream, Error> {
    let mut target = syn::parse2::<Target>(input.clone())?;

    let generics = target.generics().clone();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    Ok(match Validate::take(target.attrs_mut())? {
        Some(validate) => {
            let proxy = target.create_proxy(format_ident!("serdev_proxy_{}", target.ident()));

            let target_ident = target.ident();
            let proxy_ident  = proxy.ident();

            let transmute_from_proxy = proxy.transmute_expr("proxy", target_ident);

            let proxy_type_lit = LitStr::new(
                &quote!(#proxy_ident #ty_generics).to_string(),
                Span::call_site()
            );

            let validate_fn = validate.function()?;
            let (error_ty, e_as_error_ty) = match validate.error()? {
                Some(ty) => (
                    quote! {#ty},
                    quote! {e}
                ),
                None => (
                    quote! {::serdev::__private__::DefaultError},
                    quote! {::serdev::__private__::default_error(e)}
                )
            };

            quote! {
                const _: () = {
                    #[derive(::serdev::__private__::serde::Deserialize)]
                    #[serde(crate = "::serdev::__private__::serde")]
                    #[allow(non_camel_case_types)]
                    #proxy

                    impl #impl_generics ::core::convert::TryFrom<#proxy_ident #ty_generics> for #target_ident #ty_generics
                        #where_clause
                    {
                        type Error = #error_ty;

                        #[inline]
                        fn try_from(proxy: #proxy_ident #ty_generics) -> ::core::result::Result<Self, Self::Error> {
                            let this = #transmute_from_proxy;
                            let _: () = #validate_fn(&this).map_err(|e| #e_as_error_ty)?;
                            Ok(this)
                        }
                    }

                    #[derive(::serdev::__private__::serde::Deserialize)]
                    #[serde(crate = "::serdev::__private__::serde")]
                    #[serde(try_from = #proxy_type_lit)]
                    #[::serdev::__private__::consume]
                    #target
                };
            }
        }

        None => {
            quote! {
                #[derive(::serdev::__private__::serde::Deserialize)]
                #[serde(crate = "::serdev::__private__::serde")]
                #[::serdev::__private__::consume]
                #target
            }
        }
    })
}
