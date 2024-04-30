use super::{Token, TokenError, TokenIter};

/// Objective of this function is to parse the contents of the request(...) function signature
/// It should be presented as a TokenIter, and the response should be a Vec<crate::Param>
/// An example valid input would be quote! { req: Request, path: Path, qs: QueryString, form: Option<PostForm> }
pub(super) fn parse(
    token_stream: proc_macro2::TokenStream,
) -> Result<Vec<crate::Param>, TokenError> {
    let mut token_iter = TokenIter::new(token_stream);
    let mut params = Vec::new();

    // move to the first token
    token_iter.step();

    // special case for no parameters, if at end, return immediately
    if token_iter.is_end() {
        return Ok(params);
    }

    let mut has_app = false;
    let mut has_postgres = false;
    let mut has_redis = false;
    let mut has_request = false;
    let mut has_document = false;
    let mut has_path = false;
    let mut has_query_string = false;
    let mut has_post_form = false;

    // Process parameters
    loop {
        if token_iter.is_mut() {
            return Err(
                token_iter.error("parameter mutability is automatically handled by the macro")
            );
        }

        let ident_name = token_iter.get_ident_as_string()?;
        token_iter.step();

        token_iter.get_colon()?;
        token_iter.step();

        // use get_ instead of take_ because some errors can occur on this token, which should be emitted before .step()
        match token_iter.get_ident_as_string()?.as_ref() {
            "App" => {
                if has_app {
                    // must come before .step() so that the error is indicated in the right place
                    return Err(token_iter.error("App parameter already exists"));
                }
                token_iter.step();
                has_app = true;

                let paths = token_iter.take_less_than_paths_greater_than()?;

                params.push(crate::Param {
                    param_name: ident_name,
                    param_type: crate::ParamType::App(paths),
                });
            }

            "Document" => {
                if has_document {
                    // must come before .step() so that the error is indicated in the right place
                    return Err(token_iter.error("Document parameter already exists"));
                }
                token_iter.step();
                has_document = true;

                params.push(crate::Param {
                    param_name: ident_name,
                    param_type: crate::ParamType::Document,
                });
            }
            "Postgres" => {
                if has_postgres {
                    // must come before .step() so that the error is indicated in the right place
                    return Err(token_iter.error("Postgres parameter already exists"));
                }
                token_iter.step();
                has_postgres = true;

                params.push(crate::Param {
                    param_name: ident_name,
                    param_type: crate::ParamType::Postgres,
                });
            }

            "Redis" => {
                if has_redis {
                    // must come before .step() so that the error is indicated in the right place
                    return Err(token_iter.error("Redis parameter already exists"));
                }
                token_iter.step();
                has_redis = true;

                params.push(crate::Param {
                    param_name: ident_name,
                    param_type: crate::ParamType::Redis,
                });
            }

            "Path" => {
                if has_path {
                    // must come before .step() so that the error is indicated in the right place
                    return Err(token_iter.error("Path parameter already exists"));
                }
                token_iter.step();
                has_path = true;

                params.push(crate::Param {
                    param_name: ident_name,
                    param_type: crate::ParamType::Path,
                });
            }
            "Request" => {
                if has_request {
                    // must come before .step() so that the error is indicated in the right place
                    return Err(token_iter.error("Request parameter already exists"));
                }
                token_iter.step();
                has_request = true;

                params.push(crate::Param {
                    param_name: ident_name,
                    param_type: crate::ParamType::Request,
                });
            }
            "QueryString" => {
                if has_query_string {
                    // must come before .step() so that the error is indicated in the right place
                    return Err(token_iter.error("QueryString parameter already exists"));
                }
                token_iter.step();
                has_query_string = true;

                params.push(crate::Param {
                    param_name: ident_name,
                    param_type: crate::ParamType::QueryString,
                });
            }
            "PostForm" => {
                if has_post_form {
                    // must come before .step() so that the error is indicated in the right place
                    return Err(token_iter.error("PostForm parameter already exists"));
                }
                token_iter.step();
                has_post_form = true;

                params.push(crate::Param {
                    param_name: ident_name,
                    param_type: crate::ParamType::PostForm,
                });
            }
            "Option" => {
                token_iter.step();
                token_iter.get_less_than()?;
                token_iter.step();

                match token_iter.get_ident_as_string()?.as_ref() {
                    "QueryString" => {
                        if has_query_string {
                            // must come before .step() so that the error is indicated in the right place
                            return Err(token_iter.error("QueryString parameter already exists"));
                        }
                        token_iter.step();

                        token_iter.get_greater_than()?;
                        token_iter.step();

                        has_query_string = true;

                        params.push(crate::Param {
                            param_name: ident_name,
                            param_type: crate::ParamType::QueryStringOption,
                        });
                    }
                    "PostForm" => {
                        if has_post_form {
                            // must come before .step() so that the error is indicated in the right place
                            return Err(token_iter.error("PostForm parameter already exists"));
                        }

                        token_iter.step();

                        token_iter.get_greater_than()?;
                        token_iter.step();

                        has_post_form = true;

                        params.push(crate::Param {
                            param_name: ident_name,
                            param_type: crate::ParamType::PostFormOption,
                        });
                    }
                    ident_name_inner => {
                        token_iter.step();

                        token_iter.get_greater_than()?;
                        token_iter.step();

                        params.push(crate::Param {
                            param_name: ident_name,
                            param_type: crate::ParamType::OptionalParam(
                                ident_name_inner.to_owned(),
                            ),
                        });
                    }
                }
            }
            _ => {
                return Err(token_iter
                    .error("expected `App<...>`, `DBCX`, `Document`, `Option<...>`, `Path`, `PostForm`, `QueryString`, `RedisCX`, or `Request`"));
            }
        }

        match token_iter.token() {
            Token::Comma => {
                token_iter.step();
            }
            Token::End => {
                break;
            }
            _ => {
                return Err(token_iter.error("expected `,` or end of input"));
            }
        }

        // End can also come after a comma, so check again
        if token_iter.is_end() {
            break;
        }

        // if not the end, loop and take the next argument
    }

    // return the request line
    Ok(params)
}

#[cfg(test)]
mod tests {
    use quote::quote;

    macro_rules! test_panic {
        ($name:ident, $param_tokens:expr, $panic_message:literal) => {
            #[test]
            #[should_panic(expected = $panic_message)]
            fn $name() {
                match super::parse($param_tokens) {
                    Ok(_) => {}
                    Err(e) => e.panic(),
                };
            }
        };
    }

    macro_rules! test_return {
        ($name:ident, $param_tokens:expr, $rval:expr) => {
            #[test]
            fn $name() {
                let rval = match super::parse($param_tokens) {
                    Ok(rval) => rval,
                    Err(e) => e.panic(),
                };
                assert_eq!(rval, $rval);
            }
        };
    }

    // test no parameters
    test_return!(test_no_params, quote! {}, Vec::new());

    // panic on invalid characters
    test_panic!(test_invalid_chars, quote! { 1 }, "expected Ident");

    // Panic on missing colon
    test_panic!(test_missing_colon, quote! { foo }, "expected `:`");

    // Panic on missing colon with comma
    test_panic!(
        test_missing_colon_with_comma,
        quote! { foo, },
        "expected `:`"
    );

    // Panic on missing type
    test_panic!(test_missing_type, quote! { foo: }, "expected Ident");

    // Panic on missing type with comma
    test_panic!(
        test_missing_type_with_comma,
        quote! { foo:, },
        "expected Ident"
    );

    // test for req: Request
    test_return!(
        test_request,
        quote! { req: Request },
        vec![crate::Param {
            param_name: "req".to_string(),
            param_type: crate::ParamType::Request,
        }]
    );

    // Test same with comma
    test_return!(
        test_request_with_comma,
        quote! { req: Request, },
        vec![crate::Param {
            param_name: "req".to_string(),
            param_type: crate::ParamType::Request,
        }]
    );

    // Test for Path
    test_return!(
        test_path,
        quote! { path: Path },
        vec![crate::Param {
            param_name: "path".to_string(),
            param_type: crate::ParamType::Path,
        }]
    );

    // Test for Request and Path
    test_return!(
        test_request_and_path,
        quote! { req: Request, path: Path },
        vec![
            crate::Param {
                param_name: "req".to_string(),
                param_type: crate::ParamType::Request,
            },
            crate::Param {
                param_name: "path".to_string(),
                param_type: crate::ParamType::Path,
            },
        ]
    );

    // Test for Option<foo>
    test_return!(
        test_option,
        quote! { foo: Option<String> },
        vec![crate::Param {
            param_name: "foo".to_string(),
            param_type: crate::ParamType::OptionalParam("String".to_string()),
        }]
    );

    // Test for Option<QueryString>
    test_return!(
        test_option_query_string,
        quote! { qs: Option<QueryString> },
        vec![crate::Param {
            param_name: "qs".to_string(),
            param_type: crate::ParamType::QueryStringOption,
        }]
    );

    // Test for Option<PostForm>
    test_return!(
        test_option_post_form,
        quote! { form: Option<PostForm> },
        vec![crate::Param {
            param_name: "form".to_string(),
            param_type: crate::ParamType::PostFormOption,
        }]
    );

    // Test for Option<PostForm> with comma and another param
    test_return!(
        test_option_post_form_with_comma,
        quote! { qs: Option<PostForm>, req: Request },
        vec![
            crate::Param {
                param_name: "qs".to_string(),
                param_type: crate::ParamType::PostFormOption,
            },
            crate::Param {
                param_name: "req".to_string(),
                param_type: crate::ParamType::Request,
            },
        ]
    );

    // Fail on duplicate QueryString
    test_panic!(
        test_duplicate_query_string,
        quote! { qs1: QueryString, qs2: QueryString },
        "QueryString parameter already exists"
    );

    // Fail on duplicate PostForm
    test_panic!(
        test_duplicate_post_form,
        quote! { form1: PostForm, form2: PostForm },
        "PostForm parameter already exists"
    );

    // Fail on duplicate Request
    test_panic!(
        test_duplicate_request,
        quote! { req1: Request, req2: Request },
        "Request parameter already exists"
    );

    // Fail on duplicate Path
    test_panic!(
        test_duplicate_path,
        quote! { path1: Path, path2: Path },
        "Path parameter already exists"
    );

    // Fail on QueryString and Option<QueryString>
    test_panic!(
        test_query_string_and_option,
        quote! { qs1: QueryString, qs2: Option<QueryString> },
        "QueryString parameter already exists"
    );

    // Fail on PostForm and Option<PostForm>
    test_panic!(
        test_post_form_and_option,
        quote! { form1: PostForm, form2: Option<PostForm> },
        "PostForm parameter already exists"
    );
}
