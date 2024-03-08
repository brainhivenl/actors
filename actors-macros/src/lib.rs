use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Type};

#[proc_macro_derive(Message)]
pub fn derive_message(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

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

#[proc_macro_attribute]
pub fn result_type(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}
