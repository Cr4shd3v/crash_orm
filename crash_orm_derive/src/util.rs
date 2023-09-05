use syn::{GenericArgument, Path, PathArguments, Type};

pub(crate) fn extract_type_from_option(ty: &Type) -> Option<Type> {
    fn path_is_option(path: &Path) -> bool {
        path.leading_colon.is_none()
            && path.segments.len() == 1
            && path.segments.iter().next().unwrap().ident == "Option"
    }

    match ty {
        Type::Path(type_path) if path_is_option(&type_path.path) => {
            extract_generic_type(ty)
        }
        _ => return None,
    }
}

pub(crate) fn extract_generic_type(ty: &Type) -> Option<Type> {
    Some(match ty {
        Type::Path(type_path) if type_path.qself.is_none() => {
            let type_params = type_path.path.segments.first().unwrap().clone().arguments;
            let generic_arg = match type_params {
                PathArguments::AngleBracketed(params) => params.args.first().unwrap().clone(),
                _ => return None,
            };
            match generic_arg {
                GenericArgument::Type(ty) => ty,
                _ => return None,
            }
        }
        _ => return None,
    })
}

pub(crate) fn rust_to_postgres_type(field_type: &Type, field_name: &str) -> String {
    let Type::Path(path) = field_type else { panic!("unsupported") };
    let path = path.path.segments.last().unwrap().clone().ident;
    let path = path.to_string().replace(" ", "");

    let column_type = match &*path {
        "bool" => "bool",
        "i8" => "char",
        "i16" => "int2",
        "i32" => "int4",
        "i64" => "int8",
        "u32" => "oid",
        "f32" => "float4",
        "f64" => "float8",
        "String" => "text",
        "Decimal" => "numeric",
        "Option" => {
            let res = rust_to_postgres_type(&extract_generic_type(field_type).unwrap(), field_name);
            return format!("{}{}", res, if &*field_name != "id" { "" } else { " NOT NULL" });
        },
        _ => panic!("unsupported type {}", path),
    };

    column_type.to_string()
}