#![allow(non_snake_case)]

mod internal;

#[proc_macro_derive(Serialize, attributes(serde, serdev))]
pub fn Serialize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    internal::Serialize(input.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(Deserialize, attributes(serde, serdev))]
pub fn Deserialize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    internal::Deserialize(input.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[doc(hidden)]
#[proc_macro_attribute]
pub fn consume(_: proc_macro::TokenStream, _: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::new()
}
