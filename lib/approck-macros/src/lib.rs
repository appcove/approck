extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn http(input: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the function item
    let item_mod = syn::parse_macro_input!(item as syn::ItemMod);

    // Get the attribute struct
    let http_module_inner =
        match approck_compiler::http_macro::parse_http_module_inner(input.into(), item_mod) {
            Ok(attribute) => attribute,

            // Once diagnostic is extracted, emit as ITEM (vs attr) tokens
            Err(e) => return e.get_diagnostic().emit_as_item_tokens().into(),
        };

    // Expand it
    let expanded = approck_compiler::http_macro::codegen::expand(http_module_inner);

    // Return the generated function
    expanded.into()
}
