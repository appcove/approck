pub(super) fn clippy(workspace: &'static crate::Workspace) {
    crate::process::call_cargo_limited(workspace, "clippy", vec![]);
}
