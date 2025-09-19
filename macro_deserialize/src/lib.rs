use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

#[proc_macro_derive(Deserialize)]
pub fn deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = match input.data {
        Data::Struct(data) => data.fields,
        _ => panic!("Deserializeマクロは構造体にしか利用できません"),
    };

    println!("{:?}, {:#?}", name, fields);

    let expanded = quote! {};

    TokenStream::from(expanded)
}
