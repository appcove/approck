use super::super::{QueryArg, QueryBundleBuilder};
use crate::tokenator::{Token, TokenError, TokenIter};

/// the job of this function is to parse the ... part of:
///     `row = ... ;`
/// the `;` will be consumed by the caller
pub(super) fn parse_value_into_args(
    token_iter: &mut TokenIter,
    qbb: &mut QueryBundleBuilder,
) -> Result<(), TokenError> {
    let mut args: Vec<QueryArg> = Vec::new();

    // convert the token iter into a brace group iter
    let mut token_iter = token_iter.take_brace_group_iter()?;
    // step to the first token
    // TODO: when starting any token iterator, we should always step to the first token because it always needs done.
    token_iter.step();

    let mut arg_number = 0;
    while !token_iter.is_end() {
        arg_number += 1;

        // all args start as a dollar sign
        token_iter.take_dollar_sign()?;

        let arg_ident = match token_iter.token() {
            Token::Literal(lit) => {
                let lit = lit.to_string();
                if lit != arg_number.to_string() {
                    return Err(token_iter.error(format!("expected `${arg_number}`").as_str()));
                }
                token_iter.step();

                None
            }
            Token::Ident(ident) => {
                let ident = ident.to_owned();
                token_iter.step();
                Some(ident)
            }
            _ => {
                return Err(token_iter.error("expected ident"));
            }
        };

        token_iter.take_colon()?;

        let value_tokens = token_iter.take_token_stream_until_char_or_end(',')?;

        args.push(QueryArg {
            arg_number,
            arg_ident,
            value_tokens,
        });
    }

    qbb.args = Some(args);

    Ok(())
}

// TODO: unit tests
