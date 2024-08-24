use proc_macro2::{Span, TokenStream};
use syn::{parse::Parse, spanned::Spanned, token, Error, Ident, LitStr};


mod keyword {
    syn::custom_keyword! { by }
    syn::custom_keyword! { error }
}

pub(crate) enum Validate {
    Eq(LitStr),
    Paren { by: LitStr, error: Option<LitStr> },
}

impl Parse for Validate {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _validation = input.parse::<Ident>()?;
        if _validation != "validation" {
            return Err(Error::new(Span::call_site(), "expected `validation`"))
        }

        if input.peek(token::Eq) {
            input.parse::<token::Eq>()?;
            let by: LitStr = input.parse()?;
            Ok(Validate::Eq(by))

        } else if input.peek(token::Paren) {
            let buf; syn::parenthesized!(buf in input);
            let mut by    = None;
            let mut error = None;
            while !buf.is_empty() {
                if input.peek(token::Comma) {
                    input.parse::<token::Comma>()?;
                } else if input.peek(keyword::by) {
                    input.parse::<keyword::by>()?;
                    input.parse::<token::Eq>()?;
                    by = Some(input.parse()?)
                } else if input.peek(keyword::error) {
                    input.parse::<keyword::error>()?;
                    input.parse::<token::Eq>()?;
                    error = Some(input.parse()?)
                } else {
                    let rest = input.parse::<TokenStream>()?;
                    return Err(Error::new(rest.span(), "expected `by = \"...\"` or `error = \"...\"`"))
                }
            }
            let by = by.ok_or(Error::new(Span::call_site(), "expected `by = \"...\"`"))?;
            Ok(Validate::Paren { by, error })

        } else {
            Err(Error::new(Span::call_site(), "expected `validation = \"...\"` or `validation(by = \"...\", error = \"...\")`"))
        }
    }
}

impl Validate {
    pub(crate) fn function(&self) -> LitStr {
        match self {
            Self::Eq(by) => by.clone(),
            Self::Paren { by, error:_ } => by.clone()
        }
    }
    pub(crate) fn error(&self) -> Option<LitStr> {
        match self {
            Self::Eq(_) => None,
            Self::Paren { by:_, error } => error.clone()
        }
    }
}