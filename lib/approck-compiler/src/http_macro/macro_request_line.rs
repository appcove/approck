use crate::{Method, PathPart, QueryStringPart, QueryStringValueType};

use super::{RequestLine, Token, TokenError, TokenIter};

//use syn::spanned::Spanned;
//use proc_macro2_diagnostics::SpanDiagnosticExt;

// create a static message for http methods
pub const ERROR_EXPECTED_HTTP_METHOD: &str = "expected http method (GET, POST, PUT, DELETE, PATCH)";

pub(super) fn parse(token_iter: &mut TokenIter) -> Result<RequestLine, TokenError> {
    let mut methods = Vec::new();
    let mut path = Vec::new();
    let mut qs = None;

    // Process METHOD(|METHOD)*
    loop {
        token_iter.step();

        let method = match token_iter
            .get_ident_as_string()
            .map_err(|_| token_iter.error(ERROR_EXPECTED_HTTP_METHOD))?
            .as_str()
        {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            "PATCH" => Method::PATCH,
            _ => return Err(token_iter.error(ERROR_EXPECTED_HTTP_METHOD)),
        };

        if methods.contains(&method) {
            return Err(token_iter.error("duplicate http method"));
        }

        methods.push(method);

        token_iter.step();

        // after a method:
        //  * a pipe indiicates another method
        //  * a dot indicates the start of the path
        //  * a slash indicates the start of the path
        match token_iter.token() {
            Token::Pipe => {
                continue;
            }
            Token::Slash => {
                break;
            }
            _ => {
                return Err(token_iter
                    .error("expected `|` to add another http method, or `/` to start the path"));
            }
        }
    }

    // no need to step because we already did it above

    // paths start with a slash
    token_iter.get_slash()?;
    token_iter.step();

    // process path segments
    let mut level = 0;
    loop {
        level += 1;
        match token_iter.token() {
            Token::Ident(ident) => {
                let mut literal = ident.to_string();

                loop {
                    token_iter.step();
                    if let Some(c) = token_iter.get_dash_dot_underscore_option() {
                        literal.push(c);
                    } else {
                        break;
                    }
                    token_iter.step();
                    literal.push_str(token_iter.get_ident_as_string()?.as_str());
                }
                // note: in the logic above, the last thing that happens is token_iter.step();

                path.push((level, PathPart::Literal(literal)));
            }
            // look for path component
            Token::Group(grp) => {
                let sub_token_iter = &mut TokenIter::new(grp.stream());
                sub_token_iter.step();

                let name = sub_token_iter.get_ident_as_string()?;
                sub_token_iter.step();

                sub_token_iter.get_colon()?;
                sub_token_iter.step();

                let pp = match sub_token_iter.get_ident_as_string()?.as_str() {
                    "i8" => PathPart::Capture {
                        name,
                        capture: crate::PathPartCapture::i8,
                    },
                    "u8" => PathPart::Capture {
                        name,
                        capture: crate::PathPartCapture::u8,
                    },
                    "i32" => PathPart::Capture {
                        name,
                        capture: crate::PathPartCapture::i32,
                    },
                    "u32" => PathPart::Capture {
                        name,
                        capture: crate::PathPartCapture::u32,
                    },
                    "i64" => PathPart::Capture {
                        name,
                        capture: crate::PathPartCapture::i64,
                    },
                    "u64" => PathPart::Capture {
                        name,
                        capture: crate::PathPartCapture::u64,
                    },
                    "usize" => PathPart::Capture {
                        name,
                        capture: crate::PathPartCapture::usize,
                    },
                    "String" => PathPart::Capture {
                        name,
                        capture: crate::PathPartCapture::String,
                    },
                    _ => {
                        return Err(sub_token_iter.error(
                            "expected `i8`, `u8`, `i32`, `u32`, `i64`, `u64`, `usize`, or `String`",
                        ));
                    }
                };

                sub_token_iter.step();

                if !matches!(sub_token_iter.token(), Token::End) {
                    return Err(sub_token_iter.error("expected end of path component"));
                }

                path.push((level, pp));

                token_iter.step();
            }
            // these conditions cause it to pop out of the loop
            Token::QuestionMark | Token::Semicolon => {
                path.push((level, PathPart::Index));
                break;
            }

            _ => {
                return Err(token_iter.error("expected path segment, `?`, or `;`"));
            }
        }

        // no need to step as it was done above

        match token_iter.token() {
            // process another path segment
            Token::Slash => {
                token_iter.step();
                continue;
            }
            // signifies the end of the path
            Token::QuestionMark | Token::Semicolon => {
                break;
            }
            _ => {
                return Err(token_iter.error("expected `/`, `?`, or `;`"));
            }
        }
    }

    // process query string if there is one
    if token_iter.is_question_mark() {
        let mut qs_parts = Vec::new();

        // move past the question mark
        token_iter.step();

        loop {
            // `name` part of `name = value`
            let name = token_iter
                .get_ident_as_string()
                .map_err(|_| token_iter.error("expected query string name"))?;
            token_iter.step();

            // `=` part of `name = value`
            match token_iter.token() {
                Token::Equal => {
                    // move past the equals
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

                            let value_type =
                                extract_value_type(token_iter.get_ident_as_string()?.as_str())
                                    .map_err(|e| token_iter.error(e))?;
                            token_iter.step();

                            token_iter.get_greater_than()?;
                            token_iter.step();

                            qs_parts.push(QueryStringPart {
                                name,
                                value: crate::QueryStringValue::Option(value_type),
                            });
                        }
                        "Vec" => {
                            // move past the Vec
                            token_iter.step();

                            token_iter.get_less_than()?;
                            token_iter.step();

                            let value_type =
                                extract_value_type(token_iter.get_ident_as_string()?.as_str())
                                    .map_err(|e| token_iter.error(e))?;
                            token_iter.step();

                            token_iter.get_greater_than()?;
                            token_iter.step();

                            qs_parts.push(QueryStringPart {
                                name,
                                value: crate::QueryStringValue::Vec(value_type),
                            });
                        }
                        "HashSet" => {
                            // move past the HashSet
                            token_iter.step();

                            token_iter.get_less_than()?;
                            token_iter.step();

                            let value_type =
                                extract_value_type(token_iter.get_ident_as_string()?.as_str())
                                    .map_err(|e| token_iter.error(e))?;
                            token_iter.step();

                            token_iter.get_greater_than()?;
                            token_iter.step();

                            qs_parts.push(QueryStringPart {
                                name,
                                value: crate::QueryStringValue::HashSet(value_type),
                            });
                        }
                        v => {
                            let value_type =
                                extract_value_type(v).map_err(|e| token_iter.error(e))?;
                            token_iter.step();

                            qs_parts.push(QueryStringPart {
                                name,
                                value: crate::QueryStringValue::Require(value_type),
                            });
                        }
                    }
                }
                Token::Ampersand | Token::Semicolon => {
                    // don't move past the ampersand or semicolon, they will be processed later
                    qs_parts.push(QueryStringPart {
                        name,
                        value: crate::QueryStringValue::NoValue,
                    });
                }
                _ => return Err(token_iter.error("expected `=`, `&`, or `;`")),
            }

            // is there another query string part?
            match token_iter.token() {
                // this is the end
                Token::Semicolon => {
                    // don't move past the semicolon, it will be processed later
                    break;
                }
                // this is the start of another
                Token::Ampersand => {
                    // consume the ampersand and start over
                    token_iter.step();
                    continue;
                }
                _ => return Err(token_iter.error("expected `&`, or `;`")),
            }
        }

        // assign the query string parts
        qs = Some(qs_parts);
    }
    // end query string processing

    // consume the semicolon so the iterator is on the token following it
    token_iter.get_semicolon()?;
    token_iter.step();

    // return the request line
    Ok(RequestLine { methods, path, qs })
}
