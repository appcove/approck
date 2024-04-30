use quote::ToTokens;

use super::{TokenError, TokenIter};
use crate::{PostTypeStruct, QueryStringPart, QueryStringValue, QueryStringValueType};

/// Objective of this function is to parse the contents of the request(...) function signature
/// It should be presented as a TokenIter, and the response should be a Vec<crate::Param>
/// An example valid input would be quote! { req: Request, path: Path, qs: QueryString, form: Option<PostForm> }
pub(super) fn parse(item_struct: syn::ItemStruct) -> Result<PostTypeStruct, TokenError> {
    let token_stream = item_struct.to_token_stream();
    let mut query_string_parts = Vec::new();

    for field in item_struct.fields {
        let mut token_iter = TokenIter::new(field.to_token_stream());

        // advance to first token
        token_iter.step();

        // `name` part of `name = value`
        let name = token_iter.get_ident_as_string()?;
        token_iter.step();

        // need a colon
        token_iter.get_colon()?;
        token_iter.step();

        let extract_value_type = |s: &str| match s {
            "String" => Ok(QueryStringValueType::String),
            "i32" => Ok(QueryStringValueType::i32),
            "u32" => Ok(QueryStringValueType::u32),
            "i64" => Ok(QueryStringValueType::i64),
            "u64" => Ok(QueryStringValueType::u64),
            "f32" => Ok(QueryStringValueType::f32),
            "f64" => Ok(QueryStringValueType::f64),
            _ => Err("expected String, i32, u32, i64, u64, f32, or f64"),
        };

        match token_iter.get_ident_as_string()?.as_str() {
            "Option" => {
                // move past the Option
                token_iter.step();

                token_iter.get_less_than()?;
                token_iter.step();

                let value_type = extract_value_type(token_iter.get_ident_as_string()?.as_str())
                    .map_err(|e| token_iter.error(e))?;
                token_iter.step();

                token_iter.get_greater_than()?;
                token_iter.step();

                query_string_parts.push(QueryStringPart {
                    name,
                    value: QueryStringValue::Option(value_type),
                });
            }
            "Vec" => {
                // move past the Vec
                token_iter.step();

                token_iter.get_less_than()?;
                token_iter.step();

                let value_type = extract_value_type(token_iter.get_ident_as_string()?.as_str())
                    .map_err(|e| token_iter.error(e))?;
                token_iter.step();

                token_iter.get_greater_than()?;
                token_iter.step();

                query_string_parts.push(QueryStringPart {
                    name,
                    value: QueryStringValue::Vec(value_type),
                });
            }
            "HashSet" => {
                // move past the HashSet
                token_iter.step();

                token_iter.get_less_than()?;
                token_iter.step();

                let value_type = extract_value_type(token_iter.get_ident_as_string()?.as_str())
                    .map_err(|e| token_iter.error(e))?;
                token_iter.step();

                token_iter.get_greater_than()?;
                token_iter.step();

                query_string_parts.push(QueryStringPart {
                    name,
                    value: QueryStringValue::HashSet(value_type),
                });
            }
            v => {
                token_iter.step();

                let value_type = extract_value_type(v).map_err(|e| token_iter.error(e))?;

                query_string_parts.push(QueryStringPart {
                    name,
                    value: QueryStringValue::Require(value_type),
                });
            }
        }
    }

    Ok(PostTypeStruct {
        token_stream,
        query_string_parts,
    })
}

#[cfg(test)]
mod tests {
    use super::{QueryStringValue, QueryStringValueType};
    use quote::quote;

    macro_rules! test_panic {
        ($name:ident, $param_tokens:expr, $panic_message:literal) => {
            #[test]
            #[should_panic(expected = $panic_message)]
            fn $name() {
                let param_tokens = $param_tokens;
                let item_struct: syn::ItemStruct =
                    match syn::parse2(quote! { struct PostForm { #param_tokens } }) {
                        Ok(item_struct) => item_struct,
                        Err(e) => panic!("parse2 error: {}", e),
                    };
                match super::parse(item_struct) {
                    Ok(_) => {}
                    Err(e) => e.panic(),
                };
            }
        };
    }

    macro_rules! test_return {
        ($name:ident, $param_tokens:expr, $query_string_parts:expr) => {
            #[test]
            fn $name() {
                let param_tokens = $param_tokens;
                let item_struct: syn::ItemStruct =
                    syn::parse2(quote! { struct PostForm { #param_tokens } }).unwrap();
                let rval = match super::parse(item_struct) {
                    Ok(rval) => rval,
                    Err(e) => e.panic(),
                };
                assert_eq!(rval.query_string_parts, $query_string_parts);
            }
        };
    }

    // test empty
    test_return!(test_empty, quote! {}, vec![]);

    // test invalid tokens
    test_panic!(
        test_invalid_tokens,
        quote! { 123 },
        "parse2 error: expected identifier"
    );

    // Test invalid data type
    test_panic!(
        test_invalid_data_type,
        quote! { foo: Bar },
        "expected String, i32, u32, i64, u64, f32, or f64"
    );

    // test invalid data type in Option
    test_panic!(
        test_invalid_data_type_in_option,
        quote! { foo: Option<Bar> },
        "expected String, i32, u32, i64, u64, f32, or f64"
    );

    // test String
    test_return!(
        test_string,
        quote! { foo: String },
        vec![super::QueryStringPart {
            name: "foo".to_string(),
            value: QueryStringValue::Require(QueryStringValueType::String),
        }]
    );

    // Test String with trailing comma
    test_return!(
        test_string_trailing_comma,
        quote! { foo: String, },
        vec![super::QueryStringPart {
            name: "foo".to_string(),
            value: QueryStringValue::Require(QueryStringValueType::String),
        }]
    );

    // Test i32
    test_return!(
        test_i32,
        quote! { foo: i32 },
        vec![super::QueryStringPart {
            name: "foo".to_string(),
            value: QueryStringValue::Require(QueryStringValueType::i32),
        }]
    );

    // Test Option<i32>
    test_return!(
        test_option_i32,
        quote! { foo: Option<i32> },
        vec![super::QueryStringPart {
            name: "foo".to_string(),
            value: QueryStringValue::Option(QueryStringValueType::i32),
        }]
    );

    // Test Vec<i32>
    test_return!(
        test_vec_i32,
        quote! { foo: Vec<i32> },
        vec![super::QueryStringPart {
            name: "foo".to_string(),
            value: QueryStringValue::Vec(QueryStringValueType::i32),
        }]
    );

    // Test HashSet<i32>
    test_return!(
        test_hashset_i32,
        quote! { foo: HashSet<i32> },
        vec![super::QueryStringPart {
            name: "foo".to_string(),
            value: QueryStringValue::HashSet(QueryStringValueType::i32),
        }]
    );

    // Test multiple
    test_return!(
        test_multiple,
        quote! { foo: String, bar: i32, baz: Option<u32>, qux: Vec<i64>, quux: HashSet<f32> },
        vec![
            super::QueryStringPart {
                name: "foo".to_string(),
                value: QueryStringValue::Require(QueryStringValueType::String),
            },
            super::QueryStringPart {
                name: "bar".to_string(),
                value: QueryStringValue::Require(QueryStringValueType::i32),
            },
            super::QueryStringPart {
                name: "baz".to_string(),
                value: QueryStringValue::Option(QueryStringValueType::u32),
            },
            super::QueryStringPart {
                name: "qux".to_string(),
                value: QueryStringValue::Vec(QueryStringValueType::i64),
            },
            super::QueryStringPart {
                name: "quux".to_string(),
                value: QueryStringValue::HashSet(QueryStringValueType::f32),
            },
        ]
    );
}
