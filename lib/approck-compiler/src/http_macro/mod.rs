pub mod codegen;
pub mod macro_request_line;
pub mod macro_return_types;
pub mod post_form_struct;
pub mod request_function_return;
pub mod request_function_signature;

#[cfg(test)]
mod tests;

use crate::tokenator::{Token, TokenError, TokenIter};
use quote::ToTokens;
use syn::{spanned::Spanned, Visibility::Public};

/// Represents the request line of the attribute macro
/// request line comes first and ends with either a semicolon or the end of the attribute
/// Examples:
///   1.  #[approck::http]
///   2.  #[approck::http(GET .)]
///   3.  #[approck::http(GET|POST .)]
///   4.  #[approck::http(POST /index;)]
/// Note that #1 becomes #2
/// If the path is ommitted, it is assumed to be .

#[derive(Debug)]
struct RequestLine {
    methods: Vec<crate::Method>,
    path: Vec<(u8, crate::PathPart)>,
    qs: Option<Vec<crate::QueryStringPart>>,
}

#[allow(unused_macros)]
macro_rules! make_ident {
    ($name:expr) => {
        syn::Ident::new($name, proc_macro2::Span::call_site())
    };
}

/// Example of macro
///
/// ```text
/// #[approck::http(
///    GET|POST /foo/bar/{id:i32}/?baz=String;
///    return String;  
/// )]
/// ```

pub fn parse_http_module_inner(
    input: proc_macro2::TokenStream,
    item_mod: syn::ItemMod,
) -> Result<crate::HttpModuleInner, TokenError> {
    // Convert TokenStream to an iterator
    let mut token_iter = TokenIter::new(input);

    // This parses the request line, somthing like this.
    //   GET|POST /foo/bar/{id:i32}/?baz=String;
    // The token iter will be moved past the semicolon
    let request_line = self::macro_request_line::parse(&mut token_iter)?;

    // Set defaults
    let mut derive_debug = false;
    let mut return_types = None;

    // parse additional instructions
    loop {
        match token_iter.token() {
            Token::Ident(ident) => match ident.to_string().as_str() {
                "derive_debug" => {
                    if derive_debug {
                        return Err(token_iter.error("duplicate `derive_debug` instruction"));
                    }
                    derive_debug = true;
                    token_iter.step();

                    token_iter.get_semicolon()?;
                    token_iter.step();
                }
                "return" => {
                    if return_types.is_some() {
                        return Err(token_iter.error("duplicate `return` instruction"));
                    }
                    return_types = Some(self::macro_return_types::parse(&mut token_iter)?);
                }
                _ => {
                    return Err(
                        token_iter.error(format!("invalid instruction: `{}`", ident).as_str())
                    );
                }
            },
            Token::End => {
                break;
            }
            _ => {
                return Err(token_iter.error("expected `derive_debug`, or `return`"));
            }
        }
    }

    // read the return section
    let return_types = match return_types {
        Some(return_types) => return_types,
        None => {
            return Err(token_iter.error("missing `return` instruction"));
        }
    };

    // read the end
    token_iter.get_end()?;

    // done with this for now
    drop(token_iter);

    // ---- Now move on to parsing elements of the module itself ----

    // Get the module ident, name, and span (under the #[approck::http])
    let mod_ident = item_mod.ident.clone();
    let mod_name = mod_ident.to_string();
    let mod_span = item_mod.span();

    // Steal the items out of the module, because that is what will be used to re-assemble it
    // along with other pre and post content later
    let mod_items = match item_mod.content {
        Some((_, items)) => items,
        _ => {
            return Err(TokenError::new(mod_span, "module has no content"));
        }
    };

    // Define optional vars for any well-known items that may be found
    let mut mod_request_fn = None;
    let mut mod_post_type = crate::PostType::None;
    let mut mod_items_remaining = Vec::new();

    // Extract any well-named items
    for item in mod_items.into_iter() {
        match item {
            syn::Item::Fn(item_fn) if item_fn.sig.ident == "request" => {
                mod_request_fn = Some(item_fn);
            }
            syn::Item::Struct(item_struct) if item_struct.ident == "PostForm" => {
                if mod_post_type.is_filled() {
                    return Err(TokenError::new(
                        item_struct.span(),
                        format!("PostType already set to {:?}", mod_post_type).as_str(),
                    ));
                }
                mod_post_type =
                    crate::PostType::Struct(self::post_form_struct::parse(item_struct)?);
            }
            syn::Item::Enum(item_enum) if item_enum.ident == "PostForm" => {
                if mod_post_type.is_filled() {
                    return Err(TokenError::new(
                        item_enum.span(),
                        format!("PostType already set to {:?}", mod_post_type).as_str(),
                    ));
                }
                mod_post_type = crate::PostType::Enum(crate::PostTypeEnum {});
            }
            syn::Item::Fn(item_fn) if matches!(item_fn.vis, Public(_)) => {
                return Err(TokenError::new(
                    item_fn.span(),
                    "the only public functions allowed in this module are [request]",
                ));
            }
            item => {
                mod_items_remaining.push(item);
            }
        }
    }

    // Validate request function
    let mod_request_fn = match mod_request_fn {
        Some(mod_request_fn) => {
            if mod_request_fn.sig.asyncness.is_none() {
                return Err(TokenError::new(
                    mod_request_fn.span(),
                    "function must be async",
                ));
            }
            mod_request_fn
        }
        None => {
            return Err(TokenError::new(
                mod_span,
                "module has no `request` function",
            ));
        }
    };

    let mod_request_fn_params =
        self::request_function_signature::parse(mod_request_fn.sig.inputs.to_token_stream())?;

    let mod_request_fn_return =
        self::request_function_return::parse(mod_request_fn.sig.output.to_token_stream())?;

    Ok(crate::HttpModuleInner {
        methods: request_line.methods,
        path: request_line.path,
        query_string: request_line.qs,
        derive_debug,
        return_types,
        mod_ident,
        mod_name,
        mod_post_type,
        mod_request_fn,
        mod_items_remaining,
        mod_request_fn_params,
        mod_request_fn_return,
    })
}
