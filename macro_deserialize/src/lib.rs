use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type, parse_macro_input};

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

    let mut fragments = vec![];

    if let Fields::Named(named) = fields {
        for field in named.named {
            let field_name = field.ident.unwrap();
            let field_str = field_name.to_string();
            let ty = field.ty;

            match true {
                _ if is_string(&ty) => {
                    fragments.push(quote! {
                        #field_name: match map.get(#field_str) {
                            Some(
                                node::Node::String(s)
                            ) => s.clone(),
                            _ => return Err(node::Error::ConversionError),
                        }
                    });
                }
                _ => {}
            }
        }
    }

    let expanded = quote! {
        impl #name {
            pub fn from_value(value: &node::Node) -> Result<Self, node::Error> {
                if let node::Node::Object(map) = value {
                    Ok(Self {
                        #(#fragments),*
                    })
                } else {
                    Err(node::Error::ConversionError)
                }
            }
        }
    };

    TokenStream::from(expanded)
}

fn is_string(ty: &Type) -> bool {
    matches!(ty, Type::Path(type_path) if type_path.path.is_ident("String"))
}
