use quote::ToTokens;
use syn::{GenericArgument, Ident, PathArguments, Type};

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
        "OneToOne" =>  {
            let target_entity = extract_generic_type(field_type).unwrap();
            return format!("oid REFERENCES {}(id)", string_to_table_name(target_entity.into_token_stream().to_string()));
        },
        "ManyToOne" => {
            let target_entity = extract_generic_type(field_type).unwrap();
            return format!("oid REFERENCES {}(id)", string_to_table_name(target_entity.into_token_stream().to_string()));
        },
        "Option" => {
            let res = rust_to_postgres_type(&extract_generic_type(field_type).unwrap(), field_name);
            return format!("{}{}", res, if &*field_name != "id" { "" } else { " NOT NULL" });
        },
        _ => panic!("unsupported type {}", path),
    };

    column_type.to_string()
}

pub(crate) fn ident_to_table_name(ident: &Ident) -> String {
    string_to_table_name(ident.to_string())
}

pub(crate) fn string_to_table_name(string: String) -> String { string.to_lowercase() }