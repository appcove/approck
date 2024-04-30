use super::{TokenError, TokenIter};
use crate::RequestFunctionReturnType;

// create a static message for http methods
pub const ERROR_EXPECTED_RETURN_TYPE: &str =
    "expected `String`, `HTML`, `JavaScript`, `JSON`, `CSS`, `SVG`, `NotFound`, `Redirect`, `WebSocketUpgrade`, or `Layout`";

pub(super) fn parse(
    token_stream: proc_macro2::TokenStream,
) -> Result<RequestFunctionReturnType, TokenError> {
    let mut token_iter = TokenIter::new(token_stream);

    // step to the first token
    token_iter.step();

    if token_iter.is_end() {
        return Err(token_iter.error("return type must be `Result<Response>`, or `Response`"));
    }

    // take a dash
    token_iter.get_dash()?;
    token_iter.step();

    // take a greater than
    token_iter.get_greater_than()?;
    token_iter.step();

    let rval = match token_iter.get_ident_as_string()?.as_str() {
        "Result" => {
            token_iter.step();
            token_iter.get_less_than()?;
            token_iter.step();
            token_iter.get_ident_match("Response")?;
            token_iter.step();
            token_iter.get_greater_than()?;
            token_iter.step();
            crate::RequestFunctionReturnType::ResultResponse
        }
        "Response" => {
            token_iter.step();
            crate::RequestFunctionReturnType::Response
        }
        other => {
            return Err(token_iter.error(&format!(
                "expected `Result<Response>`, or `Response`, not: `{}`",
                other
            )));
        }
    };

    // make sure no extra tokens
    token_iter.get_end()?;

    // return the request line
    Ok(rval)
}

#[cfg(test)]
mod tests {
    use super::parse;
    use crate::RequestFunctionReturnType;
    use quote::quote;

    macro_rules! test_panic {
        ($name:ident, $param_tokens:expr, $panic_message:literal) => {
            #[test]
            #[should_panic(expected = $panic_message)]
            fn $name() {
                match parse($param_tokens) {
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
                let rval = match parse($param_tokens) {
                    Ok(rval) => rval,
                    Err(e) => e.panic(),
                };
                assert_eq!(rval, $rval);
            }
        };
    }

    // panic on no return type
    test_panic!(
        no_return_type,
        quote! {},
        "return type must be `Result<Response>`, or `Response`"
    );

    // panic on no ->
    test_panic!(no_arrow, quote! { - }, "expected `>`");

    // Panic on invalid type
    test_panic!(
        invalid_type,
        quote! { -> foo },
        "expected `Result<Response>`, or `Response`, not: `foo`"
    );

    // panic on Result<foo>
    test_panic!(result_foo, quote! { -> Result<foo> }, "expected `Response`");

    // confirm Result<Response>
    test_return!(
        result_response,
        quote! { -> Result<Response> },
        RequestFunctionReturnType::ResultResponse
    );

    // confirm Response
    test_return!(
        response,
        quote! { -> Response },
        RequestFunctionReturnType::Response
    );

    // panic on extra tokens
    test_panic!(extra_tokens, quote! { -> Response + foo }, "expected end");
}
