pub(super) fn test(workspace: &'static crate::Workspace) {
    crate::process::call_cargo_limited(workspace, "test", vec![]);
}
