pub(super) fn fmt(workspace: &'static crate::Workspace) {
    println!("---- running dprint ----");
    crate::process::call_dprint(workspace, "fmt", vec![]);
    println!("---- running cargo fmt ----");
    crate::process::call_cargo_limited(workspace, "fmt", vec![]);
    println!("---- done ----")
}
