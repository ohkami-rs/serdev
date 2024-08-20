use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{parse::Parse, punctuated::Punctuated, token, Attribute, Error, Generics, Ident, Item, ItemEnum, ItemStruct, LitStr, Visibility};


#[derive(Clone)]
enum Target {
    Enum(ItemEnum),
    Struct(ItemStruct)
}
impl Parse for Target {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        match input.parse::<Item>()? {
            Item::Enum(e)   => Ok(Self::Enum(e)),
            Item::Struct(s) => Ok(Self::Struct(s)),
            _ => Err(Error::new(Span::call_site(), ""))
        }
    }
}
impl ToTokens for Target {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Enum(e)   => e.to_tokens(tokens),
            Self::Struct(s) => s.to_tokens(tokens)
        }
    }
}
impl Target {
    fn attrs(&self) -> &[Attribute] {
        match self {
            Self::Enum(e)   => &e.attrs,
            Self::Struct(s) => &s.attrs
        }
    }
    fn attrs_mut(&mut self) -> &mut Vec<Attribute> {
        match self {
            Self::Enum(e)   => &mut e.attrs,
            Self::Struct(s) => &mut s.attrs
        }
    }

    fn generics(&self) -> &Generics {
        match self {
            Self::Enum(e)   => &e.generics,
            Self::Struct(s) => &s.generics
        }
    }

    fn ident(&self) -> &Ident {
        match self {
            Self::Enum(e)   => &e.ident,
            Self::Struct(s) => &s.ident
        }
    }
    fn ident_mut(&mut self) -> &mut Ident {
        match self {
            Self::Enum(e)   => &mut e.ident,
            Self::Struct(s) => &mut s.ident
        }
    }
}
impl Target {
    fn remove_serde_directives(&mut self) -> Result<Vec<TokenStream>, Error> {
        let mut directives = Vec::new(); {
            let attrs = self.attrs_mut();
            while let Some(i) = attrs.iter().position(
                |attr| attr.path.get_ident().is_some_and(|i| i == "serde")
            ) {
                let serde_attr = attrs.remove(i);
                directives.extend(serde_attr.parse_args_with(
                    Punctuated::<TokenStream, token::Comma>::parse_terminated
                )?)
            }
        }
        Ok(directives)
    }
}

struct Validation {
    _validation: Ident,
    _equal:      token::Eq,
    function:    LitStr,
}
impl Parse for Validation {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _validation: input.parse()?,
            _equal:      input.parse()?,
            function:    input.parse()?,
        })
    }
}

fn serde_attribute_of<T: ToTokens>(directives: impl IntoIterator<Item = T>) -> Attribute {
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
        Some(validation_index) => {
            let validation = serde_directives.remove(validation_index);
            let validation = syn::parse2::<Validation>(validation)?.function;

            let mut proxy = target.clone();
            *proxy.ident_mut() = format_ident!("__serdev_proxy_Serialize_{}__", target.ident());
            proxy .attrs_mut().push(serde_attribute_of(serde_directives.clone()));
            target.attrs_mut().push(serde_attribute_of(serde_directives));

            let proxy_ident  = proxy.ident();
            let target_ident = target.ident();

            quote! {
                const _: () = {
                    #[derive(::serdev::__private__::serde::Serialize)]
                    #[serde(crate = "::serdev::__private__::serde")]
                    #proxy

                    impl #impl_generics ::serdev::__private__::serde::Serialize
                    for #target_ident #ty_generics
                        #where_clause
                    {
                        #[inline]
                        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                        where
                            S: ::serdev::__private__::serde::Serializer
                        {
                            let _: () = #validation(&self).map_err(::serdev::__private__::serde::ser::Error::custom)?;
                            let proxy: #proxy_ident #ty_generics = unsafe {::core::mem::transmute(self)};
                            proxy.serialize(serializer)
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
        Some(validation_index) => {
            let validation = serde_directives.remove(validation_index);
            let validation = syn::parse2::<Validation>(validation)?.function;

            let mut proxy = target.clone();
            *proxy.ident_mut() = format_ident!("__serdev_proxy_Deserialize_{}__", target.ident());
            proxy .attrs_mut().push(serde_attribute_of(serde_directives.clone()));
            target.attrs_mut().push(serde_attribute_of(serde_directives));

            let proxy_ident  = proxy.ident();
            let target_ident = target.ident();

            let proxy_type_lit = LitStr::new(
                &quote!(#proxy_ident #ty_generics).to_string(),
                Span::call_site()
            );

            quote! {
                const _: () = {
                    #[derive(::serdev::__private__::serde::Deserialize)]
                    #[serde(crate = "::serdev::__private__::serde")]
                    #proxy

                    impl #impl_generics ::std::convert::TryFrom<#proxy_ident #ty_generics>
                    for #target_ident #ty_generics
                        #where_clause
                    {
                        type Error = ::std::string::String;
                        fn try_from(proxy: #proxy_ident #ty_generics) -> ::std::result::Result<Self, Self::Error> {
                            let this = unsafe {::core::mem::transmute(proxy)};
                            let _: () = #validation(&this).map_err(|e|);
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
