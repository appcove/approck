use quote::{quote, TokenStreamExt};

pub fn quote_traits(trait_var: &str, trait_list: &[String]) -> proc_macro2::TokenStream {
    let paths: Vec<proc_macro2::TokenStream> = trait_list
        .iter()
        .map(|trait_name| {
            let path: syn::Path = syn::parse_str(trait_name).expect("Failed to parse trait path");
            quote!(#path)
        })
        .collect();

    let trait_ident = syn::Ident::new(trait_var, proc_macro2::Span::call_site());

    // since the valid syntax is either:
    //    <APP: Trait1 + Trait2 + Trait3>
    //    <APP: Trait1>
    //    <APP>
    // but specifically not
    //    <APP: >
    // we defer the `:` to be part of the first one printed
    // with subsequent ones being prefixed with `+`
    let mut quoted = quote!(#trait_ident);
    for (i, path) in paths.iter().enumerate() {
        if i == 0 {
            quoted.append_all(quote!(: #path));
        } else {
            quoted.append_all(quote!(+ #path));
        }
    }
    quoted.append_all(quote!(,));
    quoted
}
