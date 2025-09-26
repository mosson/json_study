use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

use crate::ty::Ty;

mod ty;

#[proc_macro_derive(Deserialize)]
pub fn deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = match input.data {
        Data::Struct(data) => data.fields,
        _ => {
            return quote! { compile_error!("Deserializeマクロは構造体にしか利用できません") }
                .into();
        }
    };

    let mut ast = vec![];

    if let Fields::Named(named) = fields {
        for field in named.named {
            ast.push(Ty::to_token_stream(&field))
        }
    }

    let expanded = quote! {
        impl node::FromNode for #name {
            fn from_node(value: &node::Node) -> Result<Self, node::Error> {
                if let node::Node::Object(map) = value {
                    Ok(Self {
                        #(#ast),*
                    })
                } else {
                    Err(node::Error::ConversionError("構造体へのJSONのマッピングはJSONオブジェクトのみサポートしています".into()))
                }
            }
        }
    };

    TokenStream::from(expanded)
}
