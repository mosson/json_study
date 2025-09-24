use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, PathArguments, Type, parse_macro_input};

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
                            Some(node::Node::String(s)) => s.clone(),
                            _ => return Err(node::Error::RequiredError(format!("JSONオブジェクトから `{}` が読み取れません", #field_str).to_string())),
                        }
                    });
                }
                _ if is_optional_type(&ty, is_string) => {
                    fragments.push(quote! {
                        #field_name: match map.get(#field_str) {
                            Some(node::Node::String(s)) => Some(s.clone()),
                            _ => None,
                        }
                    });
                }
                _ if is_i8(&ty) => {
                    fragments.push(quote! {
                        #field_name: match map.get(#field_str) {
                            Some(node::Node::Number(s)) => {
                                let s: f64 = s.clone();
                                match i8::try_from(s as i64) {
                                    Ok(i) => i,
                                    Err(e) => return Err(node::Error::ConversionError(e.to_string())),
                                }
                            },
                            _ => return Err(node::Error::RequiredError(format!("JSONオブジェクトから `{}` が読み取れません", #field_str).to_string())),
                        }
                    });
                }
                _ if is_optional_type(&ty, is_i8) => {
                    fragments.push(quote! {
                        #field_name: match map.get(#field_str) {
                            Some(
                                node::Node::Number(s)
                            ) => {
                                let s: f64 = s.clone();
                                match i8::try_from(s as i64) {
                                    Ok(i) => Some(i),
                                    Err(e) => return Err(node::Error::ConversionError(e.to_string())),
                                }
                            },
                            _ => None,
                        }
                    });
                }
                _ if is_i16(&ty) => {
                    fragments.push(quote! {
                        #field_name: match map.get(#field_str) {
                            Some(
                                node::Node::Number(s)
                            ) => {
                                let s: f64 = s.clone();
                                match i16::try_from(s as i64) {
                                    Ok(i) => i,
                                    Err(e) => return Err(node::Error::ConversionError(e.to_string())),
                                }
                            },
                            _ => return Err(node::Error::RequiredError(format!("JSONオブジェクトから `{}` が読み取れません", #field_str).to_string())),
                        }
                    });
                }
                _ if is_optional_type(&ty, is_i16) => {
                    fragments.push(quote! {
                        #field_name: match map.get(#field_str) {
                            Some(
                                node::Node::Number(s)
                            ) => {
                                let s: f64 = s.clone();
                                match i16::try_from(s as i64) {
                                    Ok(i) => Some(i),
                                    Err(e) => return Err(node::Error::ConversionError(e.to_string())),
                                }
                            },
                            _ => None,
                        }
                    });
                }
                _ if is_i32(&ty) => {
                    fragments.push(quote! {
                        #field_name: match map.get(#field_str) {
                            Some(
                                node::Node::Number(s)
                            ) => {
                                let s: f64 = s.clone();
                                match i32::try_from(s as i64) {
                                    Ok(i) => i,
                                    Err(e) => return Err(node::Error::ConversionError(e.to_string())),
                                }
                            },
                            _ => return Err(node::Error::RequiredError(format!("JSONオブジェクトから `{}` が読み取れません", #field_str).to_string())),
                        }
                    });
                }
                _ if is_optional_type(&ty, is_i32) => {
                    fragments.push(quote! {
                        #field_name: match map.get(#field_str) {
                            Some(
                                node::Node::Number(s)
                            ) => {
                                let s: f64 = s.clone();
                                match i32::try_from(s as i64) {
                                    Ok(i) => Some(i),
                                    Err(e) => return Err(node::Error::ConversionError(e.to_string())),
                                }
                            },
                            _ => None,
                        }
                    });
                }
                _ if is_i64(&ty) => {
                    fragments.push(quote! {
                        #field_name: match map.get(#field_str) {
                            Some(
                                node::Node::Number(s)
                            ) => {
                                let s: f64 = s.clone();
                                match i64::try_from(s as i64) {
                                    Ok(i) => i,
                                    Err(e) => return Err(node::Error::ConversionError(e.to_string())),
                                }
                            },
                            _ => return Err(node::Error::RequiredError(format!("JSONオブジェクトから `{}` が読み取れません", #field_str).to_string())),
                        }
                    });
                }
                _ if is_optional_type(&ty, is_i64) => {
                    fragments.push(quote! {
                        #field_name: match map.get(#field_str) {
                            Some(
                                node::Node::Number(s)
                            ) => {
                                let s: f64 = s.clone();
                                match i64::try_from(s as i64) {
                                    Ok(i) => Some(i),
                                    Err(e) => return Err(node::Error::ConversionError(e.to_string())),
                                }
                            },
                            _ => None,
                        }
                    });
                }
                _ if is_isize(&ty) => {
                    fragments.push(quote! {
                        #field_name: match map.get(#field_str) {
                            Some(
                                node::Node::Number(s)
                            ) => {
                                let s: f64 = s.clone();
                                match isize::try_from(s as isize) {
                                    Ok(i) => i,
                                    Err(e) => return Err(node::Error::ConversionError(e.to_string())),
                                }
                            },
                            _ => return Err(node::Error::RequiredError(format!("JSONオブジェクトから `{}` が読み取れません", #field_str).to_string())),
                        }
                    });
                }
                _ if is_optional_type(&ty, is_isize) => {
                    fragments.push(quote! {
                        #field_name: match map.get(#field_str) {
                            Some(
                                node::Node::Number(s)
                            ) => {
                                let s: f64 = s.clone();
                                match isize::try_from(s as i64) {
                                    Ok(i) => Some(i),
                                    Err(e) => return Err(node::Error::ConversionError(e.to_string())),
                                }
                            },
                            _ => None,
                        }
                    });
                }
                _ if is_u8(&ty) => {
                    fragments.push(quote! {
                        #field_name: match map.get(#field_str) {
                            Some(
                                node::Node::Number(s)
                            ) => {
                                let s: f64 = s.clone();
                                match u8::try_from(s as i64) {
                                    Ok(i) => i,
                                    Err(e) => return Err(node::Error::ConversionError(e.to_string())),
                                }
                            },
                            _ => return Err(node::Error::RequiredError(format!("JSONオブジェクトから `{}` が読み取れません", #field_str).to_string())),
                        }
                    });
                }
                _ if is_optional_type(&ty, is_u8) => {
                    fragments.push(quote! {
                        #field_name: match map.get(#field_str) {
                            Some(
                                node::Node::Number(s)
                            ) => {
                                let s: f64 = s.clone();
                                match u8::try_from(s as i64) {
                                    Ok(i) => Some(i),
                                    Err(e) => return Err(node::Error::ConversionError(e.to_string())),
                                }
                            },
                            _ => None,
                        }
                    });
                }
                _ if is_u16(&ty) => {
                    fragments.push(quote! {
                        #field_name: match map.get(#field_str) {
                            Some(
                                node::Node::Number(s)
                            ) => {
                                let s: f64 = s.clone();
                                match u16::try_from(s as i64) {
                                    Ok(i) => i,
                                    Err(e) => return Err(node::Error::ConversionError(e.to_string())),
                                }
                            },
                            _ => return Err(node::Error::RequiredError(format!("JSONオブジェクトから `{}` が読み取れません", #field_str).to_string())),
                        }
                    });
                }
                _ if is_optional_type(&ty, is_u16) => {
                    fragments.push(quote! {
                        #field_name: match map.get(#field_str) {
                            Some(
                                node::Node::Number(s)
                            ) => {
                                let s: f64 = s.clone();
                                match u16::try_from(s as i64) {
                                    Ok(i) => Some(i),
                                    Err(e) => return Err(node::Error::ConversionError(e.to_string())),
                                }
                            },
                            _ => None,
                        }
                    });
                }
                _ if is_u32(&ty) => {
                    fragments.push(quote! {
                        #field_name: match map.get(#field_str) {
                            Some(
                                node::Node::Number(s)
                            ) => {
                                let s: f64 = s.clone();
                                match u32::try_from(s as i64) {
                                    Ok(i) => i,
                                    Err(e) => return Err(node::Error::ConversionError(e.to_string())),
                                }
                            },
                            _ => return Err(node::Error::RequiredError(format!("JSONオブジェクトから `{}` が読み取れません", #field_str).to_string())),
                        }
                    });
                }
                _ if is_optional_type(&ty, is_u32) => {
                    fragments.push(quote! {
                        #field_name: match map.get(#field_str) {
                            Some(
                                node::Node::Number(s)
                            ) => {
                                let s: f64 = s.clone();
                                match u32::try_from(s as i64) {
                                    Ok(i) => Some(i),
                                    Err(e) => return Err(node::Error::ConversionError(e.to_string())),
                                }
                            },
                            _ => None,
                        }
                    });
                }
                _ if is_u64(&ty) => {
                    fragments.push(quote! {
                        #field_name: match map.get(#field_str) {
                            Some(
                                node::Node::Number(s)
                            ) => {
                                let s: f64 = s.clone();
                                match u64::try_from(s as i64) {
                                    Ok(i) => i,
                                    Err(e) => return Err(node::Error::ConversionError(e.to_string())),
                                }
                            },
                            _ => return Err(node::Error::RequiredError(format!("JSONオブジェクトから `{}` が読み取れません", #field_str).to_string())),
                        }
                    });
                }
                _ if is_optional_type(&ty, is_u64) => {
                    fragments.push(quote! {
                        #field_name: match map.get(#field_str) {
                            Some(
                                node::Node::Number(s)
                            ) => {
                                let s: f64 = s.clone();
                                match u64::try_from(s as i64) {
                                    Ok(i) => Some(i),
                                    Err(e) => return Err(node::Error::ConversionError(e.to_string())),
                                }
                            },
                            _ => None,
                        }
                    });
                }
                _ if is_usize(&ty) => {
                    fragments.push(quote! {
                        #field_name: match map.get(#field_str) {
                            Some(
                                node::Node::Number(s)
                            ) => {
                                let s: f64 = s.clone();
                                match usize::try_from(s as i64) {
                                    Ok(i) => i,
                                    Err(e) => return Err(node::Error::ConversionError(e.to_string())),
                                }
                            },
                            _ => return Err(node::Error::RequiredError(format!("JSONオブジェクトから `{}` が読み取れません", #field_str).to_string())),
                        }
                    });
                }
                _ if is_optional_type(&ty, is_usize) => {
                    fragments.push(quote! {
                        #field_name: match map.get(#field_str) {
                            Some(
                                node::Node::Number(s)
                            ) => {
                                let s: f64 = s.clone();
                                match usize::try_from(s as i64) {
                                    Ok(i) => Some(i),
                                    Err(e) => return Err(node::Error::ConversionError(e.to_string())),
                                }
                            },
                            _ => None,
                        }
                    });
                }
                _ if is_f32(&ty) => {
                    fragments.push(quote! {
                        #field_name: match map.get(#field_str) {
                            Some(
                                node::Node::Number(s)
                            ) => {
                                let s: f64 = s.clone();
                                match f32::try_from(s) {
                                    Ok(i) => i,
                                    Err(e) => return Err(node::Error::ConversionError(e.to_string())),
                                }
                            },
                            _ => return Err(node::Error::RequiredError(format!("JSONオブジェクトから `{}` が読み取れません", #field_str).to_string())),
                        }
                    });
                }
                _ if is_optional_type(&ty, is_f32) => {
                    fragments.push(quote! {
                        #field_name: match map.get(#field_str) {
                            Some(
                                node::Node::Number(s)
                            ) => {
                                let s: f64 = s.clone();
                                match f32::try_from(s) {
                                    Ok(i) => Some(i),
                                    Err(e) => return Err(node::Error::ConversionError(e.to_string())),
                                }
                            },
                            _ => None,
                        }
                    });
                }
                _ if is_f64(&ty) => {
                    fragments.push(quote! {
                        #field_name: match map.get(#field_str) {
                            Some(
                                node::Node::Number(s)
                            ) => s.clone(),
                            _ => return Err(node::Error::RequiredError(format!("JSONオブジェクトから `{}` が読み取れません", #field_str).to_string())),
                        }
                    });
                }
                _ if is_optional_type(&ty, is_f64) => {
                    fragments.push(quote! {
                        #field_name: match map.get(#field_str) {
                            Some(
                                node::Node::Number(s)
                            ) => Some(s.clone()),
                            _ => None,
                        }
                    });
                }
                _ if is_bool(&ty) => {
                    fragments.push(quote! {
                        #field_name: match map.get(#field_str) {
                            Some(node::Node::True) => true,
                            Some(node::Node::False) => false,
                            _ => return Err(node::Error::RequiredError(format!("JSONオブジェクトから `{}` が読み取れません", #field_str).to_string())),
                        }
                    });
                }
                _ if is_optional_type(&ty, is_bool) => {
                    fragments.push(quote! {
                        #field_name: match map.get(#field_str) {
                            Some(node::Node::True) => Some(true),
                            Some(node::Node::False) => Some(false),
                            _ => None,
                        }
                    });
                }
                _ => {
                    let message = format!(
                        "サポートされていないデータ型が指定されました（{}: {}）",
                        field_name,
                        get_data_type(&ty)
                    );

                    fragments.push(quote! { #field_name: compile_error!(#message) })
                }
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
                    Err(node::Error::ConversionError("構造体へのJSONのマッピングはJSONオブジェクトのみサポートしています".into()))
                }
            }
        }
    };

    TokenStream::from(expanded)
}

fn get_data_type(ty: &Type) -> String {
    match ty {
        Type::Path(type_path) => type_path
            .path
            .get_ident()
            .map(|i| i.to_string())
            .unwrap_or("".to_string()),
        _ => "".to_string(),
    }
}

fn is_string(ty: &Type) -> bool {
    matches!(ty, Type::Path(type_path) if type_path.path.is_ident("String"))
}
fn is_i8(ty: &Type) -> bool {
    matches!(ty, Type::Path(type_path) if type_path.path.is_ident("i8"))
}
fn is_i16(ty: &Type) -> bool {
    matches!(ty, Type::Path(type_path) if type_path.path.is_ident("i16"))
}
fn is_i32(ty: &Type) -> bool {
    matches!(ty, Type::Path(type_path) if type_path.path.is_ident("i32"))
}
fn is_i64(ty: &Type) -> bool {
    matches!(ty, Type::Path(type_path) if type_path.path.is_ident("i64"))
}
fn is_isize(ty: &Type) -> bool {
    matches!(ty, Type::Path(type_path) if type_path.path.is_ident("isize"))
}
fn is_u8(ty: &Type) -> bool {
    matches!(ty, Type::Path(type_path) if type_path.path.is_ident("u8"))
}
fn is_u16(ty: &Type) -> bool {
    matches!(ty, Type::Path(type_path) if type_path.path.is_ident("u16"))
}
fn is_u32(ty: &Type) -> bool {
    matches!(ty, Type::Path(type_path) if type_path.path.is_ident("u32"))
}
fn is_u64(ty: &Type) -> bool {
    matches!(ty, Type::Path(type_path) if type_path.path.is_ident("u64"))
}
fn is_usize(ty: &Type) -> bool {
    matches!(ty, Type::Path(type_path) if type_path.path.is_ident("usize"))
}
fn is_f32(ty: &Type) -> bool {
    matches!(ty, Type::Path(type_path) if type_path.path.is_ident("f32"))
}
fn is_f64(ty: &Type) -> bool {
    matches!(ty, Type::Path(type_path) if type_path.path.is_ident("f64"))
}
fn is_bool(ty: &Type) -> bool {
    matches!(ty, Type::Path(type_path) if type_path.path.is_ident("bool"))
}

fn is_optional_type(ty: &Type, f: fn(&Type) -> bool) -> bool {
    match option_inner_type(ty) {
        Some(ty) => f(ty),
        None => false,
    }
}

fn option_inner_type(ty: &Type) -> Option<&Type> {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.first() {
            if segment.ident == "Option" {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                        return Some(inner_ty);
                    }
                }
            }
        }
    }
    None
}
