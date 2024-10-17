use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Type};

#[proc_macro_derive(Named)]
pub fn derive_named(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    TokenStream::from(quote! {
        impl actors::Named for #name {
            const NAME: &'static str = stringify!(#name);
        }
    })
}

#[proc_macro_derive(Message, attributes(result_type))]
pub fn derive_message(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    for attr in input.attrs.iter() {
        if !attr.meta.path().is_ident("result_type") {
            continue;
        }

        let ty = attr.parse_args::<Type>().expect("invalid arguments");

        return quote! {
            impl actors::Message for #name {
                type Result = #ty;
            }
        }
        .into();
    }

    panic!("missing #[result_type] attribute");
}
