use super::{Token, TokenError, TokenIter};
use crate::ReturnTypes;

// create a static message for http methods
pub const ERROR_EXPECTED_RETURN_TYPE: &str =
    "expected one of [Bytes, Text, Empty, HTML, JavaScript, CSS, JSON, SVG, NotFound, Redirect, WebSocketUpgrade, Stream]";

pub(super) fn parse(token_iter: &mut TokenIter) -> Result<ReturnTypes, TokenError> {
    let mut return_types = ReturnTypes::default();

    // Should be sitting on the `return` keyword at the beginning
    token_iter.get_ident_match("return")?;
    token_iter.step();

    loop {
        match token_iter.get_ident_as_string()?.as_str() {
            "Bytes" => {
                if return_types.Bytes {
                    return Err(token_iter.error("duplicate `Bytes` return type"));
                }
                return_types.Bytes = true;
                token_iter.step();
            }
            "Text" => {
                if return_types.Text {
                    return Err(token_iter.error("duplicate `Text` return type"));
                }
                return_types.Text = true;
                token_iter.step();
            }
            "Empty" => {
                if return_types.Empty {
                    return Err(token_iter.error("duplicate `Empty` return type"));
                }
                return_types.Empty = true;
                token_iter.step();
            }
            "HTML" => {
                if return_types.HTML {
                    return Err(token_iter.error("duplicate `HTML` return type"));
                }
                return_types.HTML = true;
                token_iter.step();
            }
            "JavaScript" => {
                if return_types.JavaScript {
                    return Err(token_iter.error("duplicate `JavaScript` return type"));
                }
                return_types.JavaScript = true;
                token_iter.step();
            }
            "CSS" => {
                if return_types.CSS {
                    return Err(token_iter.error("duplicate `CSS` return type"));
                }
                return_types.CSS = true;
                token_iter.step();
            }
            "JSON" => {
                if return_types.JSON {
                    return Err(token_iter.error("duplicate `JSON` return type"));
                }
                return_types.JSON = true;
                token_iter.step();
            }
            "SVG" => {
                if return_types.SVG {
                    return Err(token_iter.error("duplicate `SVG` return type"));
                }
                return_types.SVG = true;
                token_iter.step();
            }
            "NotFound" => {
                if return_types.NotFound {
                    return Err(token_iter.error("duplicate `NotFound` return type"));
                }
                return_types.NotFound = true;
                token_iter.step();
            }
            "Redirect" => {
                if return_types.Redirect {
                    return Err(token_iter.error("duplicate `Redirect` return type"));
                }
                return_types.Redirect = true;
                token_iter.step();
            }
            "WebSocketUpgrade" => {
                if return_types.WebSocketUpgrade {
                    return Err(token_iter.error("duplicate `WebSocketUpgrade` return type"));
                }
                return_types.WebSocketUpgrade = true;
                token_iter.step();
            }
            "Stream" => {
                if return_types.Stream {
                    return Err(token_iter.error("duplicate `Stream` return type"));
                }
                return_types.Stream = true;
                token_iter.step();
            }
            v => {
                return Err(token_iter
                    .error(format!("{}, not `{}`", ERROR_EXPECTED_RETURN_TYPE, v).as_str()));
            }
        }

        match token_iter.token() {
            // means take another
            Token::Pipe => {
                token_iter.step();
                continue;
            }
            // only way out of the loop
            Token::Semicolon => {
                token_iter.step();
                break;
            }
            _ => {
                return Err(token_iter.error("expected `|`, or `;`"));
            }
        }
    }

    // Note, semicolon was already verified above

    // return the request line
    Ok(return_types)
}

#[cfg(test)]
pub mod tests {
    use crate::ReturnTypes;
    use quote::quote;

    macro_rules! test_panic {
        ($name:ident, $param_tokens:expr, $panic_message:literal) => {
            #[test]
            #[should_panic(expected = $panic_message)]
            fn $name() {
                let mut token_iter = crate::tokenator::TokenIter::new($param_tokens);
                token_iter.step(); // advance it to the return statement
                match super::parse(&mut token_iter) {
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
                let mut token_iter = crate::tokenator::TokenIter::new($param_tokens);
                token_iter.step(); // advance it to the return statement
                match super::parse(&mut token_iter) {
                    Ok(v) => {
                        assert_eq!(v, $rval);
                    }
                    Err(e) => e.panic(),
                };
            }
        };
    }

    // panic on true empty no tokens
    test_panic!(test_return_empty_no_tokens, quote! {}, "expected `return`");

    // panic on empty
    test_panic!(test_return_empty, quote! { return; }, "expected Ident");

    // panic on missing ;
    test_panic!(
        test_return_no_semicolon,
        quote! { return Bytes },
        "expected `|`, or `;`"
    );

    // test on single return of Bytes
    test_return!(
        test_return_single_bytes,
        quote! { return Bytes; },
        ReturnTypes {
            Bytes: true,
            ..Default::default()
        }
    );

    // test on single return of Text
    test_return!(
        test_return_single_text,
        quote! { return Text; },
        ReturnTypes {
            Text: true,
            ..Default::default()
        }
    );

    // test on single return of Empty
    test_return!(
        test_return_single_empty,
        quote! { return Empty; },
        ReturnTypes {
            Empty: true,
            ..Default::default()
        }
    );

    // test on single return of HTML
    test_return!(
        test_return_single_html,
        quote! { return HTML; },
        ReturnTypes {
            HTML: true,
            ..Default::default()
        }
    );

    // test on single return of JavaScript
    test_return!(
        test_return_single_javascript,
        quote! { return JavaScript; },
        ReturnTypes {
            JavaScript: true,
            ..Default::default()
        }
    );

    // test on single return of CSS
    test_return!(
        test_return_single_css,
        quote! { return CSS; },
        ReturnTypes {
            CSS: true,
            ..Default::default()
        }
    );

    // test on single return of JSON
    test_return!(
        test_return_single_json,
        quote! { return JSON; },
        ReturnTypes {
            JSON: true,
            ..Default::default()
        }
    );

    // test on single return of SVG
    test_return!(
        test_return_single_svg,
        quote! { return SVG; },
        ReturnTypes {
            SVG: true,
            ..Default::default()
        }
    );

    // test on single return of NotFound
    test_return!(
        test_return_single_not_found,
        quote! { return NotFound; },
        ReturnTypes {
            NotFound: true,
            ..Default::default()
        }
    );

    // test on single return of Redirect
    test_return!(
        test_return_single_redirect,
        quote! { return Redirect; },
        ReturnTypes {
            Redirect: true,
            ..Default::default()
        }
    );

    // test on multiple return of Bytes, Text, Empty, HTML, JavaScript, CSS, JSON, SVG, NotFound, Redirect, WebSocketUpgrade
    test_return!(
        test_return_multiple,
        quote! { return Bytes | Text | Empty | HTML | JavaScript | CSS | JSON | SVG | NotFound | Redirect | WebSocketUpgrade | Stream; },
        ReturnTypes {
            Bytes: true,
            Text: true,
            Empty: true,
            HTML: true,
            JavaScript: true,
            CSS: true,
            JSON: true,
            SVG: true,
            NotFound: true,
            Redirect: true,
            WebSocketUpgrade: true,
            Stream: true,
        }
    );

    // panic on duplicate Bytes
    test_panic!(
        test_return_duplicate_bytes,
        quote! { return Bytes | Bytes; },
        "duplicate `Bytes` return type"
    );

    // panic on duplicate Text
    test_panic!(
        test_return_duplicate_text,
        quote! { return Text | Text; },
        "duplicate `Text` return type"
    );

    // panic on duplicate Empty
    test_panic!(
        test_return_duplicate_empty,
        quote! { return Empty | Empty; },
        "duplicate `Empty` return type"
    );

    // panic on duplicate HTML
    test_panic!(
        test_return_duplicate_html,
        quote! { return HTML | HTML; },
        "duplicate `HTML` return type"
    );

    // panic on duplicate JavaScript
    test_panic!(
        test_return_duplicate_javascript,
        quote! { return JavaScript | JavaScript; },
        "duplicate `JavaScript` return type"
    );

    // panic on duplicate CSS
    test_panic!(
        test_return_duplicate_css,
        quote! { return CSS | CSS; },
        "duplicate `CSS` return type"
    );

    // panic on duplicate JSON
    test_panic!(
        test_return_duplicate_json,
        quote! { return JSON | JSON; },
        "duplicate `JSON` return type"
    );

    // panic on duplicate SVG
    test_panic!(
        test_return_duplicate_svg,
        quote! { return SVG | SVG; },
        "duplicate `SVG` return type"
    );

    // panic on duplicate NotFound
    test_panic!(
        test_return_duplicate_not_found,
        quote! { return NotFound | NotFound; },
        "duplicate `NotFound` return type"
    );

    // panic on duplicate Redirect
    test_panic!(
        test_return_duplicate_redirect,
        quote! { return Redirect | Redirect; },
        "duplicate `Redirect` return type"
    );

    // panic on duplicate WebSocketUpgrade
    test_panic!(
        test_return_duplicate_websocket_upgrade,
        quote! { return WebSocketUpgrade | WebSocketUpgrade; },
        "duplicate `WebSocketUpgrade` return type"
    );

    // panic on Bytes | Text | Bytes
    test_panic!(
        test_return_duplicate_bytes_text,
        quote! { return Bytes | Text | Bytes; },
        "duplicate `Bytes` return type"
    );

    // Fail on ending with pipe;
    test_panic!(
        test_return_ending_pipe,
        quote! { return Bytes |; },
        "expected Ident"
    );
}
