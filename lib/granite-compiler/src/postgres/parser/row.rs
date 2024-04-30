use super::super::{FieldType, QueryBundleBuilder, RowField};
use crate::tokenator::{TokenError, TokenIter};

/// the job of this function is to parse the ... part of:
///     `row = ... ;`
/// the `;` will be consumed by the caller
pub(super) fn parse_value_into_row(
    token_iter: &mut TokenIter,
    qbb: &mut QueryBundleBuilder,
) -> Result<(), TokenError> {
    let mut fields: Vec<RowField> = Vec::new();

    // convert the token iter into a brace group iter
    let mut token_iter = token_iter.take_brace_group_iter()?;
    // step to the first token
    // TODO: when starting any token iterator, we should always step to the first token because it always needs done.
    token_iter.step();

    loop {
        let ident = token_iter.take_ident()?;
        token_iter.take_colon()?;
        let ty = match token_iter.get_ident_as_string()?.as_str() {
            "String" => {
                token_iter.step();
                FieldType::String
            }
            "bool" => {
                token_iter.step();
                FieldType::bool
            }
            "i32" => {
                token_iter.step();
                FieldType::i32
            }
            "i64" => {
                token_iter.step();
                FieldType::i64
            }
            t => return Err(token_iter.error(format!("invalid type: `{}`", t).as_str())),
        };

        token_iter.take_comma()?;

        fields.push(RowField { ident, ty });

        if token_iter.is_end() {
            break;
        }
    }

    qbb.row = Some(fields);

    Ok(())
}

// TODO: unit tests
