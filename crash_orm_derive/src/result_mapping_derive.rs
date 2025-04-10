use crate::util::{extract_generic_type, get_type_string, is_relation};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

pub(crate) fn derive_result_mapping_impl(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let Data::Struct(struct_data) = derive_input.data else {
        panic!("only structs are supported")
    };

    let ident = derive_input.ident;
    let mut select_fields = quote!();
    let mut all_index = 0usize;

    for field in struct_data.fields {
        let field_ident = field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        if is_relation(field_type) {
            let field_type_name = get_type_string(field_type);
            let field_type_name = if field_type_name == "Option" {
                get_type_string(&extract_generic_type(field_type, 1).unwrap())
            } else {
                field_type_name
            };

            if field_type_name == "OneToMany" {
                select_fields.extend(quote! {
                    #field_ident: crash_orm::prelude::OneToMany::new(),
                });

                continue;
            } else if field_type_name == "OneToOneRef" {
                select_fields.extend(quote! {
                    #field_ident: crash_orm::prelude::OneToOneRef::new(),
                });
                continue;
            }
        }

        select_fields.extend(quote! {
            #field_ident: row.try_get(#all_index).ok()?,
        });

        all_index += 1;
    }
    
    let output = quote! {
        impl crash_orm::result_mapping::ResultMapping for #ident {
            fn from_row(row: crash_orm::postgres::Row) -> Option<#ident> {
                Some(#ident {
                    #select_fields
                })
            }
        }
    };
    
    output.into()
}