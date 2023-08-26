use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, GenericArgument, Ident, parse_macro_input, Path, PathArguments, Type};

pub fn derive_query_impl(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let Data::Struct(struct_data) = derive_input.data else {
        panic!("only structs are supported")
    };

    let original_ident = derive_input.ident;
    let vis = derive_input.vis;

    let mut column_consts = quote!();

    for field in struct_data.fields {
        let field_ident = field.ident.unwrap();
        let field_ident_str = field_ident.to_string();
        let field_ident_upper = Ident::new(&*field_ident_str.to_uppercase(), field_ident.span());
        let field_type = extract_type_from_option(&field.ty).unwrap_or(field.ty);

        column_consts.extend(quote! {
            pub const #field_ident_upper: crash_orm::QueryColumn::<#field_type, #original_ident> = crash_orm::QueryColumn::<#field_type, #original_ident>::new(#field_ident_str);
        });
    }

    let ident = Ident::new(&*format!("{}Column", original_ident.to_string()), original_ident.span());

    let output = quote! {
        #vis struct #ident;

        impl #ident {
            #column_consts
        }

        #[crash_orm::async_trait::async_trait]
        impl crash_orm::QueryEntity<#original_ident> for #original_ident {}
    };

    output.into()
}

fn extract_type_from_option(ty: &Type) -> Option<Type> {
    fn path_is_option(path: &Path) -> bool {
        path.leading_colon.is_none()
            && path.segments.len() == 1
            && path.segments.iter().next().unwrap().ident == "Option"
    }

    Some(match ty {
        Type::Path(type_path) if type_path.qself.is_none() && path_is_option(&type_path.path) => {
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