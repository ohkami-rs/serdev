use proc_macro2::{Span, TokenStream};
use syn::{
    Attribute, Error, Expr, Ident, LitStr, MacroDelimiter, Meta, MetaList, Type, parse::Parse,
    punctuated::Punctuated, spanned::Spanned, token, parse_quote,
};

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
        let _validate = input.parse::<Ident>()?;
        if _validate != "validate" {
            return Err(Error::new(Span::call_site(), "expected `validate`"));
        }

        if input.peek(token::Eq) {
            input.parse::<token::Eq>()?;
            let by: LitStr = input.parse()?;
            Ok(Validate::Eq(by))
        } else if input.peek(token::Paren) {
            let buf;
            syn::parenthesized!(buf in input);
            let mut by = None;
            let mut error = None;
            while !buf.is_empty() {
                if buf.peek(token::Comma) {
                    buf.parse::<token::Comma>()?;
                } else if buf.peek(keyword::by) {
                    buf.parse::<keyword::by>()?;
                    buf.parse::<token::Eq>()?;
                    by = Some(buf.parse()?)
                } else if buf.peek(keyword::error) {
                    buf.parse::<keyword::error>()?;
                    buf.parse::<token::Eq>()?;
                    error = Some(buf.parse()?)
                } else {
                    let rest = buf.parse::<TokenStream>()?;
                    if !rest.is_empty() {
                        return Err(Error::new(
                            rest.span(),
                            "expected `by = \"...\"` or `error = \"...\"`",
                        ));
                    } else {
                        return Err(Error::new(rest.span(), format!("rest: `{rest}`")));
                    }
                }
            }
            let by = by.ok_or(Error::new(Span::call_site(), "expected `by = \"...\"`"))?;
            Ok(Validate::Paren { by, error })
        } else {
            Err(Error::new(
                Span::call_site(),
                "expected `validate = \"...\"` or `validate(by = \"...\", error = \"...\")`",
            ))
        }
    }
}

impl Validate {
    pub(crate) fn take(attrs: &mut Vec<Attribute>) -> Result<Option<Self>, Error> {
        for attr in attrs {
            if attr.path().get_ident().is_some_and(|i| i == "serde") {
                let directives = attr
                    .parse_args_with(Punctuated::<TokenStream, token::Comma>::parse_terminated)?;
                for (i, directive) in directives.iter().enumerate() {
                    if directive.to_string().starts_with("validate") {
                        attr.meta = Meta::List(MetaList {
                            path: syn::parse_str("serde")?,
                            delimiter: MacroDelimiter::Paren(token::Paren::default()),
                            tokens: syn::parse_str(&{
                                let mut others = String::new();
                                for (j, directive) in directives.iter().enumerate() {
                                    if j != i {
                                        others.push_str(&directive.to_string());
                                        others.push(',')
                                    }
                                }
                                others.pop();
                                others
                            })?,
                        });
                        return syn::parse2(directive.clone()).map(Some);
                    }
                }
            }
        }
        Ok(None)
    }

    pub(crate) fn as_validation_expr_with(&self, target_ident: &Ident) -> Result<Expr, Error> {
        let lit_str = match self {
            Self::Eq(by) => by,
            Self::Paren { by, error: _ } => by,
        };

        let expr = syn::parse_str::<Expr>(&lit_str.value())?;
        
        match expr {
            Expr::Path(path) => Ok(parse_quote! {
                #path(&#target_ident)
            }),
            // for type annotations for e.g. inlined closure
            _ => Ok(parse_quote! {
                {
                    let __validater: fn(&Self) -> ::core::result::Result<(), _> = #expr;
                    __validater(&#target_ident)
                }
            })
        }
    }

    pub(crate) fn as_error_ty(&self) -> Result<Option<Type>, Error> {
        match self {
            Self::Paren {
                by: _,
                error: Some(error),
            } => syn::parse_str(&error.value()).map(Some),
            _ => Ok(None),
        }
    }
}
