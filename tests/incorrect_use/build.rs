use std::process::Command;
//use colored::Colorize;

fn main() {
    // Two paths for release or debug
    // if in release mode, cargo dylint 
    // but idea is that dylint doesn't run the release mode branch of script, thus avoiding the loop

    // TODO(?) instead of toggling on release, toggle on optional flag that simply isn't passed to cargo dylint --all ?

    //println!("cargo::rerun-if-changed=./src/"); this is already the default
    let profile = std::env::var("PROFILE").unwrap();
    if profile.clone().as_str() == "release" {
        //println!("cargo::rustc-env=PROFILE=debug"); 
        let profile = std::env::var("PROFILE").unwrap();
        println!("profile changed? new: {}", profile); 

        let lint_res = Command::new("cargo")
            .arg("dylint")
            .arg("--all")
            //.arg("--path").arg("./../../linting/alohomora_type_derived/")
            .status().expect("cargo dylint failed"); 
        
        if !lint_res.success() {
            println!();
            // panic!("{}", "\nAlohomora lints failed!\n".red().bold()); // attempt to make colored panic
            panic!("\nAlohomora lints failed! See above. \n"); 
        }
    } 
}