use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{parse::Parse, Attribute, Error, Fields, Generics, Ident, Item, ItemEnum, ItemStruct};


#[derive(Clone)]
pub(crate) enum Target {
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
    pub(crate) fn attrs_mut(&mut self) -> &mut Vec<Attribute> {
        match self {
            Self::Enum(e)   => &mut e.attrs,
            Self::Struct(s) => &mut s.attrs
        }
    }

    pub(crate) fn generics(&self) -> &Generics {
        match self {
            Self::Enum(e)   => &e.generics,
            Self::Struct(s) => &s.generics
        }
    }

    pub(crate) fn ident(&self) -> &Ident {
        match self {
            Self::Enum(e)   => &e.ident,
            Self::Struct(s) => &s.ident
        }
    }
    pub(crate) fn ident_mut(&mut self) -> &mut Ident {
        match self {
            Self::Enum(e)   => &mut e.ident,
            Self::Struct(s) => &mut s.ident
        }
    }

    pub(crate) fn transmute_expr(&self,
        variable_ident: &'static str,
        target_ident:   &Ident
    ) -> TokenStream {
        let var = Ident::new(variable_ident, Span::call_site());

        fn constructor(fields: &Fields) -> TokenStream {
            match fields {
                Fields::Unit => {
                    quote! {}
                }
                Fields::Unnamed(u) => {
                    let idents = (0..u.unnamed.len()).map(|i| format_ident!("field_{i}"));
                    quote! {
                        ( #(#idents),* )
                    }
                }
                Fields::Named(n) => {
                    let idents = n.named.iter().map(|f| f.ident.as_ref().unwrap());
                    quote! {
                        { #(#idents),* }
                    }
                }
            }
        }

        match self {
            Self::Struct(s) => {
                let ident = &s.ident;
                let constructor = constructor(&s.fields);
                quote! {{
                    let #ident #constructor = #var;
                    #target_ident #constructor
                }}
            }
            Self::Enum(e) => {
                let ident = &e.ident;

                let arms = e.variants.iter().map(|v| {
                    let variant = &v.ident;
                    let fields  = constructor(&v.fields);
                    quote! {
                        #ident::#variant #fields => #target_ident::#variant #fields
                    }
                });

                quote! {
                    match #var {
                        #(#arms),*
                    }
                }
            }
        }
    }
}
