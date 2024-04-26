use convert_case::{Case, Casing};
use quote::ToTokens;
use syn::{GenericArgument, Ident, PathArguments, Type};

pub(crate) fn extract_generic_type_ignore_option(ty: &Type, number: usize) -> Option<Type> {
    if get_type_string(ty) == "Option" {
        extract_generic_type(&extract_generic_type(ty, 1).unwrap(), number)
    } else {
        extract_generic_type(ty, number)
    }
}

pub(crate) fn extract_generic_type(ty: &Type, number: usize) -> Option<Type> {
    Some(match ty {
        Type::Path(type_path) if type_path.qself.is_none() => {
            let type_params = type_path.path.segments.first().unwrap().clone().arguments;
            let generic_arg = match type_params {
                PathArguments::AngleBracketed(params) => params.args[number - 1].clone(),
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

pub(crate) fn get_type_string(field_type: &Type) -> String {
    let Type::Path(path) = field_type else {
        panic!("unsupported")
    };
    let path = path.path.segments.last().unwrap().clone().ident;
    path.to_string().replace(" ", "")
}

pub(crate) fn is_relation_value_holder(field_type: &Type) -> bool {
    let path = get_type_string(field_type);

    match &*path {
        "OneToOne" => true,
        "ManyToOne" => true,
        "Option" => is_relation_value_holder(&extract_generic_type(field_type, 1).unwrap()),
        _ => false,
    }
}

pub(crate) fn is_relation(field_type: &Type) -> bool {
    let path = get_type_string(field_type);

    match &*path {
        "OneToOneRef" => true,
        "OneToOne" => true,
        "OneToMany" => true,
        "ManyToOne" => true,
        "Option" => is_relation(&extract_generic_type(field_type, 1).unwrap()),
        _ => false,
    }
}

pub(crate) fn rust_to_postgres_type(field_type: &Type, field_name: &str) -> Option<String> {
    let (str, nullable) = _rust_to_postgres_type(field_type)?;

    Some(if nullable && field_name != "id" {
        format!("{} NULL", str)
    } else {
        format!("{} NOT NULL", str)
    })
}

fn _rust_to_postgres_type(field_type: &Type) -> Option<(String, bool)> {
    let path = get_type_string(field_type);

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
        "OneToOne" => {
            let target_entity = extract_generic_type(field_type, 1).unwrap();
            return Some((
                format!(
                    "oid REFERENCES {}(id)",
                    string_to_table_name(target_entity.into_token_stream().to_string())
                ),
                false,
            ));
        }
        "ManyToOne" => {
            let target_entity = extract_generic_type(field_type, 1).unwrap();
            return Some((
                format!(
                    "oid REFERENCES {}(id)",
                    string_to_table_name(target_entity.into_token_stream().to_string())
                ),
                false,
            ));
        }
        "OneToMany" => {
            return None;
        }
        "OneToOneRef" => {
            return None;
        }
        "Option" => {
            let (res, _) = _rust_to_postgres_type(&extract_generic_type(field_type, 1).unwrap())?;
            return Some((res, true));
        }
        "DateTime" => "timestamp with time zone",
        "NaiveDate" => "date",
        "NaiveTime" => "time",
        "Uuid" => "uuid",
        "Json" => "json",
        "Value" => "jsonb",
        "MacAddress" => "macaddr",
        _ => panic!("unsupported type {}", path),
    };

    Some((column_type.to_string(), false))
}

pub(crate) fn ident_to_table_name(ident: &Ident) -> String {
    string_to_table_name(ident.to_string())
}

pub(crate) fn string_to_table_name(string: String) -> String {
    string.to_case(Case::Snake)
}
