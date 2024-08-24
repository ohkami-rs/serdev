use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{parse::Parse, punctuated::Punctuated, token, Attribute, Error, Generics, Ident, Item, ItemEnum, ItemStruct};


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
}

impl Target {
    pub(crate) fn remove_serde_directives(&mut self) -> Result<Vec<TokenStream>, Error> {
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
