use std::process::Command;

fn main() {
    // TODO(?) instead of toggling on release, toggle on optional flag that simply isn't passed to cargo dylint --all ?

    let profile = std::env::var("PROFILE").unwrap();
    if profile.clone().as_str() == "release" {
        // TODO pass lint paths instead of using workspace.metadata ?
        let lint_res = Command::new("cargo")
            .arg("dylint")
            .arg("--all")
            .status()
            .expect("cargo dylint failed"); 
        
        if !lint_res.success() {
            panic!("\nAlohomora lints failed! See above.\n"); 
        }
    } 
}