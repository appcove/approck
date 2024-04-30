use crate::QueryStringValue;
use quote::quote;
/*
    Processing a query string involves 3 steps:

    1. create Option<Type> local variables for each query string part
    2. iterate over the query string, assigning each part to the local variable
    3. create a QueryString struct to hold the local variables


*/

pub fn process(
    codegen_tokens: &mut super::CodeGenTokens,
    post_type_struct: &crate::PostTypeStruct,
) {
    let mut struct_assign = Vec::new();
    let mut var_assign = Vec::new();
    let mut match_arms = Vec::new();
    let mut validate_stmts = Vec::new();

    let qs_parts = &post_type_struct.query_string_parts;

    for qs_part in qs_parts {
        let name_string = syn::LitStr::new(&qs_part.name, proc_macro2::Span::call_site());
        let name_ident = syn::Ident::new(&qs_part.name, proc_macro2::Span::call_site());

        // handle validation
        match &qs_part.value {
            QueryStringValue::Require(query_string_value_type) => {
                let value_type = &query_string_value_type.get_type_token_stream();

                var_assign.push(quote! {
                    let mut #name_ident: Option<#value_type> = None;
                });

                let parsing_code = get_parse_token_stream(query_string_value_type);
                match_arms.push(quote! {
                    #name_string => {
                        #name_ident = Some(#parsing_code);
                    }
                });

                validate_stmts.push(quote! {
                    let #name_ident = match #name_ident {
                        Some(v) => v,
                        None => {
                            panic!("query string part '{}' is required", #name_string);
                        }
                    };
                });
            }
            QueryStringValue::Option(query_string_value_type) => {
                let value_type = &query_string_value_type.get_type_token_stream();

                var_assign.push(quote! {
                    let mut #name_ident: Option<#value_type> = None;
                });

                let parsing_code = get_parse_token_stream(query_string_value_type);
                match_arms.push(quote! {
                    #name_string => {
                        #name_ident = Some(#parsing_code);
                    }
                });
            }
            QueryStringValue::Vec(query_string_value_type) => {
                let value_type = &query_string_value_type.get_type_token_stream();

                var_assign.push(quote! {
                    let mut #name_ident: ::std::vec::Vec<#value_type> = ::std::vec::Vec::new();
                });

                let parsing_code = get_parse_token_stream(query_string_value_type);
                match_arms.push(quote! {
                    #name_string => {
                        #name_ident.push(#parsing_code);
                    }
                });
            }
            QueryStringValue::HashSet(query_string_value_type) => {
                let value_type = &query_string_value_type.get_type_token_stream();

                var_assign.push(quote! {
                    let mut #name_ident: ::std::collections::HashSet<#value_type> = ::std::collections::HashSet::new();
                });

                let parsing_code = get_parse_token_stream(query_string_value_type);
                match_arms.push(quote! {
                    #name_string => {
                        #name_ident.insert(#parsing_code);
                    }
                });
            }
            QueryStringValue::NoValue => {
                var_assign.push(quote! {
                    let mut #name_ident: bool = false;
                });

                match_arms.push(quote! {
                    #name_string => {
                        #name_ident = true;
                    }
                });
            }
        }

        struct_assign.push(quote! {
            #name_ident,
        });
    }

    let struct_tokens = &post_type_struct.token_stream;

    codegen_tokens.mod_items(quote! {
        // Place the original struct verbaitm
        #struct_tokens

        impl PostForm {
            pub async fn parse(req: &mut approck::server::Request<'_>) -> approck::Result<Self> {
                // create mutable default values for all query string elements
                #( #var_assign )*

                // iterate over the incomking key=value pairs and match them into the right mutable element
                for (k,v) in req.read_body_query_pairs().await {
                    match k.as_str() {
                        #( #match_arms )*
                        // key doesn't match any registered query string part
                        _ => {}
                    }
                }

                // Some require additional validation and transformation, e.g. if required
                #( #validate_stmts )*

                // Generate the output struct
                Ok(Self {
                    #( #struct_assign )*
                })
            }
        }
    });
}

pub fn get_parse_token_stream(
    query_string_value_type: &crate::QueryStringValueType,
) -> proc_macro2::TokenStream {
    use crate::QueryStringValueType;
    match query_string_value_type {
        QueryStringValueType::String => quote! { v.to_string() },
        QueryStringValueType::i32 => quote! { v.parse()? },
        QueryStringValueType::u32 => quote! { v.parse()? },
        QueryStringValueType::i64 => quote! { v.parse()? },
        QueryStringValueType::u64 => quote! { v.parse()? },
        QueryStringValueType::f32 => quote! { v.parse()? },
        QueryStringValueType::f64 => quote! { v.parse()? },
    }
}
