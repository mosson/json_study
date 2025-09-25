use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Data, DeriveInput, Field, Fields, GenericArgument, PathArguments, Type, TypePath,
    parse_macro_input,
};

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
            ast.push(resolve_primitive_token(&field))
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

fn resolve_primitive_token(field: &Field) -> proc_macro2::TokenStream {
    let field_name = field.ident.as_ref().unwrap();
    let field_str = field_name.to_string();

    let exp = value_expression(
        data_type(field).expect("文法がおかしい"),
        field_str.as_str(),
    );

    quote! {
        #field_name: match map.get(#field_str) {
            #exp
        }
    }
}

fn value_expression(type_key: String, key: &str) -> proc_macro2::TokenStream {
    let type_key = type_key.as_str();

    match type_key {
        "String" => string_expression::<true>(key),
        "Option<String>" => string_expression::<false>(key),
        "i8" | "i16" | "i32" | "i64" | "isize" | "u8" | "u16" | "u32" | "u64" | "usize" => {
            let ty = &syn::parse_str(type_key).unwrap();

            int_expression::<true>(key, ty)
        }
        "Option<i8>" | "Option<i16>" | "Option<i32>" | "Option<i64>" | "Option<isize>"
        | "Option<u8>" | "Option<u16>" | "Option<u32>" | "Option<u64>" | "Option<usize>" => {
            let ty = &syn::parse_str(type_key).unwrap();
            let ty = inner_ty(ty);

            int_expression::<false>(key, ty)
        }
        "f32" | "f64" => {
            let ty = &syn::parse_str(type_key).unwrap();

            float_expression::<true>(key, ty)
        }
        "Option<f32>" | "Option<f64>" => {
            let ty = &syn::parse_str(type_key).unwrap();
            let ty = inner_ty(ty);

            float_expression::<false>(key, ty)
        }
        "bool" => bool_expression::<true>(key),
        "Option<bool>" => bool_expression::<false>(key),
        _ => {
            let ty = &syn::parse_str(type_key).unwrap();
            if option_type(ty) {
                let ty = inner_ty(ty);
                object_expression::<false>(key, ty)
            } else {
                object_expression::<true>(key, ty)
            }
        }
    }
}

fn inner_ty(ty: &Type) -> &Type {
    match ty {
        Type::Path(TypePath { path, .. }) => {
            let segment = path.segments.last().unwrap();

            match &segment.arguments {
                PathArguments::AngleBracketed(args) => match args.args.last().unwrap() {
                    GenericArgument::Type(inner_ty) => inner_ty,
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

fn string_expression<const REQUIRED: bool>(key: &str) -> proc_macro2::TokenStream {
    if REQUIRED {
        quote! {
            Some(node::Node::String(s)) => s.clone(),
            _ => return Err(node::Error::RequiredError(format!("JSONオブジェクトから `{}` が読み取れません", #key).to_string())),
        }
    } else {
        quote! {
            Some(node::Node::String(s)) => Some(s.clone()),
            _ => None,
        }
    }
}

fn int_expression<const REQUIRED: bool>(key: &str, ty: &Type) -> proc_macro2::TokenStream {
    if REQUIRED {
        quote! {
            Some(node::Node::Number(s)) => {
                let s: f64 = s.clone();
                match <#ty as TryFrom<i64>>::try_from(s as i64) {
                    Ok(i) => i,
                    Err(e) => return Err(node::Error::ConversionError(e.to_string())),
                }
            },
            _ => return Err(node::Error::RequiredError(format!("JSONオブジェクトから `{}` が読み取れません", #key).to_string())),
        }
    } else {
        quote! {
            Some(node::Node::Number(s)) => {
                let s: f64 = s.clone();
                match <#ty as TryFrom<i64>>::try_from(s as i64) {
                    Ok(i) => Some(i),
                    Err(e) => return Err(node::Error::ConversionError(e.to_string())),
                }
            },
            _ => None,
        }
    }
}

fn float_expression<const REQUIRED: bool>(key: &str, ty: &Type) -> proc_macro2::TokenStream {
    if REQUIRED {
        quote! {
            Some(node::Node::Number(s)) => {
                let s: f64 = s.clone();
                match <#ty as TryFrom<f64>>::try_from(s) {
                    Ok(i) => i,
                    Err(e) => return Err(node::Error::ConversionError(e.to_string())),
                }
            },
            _ => return Err(node::Error::RequiredError(format!("JSONオブジェクトから `{}` が読み取れません", #key).to_string())),
        }
    } else {
        quote! {
            Some(node::Node::Number(s)) => {
                let s: f64 = s.clone();
                match <#ty as TryFrom<f64>>::try_from(s) {
                    Ok(i) => Some(i),
                    Err(e) => return Err(node::Error::ConversionError(e.to_string())),
                }
            },
            _ => None,
        }
    }
}

fn bool_expression<const REQUIRED: bool>(key: &str) -> proc_macro2::TokenStream {
    if REQUIRED {
        quote! {
            Some(node::Node::True) => true,
            Some(node::Node::False) => false,
            _ => return Err(node::Error::RequiredError(format!("JSONオブジェクトから `{}` が読み取れません", #key).to_string())),
        }
    } else {
        quote! {
            Some(node::Node::True) => Some(true),
            Some(node::Node::False) => Some(false),
            _ => None,
        }
    }
}

fn object_expression<const REQUIRED: bool>(key: &str, ty: &Type) -> proc_macro2::TokenStream {
    if REQUIRED {
        quote! {
            Some(node) => <#ty as node::FromNode>::from_node(&node)?,
            _ => return Err(node::Error::RequiredError(format!("JSONオブジェクトから `{}` が読み取れません", #key).to_string())),
        }
    } else {
        quote! {
            Some(node::Node::Null) => None,
            Some(node) => Some(<#ty as node::FromNode>::from_node(node)?),
            _ => None,
        }
    }
}

fn data_type(field: &Field) -> Option<String> {
    if let Type::Path(type_path) = &field.ty {
        if let Some(segment) = type_path.path.segments.first() {
            if segment.ident == "Option" {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                        return Some(format!(
                            "Option<{}>",
                            inner_ty.to_token_stream().to_string()
                        ));
                    }
                }
            }
        }

        return type_path.path.get_ident().map(|ident| ident.to_string());
    }

    None
}

fn option_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.first() {
            return segment.ident == "Option";
        }
    }

    false
}
