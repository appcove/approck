use syn::{parse_file, Attribute};

pub(super) fn parse_all(path_hits: Vec<super::PathHit>) -> Vec<crate::HttpModule> {
    let mut rval = Vec::new();

    for path_hit in path_hits {
        rval.extend(parse(path_hit));
    }

    rval
}

pub(super) fn parse(path_hit: super::PathHit) -> Vec<crate::HttpModule> {
    // read path
    let code = std::fs::read_to_string(&path_hit.abs_path)
        .unwrap_or_else(|e| panic!("\n\nFailed to read file {:?}: {}\n\n", path_hit.abs_path, e));

    let ast = parse_file(&code).unwrap_or_else(|e| {
        panic!(
            "\n\nUnable to parse rust code in `{}`: {:#?}\n\n",
            path_hit.rel_path, e
        )
    });

    let mut pairs = vec![];

    // construct a syn::path::Path to "approck::http"
    let approck_http_path = syn::parse_str::<syn::Path>("approck::http").unwrap(); // okay because it is a literal

    for item in ast.items {
        if let syn::Item::Mod(item_mod) = item {
            let fn_name = item_mod.ident.to_string();

            let span = item_mod.ident.span();
            let fn_line = span.start().line;

            let mut approck_attrs = item_mod
                .attrs
                .iter()
                .filter(|attr| {
                    // this code says if it is an approck proc macro or not
                    syn_path_equal(attr.path(), &approck_http_path)
                })
                .collect::<Vec<&Attribute>>();

            match approck_attrs.len() {
                0 => continue,
                1 => {
                    let attr = approck_attrs.pop().unwrap();
                    pairs.push((attr.clone(), item_mod, fn_name, fn_line));
                }
                _ => panic!("Found more than one [approck::http] attribute on function `{}(...)` defined in `{}`", fn_name, path_hit.rel_path),
            }
        }
    }

    let mut rval = Vec::new();
    for (attr, item_mod, fn_name, fn_line) in pairs {
        let attr_token_stream: proc_macro2::TokenStream = attr.parse_args().unwrap_or_else(|_| {
            panic!(
                "Error parsing #[approck::http] on function `{}(...)` in file `{}`",
                fn_name, path_hit.rel_path
            )
        });

        let attribute = match crate::http_macro::parse_http_module_inner(
            attr_token_stream,
            item_mod,
        ) {
            Ok(attribute) => attribute,
            Err(e) => {
                eprintln!(
                        "Error parsing #[approck::http] on function `{}(...)` in file `{}` at span {:?}: {}",
                        fn_name,
                        path_hit.rel_path,
                        e.get_diagnostic(),
                        e.get_error_str()
                    );
                continue;
            }
        };

        let rel_path = path_hit.rel_path.to_owned();
        let rust_ident = format!("{}::{}", path_hit.rust_ident, fn_name);

        rval.push(crate::HttpModule {
            rel_path,
            fn_line,
            inner: attribute,
            rust_ident,
        });
    }

    rval
}

/// Utility function for implementing == for syn::Path
fn syn_path_equal(path1: &syn::Path, path2: &syn::Path) -> bool {
    if path1.segments.len() != path2.segments.len() {
        return false;
    }

    path1
        .segments
        .iter()
        .zip(path2.segments.iter())
        .all(|(seg1, seg2)| seg1.ident == seg2.ident)
}
