/// This is the translation function between the CLI invocation and the back-end.
pub(super) fn clone(workspace: &'static crate::Workspace) {
    match crate::aws_sdk::clone(workspace) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

/// This is the translation function between the CLI invocation and the back-end.
pub(super) fn build(workspace: &'static crate::Workspace) {
    match crate::aws_sdk::build(workspace) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

/// This is the translation function between the CLI invocation and the back-end.
pub(super) fn clean(workspace: &'static crate::Workspace) {
    match crate::aws_sdk::clean(workspace) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
