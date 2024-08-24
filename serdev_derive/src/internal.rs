mod target;
mod validate;
mod bound;

use self::target::Target;
use self::validate::Validate;

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{parse::Parse, punctuated::Punctuated, token, Attribute, Error, GenericParam, Generics, Ident, Item, ItemEnum, ItemStruct, Lifetime, LifetimeDef, LitStr, WhereClause, WherePredicate};


fn build_serde_attribute<T: ToTokens>(directives: impl IntoIterator<Item = T>) -> Attribute {
    let directives = directives.into_iter();
    Attribute {
        pound_token:   token::Pound::default(),
        style:         syn::AttrStyle::Outer,
        bracket_token: token::Bracket::default(),
        path:          syn::parse_str("serde").unwrap(),
        tokens:        quote![( #(#directives),* )]
    }
}

pub(super) fn Serialize(input: TokenStream) -> Result<TokenStream, Error> {
    let mut target = syn::parse2::<Target>(input.clone())?;

    let generics = target.generics().clone();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut serde_directives = target.remove_serde_directives()?;

    Ok(match serde_directives.iter().position(
        |d| d.to_string().starts_with("validate")
    ) {
        Some(validator_index) => {
            let validate = serde_directives.remove(validator_index);
            let validate = syn::parse2::<Validate>(validate)?;

            let mut proxy = target.clone();
            *proxy.ident_mut() = format_ident!("__serdev_proxy_Serialize_{}__", target.ident());
            proxy .attrs_mut().push(build_serde_attribute(serde_directives.clone()));
            target.attrs_mut().push(build_serde_attribute(serde_directives));

            let proxy_ident  = proxy.ident();
            let target_ident = target.ident();

            let mut where_clause = where_clause.map(ToOwned::to_owned);
            ↓
            __HANDLE_BOUNDS__

            quote! {
                const _: () = {
                    #[derive(::serdev::__private__::serde::Serialize)]
                    #[serde(crate = "::serdev::__private__::serde")]
                    #[allow(non_camel_case_types)]
                    #proxy

                    impl #impl_generics ::serdev::__private__::serde::Serialize for #target #ty_generics
                    where #where_clause {
                        #[inline]
                        fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
                        where S: ::serdev::__private__::serde::Serializer {
                            self.0.map_err(::serdev::__private__::serde::ser::Error::custom)?
                                .serialize(serializer)
                        }
                    }
                };
            }
        }

        None => {
            quote! {
                #[derive(::serdev::__private__::serde::Serialize)]
                #[serde(crate = "::serdev::__private__::serde")]
                #[::serdev::__private__::consume]
                #target
            }
        }
    })
}

pub(super) fn Deserialize(input: TokenStream) -> Result<TokenStream, Error> {
    let mut target = syn::parse2::<Target>(input.clone())?;

    let generics = target.generics().clone();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut serde_directives = target.remove_serde_directives()?;

    Ok(match serde_directives.iter().position(
        |d| d.to_string().starts_with("validate")
    ) {
        Some(validator_index) => {
            let validate = serde_directives.remove(validator_index);
            let validate = syn::parse2::<Validate>(validate)?;

            let mut proxy = target.clone();
            *proxy.ident_mut() = format_ident!("__serdev_proxy_Serialize_{}__", target.ident());
            proxy .attrs_mut().push(build_serde_attribute(serde_directives.clone()));
            target.attrs_mut().push(build_serde_attribute(serde_directives));

            let target_ident = target.ident();
            let proxy_ident  = proxy.ident();

            let proxy_type_lit = LitStr::new(
                &quote!(#proxy_ident #ty_generics).to_string(),
                Span::call_site()
            );
            let validate_fn = validate.function().value();
            let (error_ty, e_as_error_ty) = match validate.error() {
                Some(ty) => {let ty = ty.value();
                    (quote! {#ty}, quote! {e})
                }
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

                    impl #impl_generics ::core::convert::TryFrom<#proxy_ident #ty_generics>
                    for #target_ident #ty_generics
                        #where_clause
                    {
                        type Error = #error_ty;

                        #[inline]
                        fn try_from(proxy: #proxy_ident #ty_generics) -> ::core::result::Result<Self, Self::Error> {
                            let this = unsafe {::core::mem::transmute(proxy)};
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
