extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn pg_execute(input: TokenStream) -> TokenStream {
    match granite_compiler::postgres::QueryBundle::new(input.into()) {
        Ok(query_bundle) => query_bundle.pg_execute().into(),
        Err(e) => e.get_diagnostic().emit_as_item_tokens().into(),
    }
}

#[proc_macro]
pub fn pg_value(input: TokenStream) -> TokenStream {
    match granite_compiler::postgres::QueryBundle::new(input.into()) {
        Ok(query_bundle) => query_bundle.pg_value().into(),
        Err(e) => e.get_diagnostic().emit_as_item_tokens().into(),
    }
}

#[proc_macro]
pub fn pg_value_vec(input: TokenStream) -> TokenStream {
    match granite_compiler::postgres::QueryBundle::new(input.into()) {
        Ok(query_bundle) => query_bundle.pg_value_vec().into(),
        Err(e) => e.get_diagnostic().emit_as_item_tokens().into(),
    }
}

#[proc_macro]
pub fn pg_row(input: TokenStream) -> TokenStream {
    match granite_compiler::postgres::QueryBundle::new(input.into()) {
        Ok(query_bundle) => query_bundle.pg_row().into(),
        Err(e) => e.get_diagnostic().emit_as_item_tokens().into(),
    }
}

#[proc_macro]
pub fn pg_row_vec(input: TokenStream) -> TokenStream {
    match granite_compiler::postgres::QueryBundle::new(input.into()) {
        Ok(query_bundle) => query_bundle.pg_row_vec().into(),
        Err(e) => e.get_diagnostic().emit_as_item_tokens().into(),
    }
}
