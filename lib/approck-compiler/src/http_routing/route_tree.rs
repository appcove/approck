use crate::{HttpModule, PathPart};
use std::collections::HashMap;

pub struct RouteTree {
    pub function: Option<HttpModule>,
    pub route_tree: Vec<(crate::PathPart, RouteTree)>,
}

impl RouteTree {
    pub fn new() -> Self {
        Self {
            function: None,
            route_tree: Vec::new(),
        }
    }

    fn sort(self) -> RouteTree {
        let mut rval = RouteTree::new();
        rval.function = self.function;

        for (path_part, route_tree_builder) in self.route_tree {
            rval.route_tree.push((path_part, route_tree_builder.sort()));
        }

        rval
    }
}

/// Call this with the full list of functions and 0 as the level
fn extract_route_tree(function_list: Vec<HttpModule>, index: usize) -> RouteTree {
    let mut rval = RouteTree::new();
    let mut fmap: HashMap<PathPart, Vec<HttpModule>> = HashMap::new();

    // prepare fmap to use to recuse with
    for function in function_list {
        if function.inner.path.len() == index {
            if let Some(existing_function) = rval.function {
                panic!(
                    "\n\n0x8302928472; Conflicting functions at level {} path {:?}\nfunction is already in place: {:#?}\n\n",
                    index,
                    function.inner.path,
                    existing_function.inner.mod_name,
                );
            }

            rval.function = Some(function);
        } else {
            let (_level, path_part) = &function.inner.path[index];

            let fvec = {
                if let Some(fvec) = fmap.get_mut(path_part) {
                    fvec
                } else {
                    fmap.insert(path_part.clone(), Vec::new());
                    fmap.get_mut(path_part).unwrap()
                }
            };

            fvec.push(function);
        }
    }

    // convert to a vec and sort by the path_part (e.g. key)
    let mut flst: Vec<_> = fmap.into_iter().collect();
    flst.sort_by(|(a, _), (b, _)| a.cmp(b));

    // recurse
    for (path_part, function_list) in flst {
        rval.route_tree
            .push((path_part, extract_route_tree(function_list, index + 1)));
    }

    rval
}

pub fn compile_route_tree(function_list: Vec<HttpModule>) -> RouteTree {
    let route_tree = extract_route_tree(function_list, 0);

    // create a new copy of it, sorted
    route_tree.sort()
}
