pub(super) fn init(workspace: &'static crate::Workspace) {
    // install `git config alias.ff 'merge --ff-only'`
    {
        let mut cmd = std::process::Command::new("git");
        cmd.current_dir(&workspace.path);
        cmd.arg("config");
        cmd.arg("alias.ff");
        cmd.arg("merge --ff-only");
        let status = cmd.status().expect("failed to execute process");
        if !status.success() {
            println!("Error: `git config alias.ff 'merge --ff-only'` failed");
            std::process::exit(1);
        }
    }

    // run `git lfs install` or die with a message
    {
        let mut cmd = std::process::Command::new("git");
        cmd.current_dir(&workspace.path);
        cmd.arg("lfs");
        cmd.arg("install");
        let status = cmd.status().expect("failed to execute process");
        if !status.success() {
            println!(
                "Error: `git lfs install` failed; Perhaps you need to install a `git-lfs` package?"
            );
            std::process::exit(1);
        }
    }

    // run `git lfs fetch`
    {
        let mut cmd = std::process::Command::new("git");
        cmd.current_dir(&workspace.path);
        cmd.arg("lfs");
        cmd.arg("fetch");
        let status = cmd.status().expect("failed to execute process");
        if !status.success() {
            println!("Error: `git lfs fetch` failed");
            std::process::exit(1);
        }
    }

    // run `git lfs checkout`
    {
        let mut cmd = std::process::Command::new("git");
        cmd.current_dir(&workspace.path);
        cmd.arg("lfs");
        cmd.arg("checkout");
        let status = cmd.status().expect("failed to execute process");
        if !status.success() {
            println!("Error: `git lfs checkout` failed");
            std::process::exit(1);
        }
    }
}
