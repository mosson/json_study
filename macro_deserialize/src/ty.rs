use quote::quote;
use syn::{Field, PathArguments, PathSegment, Type};

pub(crate) enum Ty {
    String,
    Signed8,
    Signed16,
    Signed32,
    Signed64,
    SignedSize,
    Unsigned8,
    Unsigned16,
    Unsigned32,
    Unsigned64,
    UnsignedSize,
    Float64,
    Bool,
    Optional(Box<Type>),
    Vector(Box<Type>),
    Object,
    Tuple(Vec<Type>),
}

impl Ty {
    pub(crate) fn to_token_stream(field: &Field) -> proc_macro2::TokenStream {
        let field_name = field.ident.as_ref().unwrap();
        let field_str = field_name.to_string();
        let ty = &field.ty;
        let exp = token_stream(&field_str, ty, true);

        quote! {
            #field_name: match map.get(#field_str) {
                #exp
            }
        }
    }

    fn from_ident(ident: &proc_macro2::Ident) -> Self {
        match ident.to_string().as_str() {
            "String" | "alloc::String" => Self::String,
            "i8" => Self::Signed8,
            "i16" => Self::Signed16,
            "i32" => Self::Signed32,
            "i64" => Self::Signed64,
            "isize" => Self::SignedSize,
            "u8" => Self::Unsigned8,
            "u16" => Self::Unsigned16,
            "u32" => Self::Unsigned32,
            "u64" => Self::Unsigned64,
            "usize" => Self::UnsignedSize,
            "f64" => Self::Float64,
            "bool" => Self::Bool,
            _ => Self::Object,
        }
    }
}

impl From<&Type> for Ty {
    fn from(value: &Type) -> Self {
        match value {
            Type::Path(type_path) => match type_path.path.segments.first() {
                Some(segment) => match segment.ident.to_string().as_str() {
                    "Option" => Self::Optional(Box::new(inner_type(segment))),
                    "Vec" => Self::Vector(Box::new(inner_type(segment))),
                    _ => Self::from_ident(&segment.ident),
                },
                _ => Self::from_ident(&type_path.path.get_ident().unwrap()),
            },
            Type::Tuple(tuple) => Self::Tuple(tuple.elems.iter().cloned().collect::<Vec<_>>()),
            _ => Self::Object,
        }
    }
}

fn inner_type(segment: &PathSegment) -> Type {
    match &segment.arguments {
        PathArguments::AngleBracketed(args) => match args.args.first() {
            Some(syn::GenericArgument::Type(ty)) => ty.clone(),
            _ => panic!("ジェネリクスの型が取得できませんでした"),
        },
        _ => panic!("ジェネリクスであるべきところでアングルブラケットを取得できませんでした"),
    }
}

fn token_stream(key: &str, ty: &Type, required: bool) -> proc_macro2::TokenStream {
    match &Ty::from(ty) {
        Ty::String => string_expression(key, required),
        Ty::Signed8
        | Ty::Signed16
        | Ty::Signed32
        | Ty::Signed64
        | Ty::SignedSize
        | Ty::Unsigned8
        | Ty::Unsigned16
        | Ty::Unsigned32
        | Ty::Unsigned64
        | Ty::UnsignedSize => int_expression(key, ty, required),
        Ty::Float64 => float_expression(key, ty, required),
        Ty::Bool => bool_expression(key, required),
        Ty::Optional(inner_ty) => token_stream(key, inner_ty, false),
        Ty::Object => object_expression(key, ty, required),
        Ty::Vector(inner_ty) => vector_expression(key, inner_ty, required),
        Ty::Tuple(tuple) => tuple_expression(key, tuple, required),
    }
}

fn string_expression(key: &str, required: bool) -> proc_macro2::TokenStream {
    if required {
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

fn int_expression(key: &str, ty: &Type, required: bool) -> proc_macro2::TokenStream {
    if required {
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

fn float_expression(key: &str, ty: &Type, required: bool) -> proc_macro2::TokenStream {
    if required {
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

fn bool_expression(key: &str, required: bool) -> proc_macro2::TokenStream {
    if required {
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

fn object_expression(key: &str, ty: &Type, required: bool) -> proc_macro2::TokenStream {
    if required {
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

fn vector_expression(key: &str, ty: &Type, required: bool) -> proc_macro2::TokenStream {
    let exp = token_stream(key, ty, required);

    if required {
        quote! {
            Some(node::Node::Array(nodes)) => {
                let mut values = vec![];

                for node in nodes.into_iter() {
                    values.push(
                        match Some(node) {
                            #exp
                        }
                    )
                }

                values
            },
            _ => return Err(node::Error::RequiredError(format!("JSONオブジェクトから `{}` が読み取れません", #key).to_string())),
        }
    } else {
        quote! {
            Some(node::Node::Array(nodes)) => {
                let mut values = vec![];

                for node in nodes.into_iter() {
                    values.push(
                        match Some(node) {
                            #exp
                        }
                    )
                }

                Some(values)
            },
            _ => None,
        }
    }
}

fn tuple_expression(key: &str, tuple: &Vec<Type>, required: bool) -> proc_macro2::TokenStream {
    let mut exps = vec![];

    for ty in tuple.into_iter() {
        let exp = token_stream(key, ty, true);

        exps.push(quote! {
            {
                let node = iter.next();
                match node {
                    #exp
                }
            }
        });
    }

    if required {
        quote! {
            Some(node::Node::Array(nodes)) => {
                let mut iter = nodes.into_iter();

                (#(#exps),*)
            },
            _ => return Err(node::Error::RequiredError(format!("JSONオブジェクトから `{}` が読み取れません", #key).to_string())),
        }
    } else {
        quote! {
            Some(node::Node::Array(nodes)) => {
                let mut iter = nodes.into_iter();

                Some((#(#exps),*))
            },
            _ => None,
        }
    }
}
