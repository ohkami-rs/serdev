use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{parse::Parse, punctuated::Punctuated, token, Attribute, Error, Generics, Ident, Item, ItemEnum, ItemStruct, LitStr, WhereClause, WherePredicate};


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
        let _validation = input.parse()?;
        if _validation != "validation" {
            return Err(Error::new(Span::call_site(), "expected `validation`"))
        }

        let _equal      = input.parse()?;
        let function    = input.parse()?;

        Ok(Self { _validation, _equal, function })
    }
}

struct Bound {
    _bound:      Ident,
    _equal:      Option<token::Eq>,
    _paren:      Option<token::Paren>,
    always:      Option<LitStr>,
    serialize:   Option<LitStr>,
    #[allow(unused)]
    deserialize: Option<LitStr>,
}
impl Parse for Bound {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _bound = input.parse()?;
        if _bound != "bound" {
            return Err(Error::new(Span::call_site(), "expected `bound`"))
        }

        let mut _equal      = None;
        let mut _paren      = None;
        let mut serialize   = None;
        let mut deserialize = None;
        let mut always      = None;

        if input.peek(token::Eq) {
            _equal = Some(input.parse()?);
            always = Some(input.parse()?);

        } else if input.peek(token::Paren) {
            let buf; syn::parenthesized!(buf in input);
            while buf.peek(syn::Ident) {
                let when: Ident  = buf.parse()?;
                let _: token::Eq = buf.parse()?;
                if when == "serialize" {
                    serialize   = Some(buf.parse()?);
                } else if when == "deserialize" {
                    deserialize = Some(buf.parse()?);
                }

                if buf.peek(token::Comma) {buf.parse::<token::Comma>()?;}
            }

        } else {
            return Err(Error::new(Span::call_site(), "expected `bound = ...` or `bound(...)`"))
        }

        Ok(Self { _bound, _equal, _paren, serialize, deserialize, always })
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

    let mut where_clause = where_clause.map(ToOwned::to_owned);
    if let Some(bound_position) = serde_directives.iter().position(
        |d| d.to_string().starts_with("bound")
    ) {
        let bound = serde_directives[bound_position].clone();
        let bound = syn::parse2::<Bound>(bound)?;
        if let Some(serialize_bound) = bound.always.or(bound.serialize) {
            let predicates = serialize_bound.value()
                .split(',')
                .map(syn::parse_str::<WherePredicate>)
                .collect::<syn::Result<Punctuated<WherePredicate, token::Comma>>>()?;
            match &mut where_clause {
                Some(wc) => wc.predicates.extend(predicates),
                None => where_clause = Some(WhereClause {
                    where_token: token::Where::default(),
                    predicates 
                })
            }
        }
    }

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
                        fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
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

                    impl #impl_generics ::core::convert::TryFrom<#proxy_ident #ty_generics>
                    for #target_ident #ty_generics
                        #where_clause
                    {
                        type Error = ::std::box::Box<dyn ::std::fmt::Display>;

                        #[inline]
                        fn try_from(proxy: #proxy_ident #ty_generics) -> ::core::result::Result<Self, Self::Error> {
                            let this = unsafe {::core::mem::transmute(proxy)};
                            let _: () = #validation(&this).map_err(|e| ::std::box::Box::new(e))?;
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
