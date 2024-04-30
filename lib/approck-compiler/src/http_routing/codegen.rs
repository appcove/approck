use quote::quote;

pub fn quote_router(
    trait_list: &[String],
    route_tree: &super::route_tree::RouteTree,
    tspaths: &[(String, String)],
) -> proc_macro2::TokenStream {
    let match_arms = get_match_arms(1, route_tree);
    let error_404_code = get_404_code();

    let traits = crate::codegen::quote_traits("APP", trait_list);

    let mut tsarms = Vec::new();
    for (abs_path, web_path) in tspaths.iter() {
        let ext = match web_path.rsplit_once('.') {
            Some((_, ext)) => ext,
            None => "",
        };

        match ext {
            "js" => {
                tsarms.push(quote! {
                    #web_path => {
                        let text = include_str!(#abs_path);
                        return Ok(approck::server::response::Response::JavaScript(text.into()));
                    }
                });
            }
            "css" => {
                tsarms.push(quote! {
                    #web_path => {
                        let text = include_str!(#abs_path);
                        return Ok(approck::server::response::Response::CSS(text.into()));
                    }
                });
            }
            "map" => {
                tsarms.push(quote! {
                    #web_path => {
                        let text = include_str!(#abs_path);
                        return Ok(approck::server::response::Response::JSON(text.into()));
                    }
                });
            }
            _ => {
                panic!("0x234234234; Unexpected file extension: {}", ext);
            }
        }
    }

    quote!(
        #[allow(clippy::match_single_binding,unused_variables)]
        pub async fn router<'a, #traits>(app: &'static APP, req: approck::server::Request<'a>) -> approck::Result<approck::server::response::Response> {

            #[allow(clippy::single_match)]
            match req.path() {
                #(#tsarms)*
                _ => {}
            }

            let path_vec = req.path_chunks();
            let mut path_parts = path_vec.into_iter();
            match &path_parts.next() {
                #(#match_arms),*
                _ => {#error_404_code}
            }
        }
    )
}

fn get_match_arms(
    depth: i32,
    route_tree: &super::route_tree::RouteTree,
) -> Vec<proc_macro2::TokenStream> {
    let mut match_arms = Vec::new();
    for (path_part, route_tree) in &route_tree.route_tree {
        let function = &route_tree.function;

        let match_item = match (path_part, function) {
            // Index with a function (e.g. only a handler)
            (crate::PathPart::Index, Some(function)) => {
                let function_call_code = get_function_call_code(function);
                let error_404_code = get_404_code();
                quote!(
                    Some("") => {
                        match &path_parts.next() {
                            None => {#function_call_code}
                            _ => {#error_404_code}
                        }
                    }
                )
            }
            // Literal with a function (e.g. either a handler or recurse deeper)
            (crate::PathPart::Literal(literal), Some(function)) => {
                let function_call_code = get_function_call_code(function);
                let sub_match_arms = get_match_arms(depth + 1, route_tree);
                let error_404_code = get_404_code();

                quote!(
                    Some(#literal) => match &path_parts.next() {
                        None => {#function_call_code}
                        #(#sub_match_arms)*
                        _ => {#error_404_code}
                    }
                )
            }
            // Literal without a function (e.g. just a directory, no handler)
            (crate::PathPart::Literal(literal), None) => {
                let sub_match_arms = get_match_arms(depth + 1, route_tree);
                let error_404_code = get_404_code();
                quote!(
                    Some(#literal) => match &path_parts.next() {
                        #(#sub_match_arms)*
                        _ => {#error_404_code}
                    }
                )
            }

            // Capture with a function (e.g. either a handler or recurse deeper)
            (crate::PathPart::Capture { name: _, capture }, optional_function) => {
                let sub_match_arms = get_match_arms(depth + 1, route_tree);
                let error_404_code = get_404_code();

                // create an ident called capture_{capture_name}
                let capture_ident =
                    syn::parse_str::<syn::Ident>(&format!("capture_{}", depth)).unwrap();

                let (guard, guard_conversion) = match capture {
                    // Strings don't have a guard
                    crate::PathPartCapture::String { .. } => (
                        quote! { if !#capture_ident.is_empty() },
                        quote! { let #capture_ident = #capture_ident.to_string(); },
                    ),

                    // Integers need cast into their respective types
                    crate::PathPartCapture::i8 { .. } => (
                        quote! { if #capture_ident.parse::<i8>().is_ok() },
                        quote! { let #capture_ident = #capture_ident.parse::<i8>()?; },
                    ),
                    crate::PathPartCapture::u8 { .. } => (
                        quote! { if #capture_ident.parse::<u8>().is_ok() },
                        quote! { let #capture_ident = #capture_ident.parse::<u8>()?; },
                    ),
                    crate::PathPartCapture::i32 { .. } => (
                        quote! { if #capture_ident.parse::<i32>().is_ok() },
                        quote! { let #capture_ident = #capture_ident.parse::<i32>()?; },
                    ),
                    crate::PathPartCapture::u32 { .. } => (
                        quote! { if #capture_ident.parse::<u32>().is_ok() },
                        quote! { let #capture_ident = #capture_ident.parse::<u32>()?; },
                    ),
                    crate::PathPartCapture::i64 { .. } => (
                        quote! { if #capture_ident.parse::<i64>().is_ok() },
                        quote! { let #capture_ident = #capture_ident.parse::<i64>()?; },
                    ),
                    crate::PathPartCapture::u64 { .. } => (
                        quote! { if #capture_ident.parse::<u64>().is_ok() },
                        quote! { let #capture_ident = #capture_ident.parse::<u64>()?; },
                    ),
                    crate::PathPartCapture::usize { .. } => (
                        quote! { if #capture_ident.parse::<usize>().is_ok() },
                        quote! { let #capture_ident = #capture_ident.parse::<usize>()?; },
                    ),
                };

                match optional_function {
                    Some(function) => {
                        let function_call_code = get_function_call_code(function);
                        quote! (
                            Some(#capture_ident) #guard => {
                                #guard_conversion
                                match &path_parts.next() {
                                    #(#sub_match_arms)*
                                    None => {#function_call_code}
                                    _ => {#error_404_code}
                                }
                            }
                        )
                    }
                    None => {
                        quote!(
                            Some(#capture_ident) #guard => {
                                guard_conversion
                                match &path_parts.next() {
                                    #(#sub_match_arms)*
                                    _ => {#error_404_code}
                                }
                            }
                        )
                    }
                }
            }

            // No other valid combinations exist
            _ => {
                panic!(
                    "0x4353453423; Unexpected combination of path_part {:?} function {:?}",
                    path_part,
                    match function {
                        Some(function) => function.rust_ident.clone(),
                        None => "None".to_string(),
                    }
                );
            }
        };

        match_arms.push(match_item);
    }

    match_arms
}

fn get_function_call_code(function: &crate::HttpModule) -> proc_macro2::TokenStream {
    let wrapper_ident =
        syn::parse_str::<syn::Path>(&format!("{}::wrap", &function.rust_ident)).unwrap();

    let path_fields = function
        .inner
        .iter_path_captures()
        .map(|(level, _name, _capture)| {
            let var_ident = syn::Ident::new(
                &format!("capture_{}", level),
                proc_macro2::Span::call_site(),
            );
            quote! {
                #var_ident,
            }
        });

    quote!(
        #wrapper_ident(app, req, #(#path_fields)*).await
    )
}

fn get_404_code() -> proc_macro2::TokenStream {
    // TODO: instead return an Err(approck::Error)
    quote!(Ok(approck::server::response::Response::NotFound(
        approck::server::response::NotFound
    )))
}
