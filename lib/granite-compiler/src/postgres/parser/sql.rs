use std::collections::HashMap;

use self::sql_iter::SqlToken;
use super::super::QueryBundleBuilder;
use crate::tokenator::TokenError;

// create an array of identifers that should be quoted in postgresql
static SHOULD_QUOTE: [&str; 3] = ["user", "group", "order"];

/// ### parse the QUERY part of the macro input
/// This is a raw token stream of the whole query up to but not including the final `;`
/// * Must be able to convert `$user_id` into `$3`
/// * Must be able to distinguish keywords
pub(super) fn parse_sql_token_stream(
    token_stream: proc_macro2::TokenStream,
    qbb: &mut QueryBundleBuilder,
) -> Result<(), TokenError> {
    // items that need

    let mut query = String::new();
    let mut sql_iter = self::sql_iter::create(token_stream);

    let arg_map = {
        if let Some(args) = &qbb.args {
            args.iter()
                .map(|arg| {
                    if let Some(ident) = &arg.arg_ident {
                        (ident.to_string(), format!("${}", arg.arg_number))
                    } else {
                        (arg.arg_number.to_string(), format!("${}", arg.arg_number))
                    }
                })
                .collect()
        } else {
            HashMap::new()
        }
    };

    while let Some(token) = sql_iter.next() {
        match token {
            SqlToken::Keyword(kw) => {
                query.push_str(kw.as_str());
                query.push(' ');
            }
            SqlToken::Ident(ident) => {
                query.push_str(quote_pg_ident(&ident).as_str());
                query.push(' ');
            }
            SqlToken::Punct(punct, has_space_after) => {
                if punct == '$' {
                    if let Some(arg_name) = match sql_iter.peek() {
                        Some(SqlToken::LiteralInteger(lit)) => Some(lit.to_string()),
                        Some(SqlToken::Ident(ident)) => Some(ident.to_string()),
                        _ => None,
                    } {
                        if let Some(replacement) = arg_map.get(arg_name.as_str()) {
                            query.push_str(replacement.as_str());
                            query.push(' ');
                            sql_iter.next();
                            continue;
                        } else {
                            // TODO: error handling not available through peekable iterator
                            panic!("arg not found: {}", arg_name);
                        }
                    }
                }

                query.push(punct);
                if has_space_after {
                    query.push(' ');
                }
            }
            SqlToken::LiteralString(lit) => {
                query.push_str(quote_pg_string(&lit).as_str());
                query.push(' ');
            }
            SqlToken::LiteralInteger(lit) => {
                query.push_str(lit.as_str());
                query.push(' ');
            }
            SqlToken::LiteralFloat(lit) => {
                query.push_str(lit.as_str());
                query.push(' ');
            }
        }
    }

    qbb.query = Some(query);

    Ok(())
}

// TODO: unit tests

// TODO: fix
fn quote_pg_string(s: &str) -> String {
    format!("'{}'", s.replace('\'', "''"))
}

fn quote_pg_ident(s: &str) -> String {
    if SHOULD_QUOTE.contains(&s) {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

mod sql_iter {
    use crate::tokenator::TokenError;
    use proc_macro2::{Delimiter, TokenStream};
    use quote::ToTokens;
    use std::iter::Peekable;
    use syn::{parse2, Lit};

    fn is_all_uppercase(s: &str) -> bool {
        s.chars().all(|c| c.is_uppercase())
    }

    pub(super) fn create(tokens: proc_macro2::TokenStream) -> Peekable<SqlTokenIter> {
        SqlTokenIter::new(tokens).peekable()
    }

    pub(super) enum SqlToken {
        Keyword(String),
        Ident(String),
        Punct(char, bool),
        LiteralString(String),
        LiteralInteger(String),
        LiteralFloat(String),
    }

    pub(super) struct SqlTokenIter {
        iter_stack: Vec<SqlIter>,
        current_tree: Option<proc_macro2::TokenTree>,
    }

    impl SqlTokenIter {
        fn new(token_stream: TokenStream) -> Self {
            let mut iter_stack = Vec::with_capacity(8); // how much nesting do we get in SQL?
            iter_stack.push(SqlIter {
                delimiter: Delimiter::None,
                iter: token_stream.into_iter(),
            });
            Self {
                iter_stack,
                current_tree: None,
            }
        }

        #[allow(dead_code)]
        pub fn depth(&self) -> usize {
            self.iter_stack.len()
        }

        /// Attempt to generate an error based on the current token, but if it is None, then use the previous token if there was one.
        /// Note: this is a bit of a hack to get the red underline to not use call_site unless totally necessary.
        #[allow(dead_code)]
        pub fn error(&self, error: &str) -> TokenError {
            TokenError::new(
                match &self.current_tree {
                    Some(tree) => tree.span(),
                    None => proc_macro2::Span::call_site(),
                },
                error,
            )
        }
    }

    struct SqlIter {
        delimiter: Delimiter,
        iter: proc_macro2::token_stream::IntoIter,
    }

    impl Iterator for SqlTokenIter {
        type Item = SqlToken;

        fn next(&mut self) -> Option<Self::Item> {
            match self.iter_stack.last_mut() {
                Some(stack_item) => {
                    self.current_tree = stack_item.iter.next();
                    match &self.current_tree {
                        Some(proc_macro2::TokenTree::Ident(ident)) => {
                            if is_all_uppercase(&ident.to_string()) {
                                Some(SqlToken::Keyword(ident.to_string()))
                            } else {
                                Some(SqlToken::Ident(ident.to_string()))
                            }
                        }
                        Some(proc_macro2::TokenTree::Punct(punct)) => Some(SqlToken::Punct(
                            punct.as_char(),
                            punct.spacing() == proc_macro2::Spacing::Alone,
                        )),
                        Some(proc_macro2::TokenTree::Literal(lit)) => {
                            // parse with syn
                            match parse2::<Lit>(lit.to_token_stream()) {
                                Ok(Lit::Str(lit_str)) => {
                                    Some(SqlToken::LiteralString(lit_str.value()))
                                }
                                Ok(Lit::Int(lit_int)) => Some(SqlToken::LiteralInteger(
                                    lit_int.base10_digits().to_string(),
                                )),
                                Ok(Lit::Float(lit_float)) => Some(SqlToken::LiteralFloat(
                                    lit_float.base10_digits().to_string(),
                                )),
                                Ok(_) => {
                                    panic!("unexpected literal type");
                                }
                                Err(e) => {
                                    panic!("error parsing literal: {}", e);
                                }
                            }
                        }
                        Some(proc_macro2::TokenTree::Group(group)) => {
                            self.iter_stack.push(SqlIter {
                                delimiter: group.delimiter(),
                                iter: group.stream().into_iter(),
                            });
                            match group.delimiter() {
                                Delimiter::Brace => Some(SqlToken::Punct('{', false)),
                                Delimiter::Bracket => Some(SqlToken::Punct('[', false)),
                                Delimiter::Parenthesis => Some(SqlToken::Punct('(', false)),
                                Delimiter::None => Some(SqlToken::Punct(' ', false)),
                            }
                        }
                        None => {
                            let rval = match &stack_item.delimiter {
                                Delimiter::Brace => Some(SqlToken::Punct('}', true)),
                                Delimiter::Bracket => Some(SqlToken::Punct(']', true)),
                                Delimiter::Parenthesis => Some(SqlToken::Punct(')', true)),
                                Delimiter::None => Some(SqlToken::Punct(' ', true)),
                            };
                            self.iter_stack.pop();
                            rval
                        }
                    }
                }
                None => None,
            }
        }
    }
}
