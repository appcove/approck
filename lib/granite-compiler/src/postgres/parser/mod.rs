mod args;
mod row;
mod sql;

use crate::tokenator::{Token, TokenError, TokenIter};
use quote::ToTokens;
use syn::{parse2, Lit};

static ERR_EXPECTED: &str = r###"expected `args`, `db`, `row`, `WITH`, `SELECT`, `INSERT`, `UPDATE`, `DELETE`, or r#"...sql..."#"###;

/// the structure of the tokenstream is as follows: {
///     directive_1 = value_1;
///     directive_2 = value_2;
///     directive_n = value_n;     
///     QUERY
pub(super) fn parse_into_query_bundle(
    _input: proc_macro2::TokenStream,
) -> Result<super::QueryBundle, TokenError> {
    let mut qbb = super::QueryBundleBuilder::default();

    let mut token_iter = TokenIter::new(_input);
    token_iter.step();

    loop {
        // Uses .get_ instead of .take_ because there may yet be errors that need to report on this token
        match token_iter.token() {
            Token::Ident(ident) => match ident.to_string().as_ref() {
                "args" => {
                    if qbb.args.is_some() {
                        // must be before token_iter.step() so that the error is indicated in the right place
                        return Err(token_iter.error("duplicate `args` instruction"));
                    }
                    token_iter.step();
                    token_iter.take_equals()?;
                    self::args::parse_value_into_args(&mut token_iter, &mut qbb)?;
                    token_iter.take_semicolon()?;
                }
                "db" => {
                    if qbb.db_ident.is_some() {
                        // must be before token_iter.step() so that the error is indicated in the right place
                        return Err(token_iter.error("duplicate `db` instruction"));
                    }
                    token_iter.step();
                    token_iter.take_equals()?;
                    qbb.db_ident = Some(token_iter.take_ident()?);
                    token_iter.take_semicolon()?;
                }
                "row" => {
                    if qbb.row.is_some() {
                        // must be before token_iter.step() so that the error is indicated in the right place
                        return Err(token_iter.error("duplicate `row` instruction"));
                    }
                    token_iter.step();
                    token_iter.take_equals()?;
                    self::row::parse_value_into_row(&mut token_iter, &mut qbb)?;
                    token_iter.take_semicolon()?;
                }
                "WITH" | "SELECT" | "INSERT" | "UPDATE" | "DELETE" => {
                    if qbb.query.is_some() {
                        return Err(token_iter.error("duplicate query instruction"));
                    }
                    // This token is needed for sql parsing, so don't step

                    let token_stream = token_iter.take_token_stream_until_char_or_end(';')?;
                    self::sql::parse_sql_token_stream(token_stream, &mut qbb)?;

                    // make sure no instructions follow
                    if !token_iter.is_end() {
                        return Err(token_iter.error("expected end of query"));
                    }

                    break;
                }
                token => {
                    return Err(
                        token_iter.error(format!("invalid instruction: `{}`", token).as_str())
                    );
                }
            },
            // the only valid literal is r#"...sql..."#
            Token::Literal(lit) => {
                if qbb.query.is_some() {
                    return Err(token_iter.error("duplicate query instruction"));
                }

                let query = match parse2::<Lit>(lit.to_token_stream()) {
                    Ok(Lit::Str(lit_str)) => lit_str.value(),
                    Ok(_) => {
                        panic!("unexpected literal type");
                    }
                    Err(e) => {
                        panic!("error parsing literal: {}", e);
                    }
                };

                token_iter.step();
                token_iter.get_end()?;

                qbb.query = Some(query);
            }
            Token::End => {
                break;
            }
            _ => {
                return Err(token_iter.error(ERR_EXPECTED));
            }
        }
    }

    qbb.build()
}

#[cfg(test)]
mod tests {
    use quote::quote;

    macro_rules! test_panic {
        ($name:ident, $input:expr, $panic_message:literal) => {
            #[test]
            #[should_panic(expected = $panic_message)]
            fn $name() {
                match super::parse_into_query_bundle($input) {
                    Ok(_) => {}
                    Err(e) => e.panic(),
                };
            }
        };
    }

    macro_rules! test_ok {
        ($name:ident, $input:expr) => {
            #[test]
            fn $name() {
                match super::parse_into_query_bundle($input) {
                    Ok(_) => {}
                    Err(e) => e.panic(),
                };
            }
        };
    }

    // write a test that is oka
    test_ok!(
        test_ok_1,
        quote! {
            db = dbcx;
            row = { user_id: i32, first_name: String, last_name: String, };
            SELECT * FROM user;
        }
    );

    // panic because no row
    test_panic!(
        test_panic_1,
        quote! {
            db = dbcx;
            SELECT * FROM user;
        },
        "missing `row` instruction"
    );
}
