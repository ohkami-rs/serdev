use proc_macro2::TokenStream;
use syn::{parse::Parse, punctuated::Punctuated, token, Attribute, Error, LitStr, MacroDelimiter, Meta, MetaList, Path};


pub(crate) struct Reexport {
    path: LitStr,
}

impl Parse for Reexport {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _path: token::Crate = input.parse()?;

        let _eq: token::Eq = input.parse()?;

        let path: LitStr = input.parse()?;

        Ok(Self { path })
    }
}

impl Reexport {
    pub(crate) fn take(attrs: &mut Vec<Attribute>) -> Result<Option<Self>, Error> {
        for attr in attrs {
            if attr.path().get_ident().is_some_and(|i| i == "serdev") {
                let directives = attr.parse_args_with(
                    Punctuated::<TokenStream, token::Comma>::parse_terminated
                )?;
                for (i, directive) in directives.iter().enumerate() {
                    if directive.to_string().starts_with("crate") {
                        attr.meta = Meta::List(MetaList {
                            path:      syn::parse_str("serdev")?,
                            delimiter: MacroDelimiter::Paren(Default::default()),
                            tokens:    syn::parse_str(&{
                                let mut others = String::new();
                                for (j, directive) in directives.iter().enumerate() {
                                    if j != i {
                                        others.push_str(&directive.to_string());
                                        others.push(',')
                                    }
                                }; others.pop();
                                others
                            })?
                        });
                        return syn::parse2(directive.clone()).map(Some)
                    }
                }
            }
        }; Ok(None)
    }
}

impl Reexport {
    pub(crate) fn path(&self) -> Result<Path, Error> {
        syn::parse_str(&self.path.value())
    }

    pub(crate) fn path_str(&self) -> String {
        self.path.value()
    }
}
