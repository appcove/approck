pub(super) fn doc(workspace: &'static crate::Workspace) {
    crate::process::call_cargo_limited(workspace, "doc", vec![]);
}
