pub(super) fn clean(workspace: &'static crate::Workspace) {
    crate::process::call_cargo_limited(workspace, "clean", vec![]);
}
