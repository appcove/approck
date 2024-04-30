use crate::{Method, PathPart, QueryStringPart, QueryStringValue, QueryStringValueType};
use quote::quote;

// ----------------------------------------------------------------------------
// Utility

macro_rules! get_item_mod {
    () => {{
        let tokens = quote! {
            pub mod foo {
                pub async fn request() -> Response {
                    Response::Empty
                }
            }
        };
        let item_mod: syn::ItemMod = syn::parse2(tokens).unwrap();
        item_mod
    }};
}

// ----------------------------------------------------------------------------
// General panic tests

macro_rules! test_panic {
    ($name:ident, $request_line:expr, $panic_message:literal) => {
        #[test]
        #[should_panic(expected = $panic_message)]
        fn $name() {
            match super::parse_http_module_inner($request_line, get_item_mod!()) {
                Ok(_) => {}
                Err(e) => e.panic(),
            };
        }
    };
}

// ----------------------------------------------------------------------------
// Test Method

macro_rules! test_method {
    ($name:ident, $request_line:expr, $method:expr) => {
        #[test]
        fn $name() {
            let http_function_inner =
                match super::parse_http_module_inner($request_line, get_item_mod!()) {
                    Ok(http_function_inner) => http_function_inner,
                    Err(e) => e.panic(),
                };
            assert_eq!(http_function_inner.methods, $method);
        }
    };
}

// fail a missing method
test_panic!(
    test_missing_method,
    quote! { /;  return Empty;},
    "expected http method (GET, POST, PUT, DELETE, PATCH)"
);

// fail on invalid type for method
test_panic!(
    test_invalid_type_for_method,
    quote! { 1 /;  return Empty;},
    "expected http method (GET, POST, PUT, DELETE, PATCH)"
);

// fail an invalid method
test_panic!(
    test_invalid_method,
    quote! { FOO /;  return Empty;},
    "expected http method (GET, POST, PUT, DELETE, PATCH)"
);

// fail a method that starts with |
test_panic!(
    test_method_starting_with_pipe,
    quote! { |GET /;  return Empty;},
    "expected http method (GET, POST, PUT, DELETE, PATCH)"
);

// fail a method that ends in |
test_panic!(
    test_method_ending_with_pipe,
    quote! { GET| /;  return Empty;},
    "expected http method (GET, POST, PUT, DELETE, PATCH)"
);

// fail a method that is duplicated
test_panic!(
    test_method_duplicated,
    quote! { GET|GET /;  return Empty; },
    "duplicate http method"
);

// fail a GET|POST|PUT|POST to see if it catches the dup
test_panic!(
    test_method_duplicated_2,
    quote! { GET|POST|PUT|POST /;  return Empty;},
    "duplicate http method"
);

// test a simple GET method
test_method!(
    test_method_get,
    quote! { GET /; return Empty;},
    vec![Method::GET]
);

// test a simple POST method
test_method!(
    test_method_post,
    quote! { POST /; return Empty;},
    vec![Method::POST]
);

// test a simple PUT method
test_method!(
    test_method_put,
    quote! { PUT /; return Empty;},
    vec![Method::PUT]
);

// test a simple DELETE method
test_method!(
    test_method_delete,
    quote! { DELETE /; return Empty; },
    vec![Method::DELETE]
);

// test a simple PATCH method
test_method!(
    test_method_patch,
    quote! { PATCH /; return Empty;},
    vec![Method::PATCH]
);

// test GET|POST
test_method!(
    test_method_get_post,
    quote! { GET|POST /; return Empty; },
    vec![Method::GET, Method::POST]
);

// ----------------------------------------------------------------------------
// Test Path

macro_rules! test_path {
    ($name:ident, $request_line:expr, $path:expr) => {
        #[test]
        fn $name() {
            let http_function_inner =
                match super::parse_http_module_inner($request_line, get_item_mod!()) {
                    Ok(http_function_inner) => http_function_inner,
                    Err(e) => e.panic(),
                };
            assert_eq!(http_function_inner.path, $path);
        }
    };
}

// create a test to check a missing path
test_panic!(
    test_missing_path,
    quote! { GET ; return Empty; },
    "expected `|` to add another http method, or `/` to start the path"
);

// create a test to check a path starting with a dot
test_panic!(
    test_path_starting_with_dot,
    quote! { GET .; return Empty; },
    "expected `|` to add another http method, or `/` to start the path"
);

// create a test to check a path starting with a dash
test_panic!(
    test_path_starting_with_dash,
    quote! { GET -; return Empty; },
    "expected `|` to add another http method, or `/` to start the path"
);

// create a test to check a path starting with an identifier
test_panic!(
    test_path_starting_with_ident,
    quote! { GET foo; return Empty; },
    "expected `|` to add another http method, or `/` to start the path"
);

// create a test to check a path that is not terminated
test_panic!(
    test_path_not_terminated,
    quote! { GET /foo/bar },
    "expected `/`, `?`, or `;`"
);

// create a test to check a path that is a simple /
test_path!(
    test_path_simple_slash,
    quote! { GET /; return Empty; },
    vec![(1, PathPart::Index)]
);

// create a test to check a path that is a simple /foo
test_path!(
    test_path_simple_foo,
    quote! { GET /foo; return Empty; },
    vec![(1, PathPart::Literal("foo".to_string()))]
);

// create a test to check a multipart path
test_path!(
    test_path_multipart,
    quote! { GET /foo/bar/baz; return Empty; },
    vec![
        (1, PathPart::Literal("foo".to_string())),
        (2, PathPart::Literal("bar".to_string())),
        (3, PathPart::Literal("baz".to_string()))
    ]
);

// test a path that ends in a slash
test_path!(
    test_path_ends_in_slash,
    quote! { GET /foo/bar/; return Empty; },
    vec![
        (1, PathPart::Literal("foo".to_string())),
        (2, PathPart::Literal("bar".to_string())),
        (3, PathPart::Index)
    ]
);

// test a path that ends in a slash with a query string
test_path!(
    test_path_ends_in_slash_with_query_string,
    quote! { GET /foo/bar/?baz=String; return Empty; },
    vec![
        (1, PathPart::Literal("foo".to_string())),
        (2, PathPart::Literal("bar".to_string())),
        (3, PathPart::Index)
    ]
);

// test a path that incorporates dashes
test_path!(
    test_path_with_dashes,
    quote! { GET /foo-bar/baz-a-b-c-d; return Empty; },
    vec![
        (1, PathPart::Literal("foo-bar".to_string())),
        (2, PathPart::Literal("baz-a-b-c-d".to_string()))
    ]
);

// test a path that incorporates dots, as in a file extension
test_path!(
    test_path_with_dots,
    quote! { GET /foo/bar/baz.js; return Empty; },
    vec![
        (1, PathPart::Literal("foo".to_string())),
        (2, PathPart::Literal("bar".to_string())),
        (3, PathPart::Literal("baz.js".to_string()))
    ]
);

// test a path that incorporates dots an early path segment
test_path!(
    test_path_with_dots_early,
    quote! { GET /foo.bar/baz; return Empty; },
    vec![
        (1, PathPart::Literal("foo.bar".to_string())),
        (2, PathPart::Literal("baz".to_string()))
    ]
);

// fail a path that ends in a dot
test_panic!(
    test_path_ends_in_dot,
    quote! { GET /foo/bar.; return Empty; },
    "expected Ident"
);

// fail a path that ends in a dash
test_panic!(
    test_path_ends_in_dash,
    quote! { GET /foo/bar-; return Empty; },
    "expected Ident"
);

// fail a path with double dashes
test_panic!(
    test_path_double_dash,
    quote! { GET /foo--bar; return Empty; },
    "expected Ident"
);

// pass a path with a capture
test_path!(
    test_path_with_capture,
    quote! { GET /foo/{id:i32}; return Empty; },
    vec![
        (1, PathPart::Literal("foo".to_string())),
        (
            2,
            PathPart::Capture {
                name: "id".to_string(),
                capture: crate::PathPartCapture::i32
            }
        )
    ]
);

// fail a path with a capture that is missing a type
test_panic!(
    test_path_with_capture_missing_type,
    quote! { GET /foo/{id:}; return Empty; },
    "expected Ident"
);

// fail a path capture with a missing colon
test_panic!(
    test_path_with_capture_missing_colon,
    quote! { GET /foo/{id}; return Empty; },
    "expected `:`"
);

// fail a path capture with an invalid type
test_panic!(
    test_path_with_capture_invalid_type,
    quote! { GET /foo/{id:foo}; return Empty; },
    "expected `i8`, `u8`, `i32`, `u32`, `i64`, `u64`, `usize`, or `String`"
);

// ----------------------------------------------------------------------------
// Test query string

macro_rules! test_query_string {
    ($name:ident, $request_line:expr, $query_string:expr) => {
        #[test]
        fn $name() {
            let mut token_iter = crate::tokenator::TokenIter::new($request_line);
            match super::macro_request_line::parse(&mut token_iter) {
                Ok(request_line) => {
                    assert_eq!(request_line.qs, $query_string);
                }
                Err(e) => e.panic(),
            };
        }
    };
}

// fail a query string that has no pairs
test_panic!(
    test_query_string_no_pairs,
    quote! { GET /?; },
    "expected query string name"
);

// fail a query string that has an equal but no value
test_panic!(
    test_query_string_no_value,
    quote! { GET /?foo=; },
    "expected Ident"
);

// fail a query string value that is invaild (e.g. bar)
test_panic!(
    test_query_string_invalid_value,
    quote! { GET /?foo=bar; },
    "expected String, i32, u32, i64, u64, f32, or f64"
);

// fail a query string that is in a Vec<> with an invalid value
test_panic!(
    test_query_string_vec_invalid_value,
    quote! { GET /?foo=Vec<bar>; },
    "expected String, i32, u32, i64, u64, f32, or f64"
);

// fail a query string that ends in an ampersand
test_panic!(
    test_query_string_ends_in_ampersand,
    quote! { GET /?foo=String&; },
    "expected query string name"
);

// fail a query string that is not terminated
test_panic!(
    test_query_string_not_terminated,
    quote! { GET /?foo=String },
    "expected `&`, or `;`"
);

// fail a query string with a dash in the name
test_panic!(
    test_query_string_dash_in_name,
    quote! { GET /?foo-bar=String; },
    "expected `=`, `&`, or `;`"
);

// check missing query string
test_query_string!(test_missing_query_string, quote! { GET /; }, None);

// test a query string with a single name
test_query_string!(
    test_query_string_single_name,
    quote! { GET /?foo; },
    Some(vec![QueryStringPart {
        name: "foo".to_string(),
        value: QueryStringValue::NoValue,
    }])
);

// test a query string with a single name and value
test_query_string!(
    test_query_string_single_name_value,
    quote! { GET /?foo=String; },
    Some(vec![QueryStringPart {
        name: "foo".to_string(),
        value: QueryStringValue::Require(QueryStringValueType::String),
    }])
);

// test a query string with multiple names and values
test_query_string!(
    test_query_string_multiple_names_values,
    quote! { GET /?foo=String&bar=i32; },
    Some(vec![
        QueryStringPart {
            name: "foo".to_string(),
            value: QueryStringValue::Require(QueryStringValueType::String),
        },
        QueryStringPart {
            name: "bar".to_string(),
            value: QueryStringValue::Require(QueryStringValueType::i32),
        },
    ])
);

// test a query string with a Vec<String>
test_query_string!(
    test_query_string_vec_string,
    quote! { GET /?foo=Vec<String>; },
    Some(vec![QueryStringPart {
        name: "foo".to_string(),
        value: QueryStringValue::Vec(QueryStringValueType::String),
    }])
);

// test a query string with a HashSet<String>
test_query_string!(
    test_query_string_hashset_string,
    quote! { GET /?foo=HashSet<String>; },
    Some(vec![QueryStringPart {
        name: "foo".to_string(),
        value: QueryStringValue::HashSet(QueryStringValueType::String),
    }])
);

// test a query string with an Option<i32>
test_query_string!(
    test_query_string_option_i32,
    quote! { GET /?foo=Option<i32>; },
    Some(vec![QueryStringPart {
        name: "foo".to_string(),
        value: QueryStringValue::Option(QueryStringValueType::i32),
    }])
);

// test a query string with underscores in the name
test_query_string!(
    test_query_string_underscores_in_name,
    quote! { GET /?foo_bar=String; },
    Some(vec![QueryStringPart {
        name: "foo_bar".to_string(),
        value: QueryStringValue::Require(QueryStringValueType::String),
    }])
);

// ----------------------------------------------------------------------------
// test derive_debug

macro_rules! test_derive_debug {
    ($name:ident, $request_line:expr, $derive_debug:expr) => {
        #[test]
        fn $name() {
            let http_function_inner =
                match super::parse_http_module_inner($request_line, get_item_mod!()) {
                    Ok(http_function_inner) => http_function_inner,
                    Err(e) => e.panic(),
                };
            assert_eq!(http_function_inner.derive_debug, $derive_debug);
        }
    };
}

// fail a derive_debug that is not terminated
test_panic!(
    test_derive_debug_not_terminated,
    quote! { GET /; derive_debug },
    "expected `;`"
);

// pass a missing derive debug=false
test_derive_debug!(
    test_derive_debug_false,
    quote! { GET /; return HTML; },
    false
);

// pass a derive_debug
test_derive_debug!(
    test_derive_debug,
    quote! { GET /; derive_debug; return HTML; },
    true
);

// fail a duplicate derive_debug
test_panic!(
    test_derive_debug_duplicate,
    quote! { GET /; derive_debug; derive_debug; return HTML; },
    "duplicate `derive_debug` instruction"
);
