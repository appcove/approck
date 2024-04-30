mod codegen;
mod filesystem;
mod inputfile;
mod route_tree;

use std::collections::HashSet;

use std::path::{Path, PathBuf};

#[allow(dead_code)]
#[cfg_attr(test, derive(Debug, Clone))]
struct PathHit {
    abs_path: PathBuf,
    rel_path: String,
    ext: Option<String>,
    crate_name: String,
    rust_ident: String,
}

#[derive(Debug)]
pub struct CrateInfo {
    pub name: String,
    pub ident: String,
    pub rel_path: String,
    pub abs_path: PathBuf,
}

/// Purpose of this function is to build the router and write it to disk as approck_generated.rs
pub fn build_and_quote_router_function(
    workspace_path: &Path,
    app_crate_name: &str,
    extended_crates: &[CrateInfo],
    tspaths: &[(String, String)],
) -> proc_macro2::TokenStream {
    // consumes the paths and reuturns a Vec<PathHit>
    let path_hits = self::filesystem::scan(workspace_path, app_crate_name, extended_crates);

    let function_list = inputfile::parse_all(path_hits);

    let trait_list = {
        let mut trait_set = HashSet::new();
        for function in function_list.iter() {
            trait_set.extend(function.inner.get_wrap_fn_app_trait_list())
        }

        let mut trait_list = trait_set.into_iter().collect::<Vec<_>>();
        trait_list.sort();
        trait_list
    };

    let route_tree = self::route_tree::compile_route_tree(function_list);

    self::codegen::quote_router(&trait_list, &route_tree, tspaths)
}
