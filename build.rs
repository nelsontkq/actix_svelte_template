use std::process::{Command};

// Dev only build
fn main() -> std::io::Result<()> {
    // build client as static files
    //test
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=client/src/**");
    println!("cargo:rerun-if-changed=client/static/**");
    println!("cargo:rerun-if-changed=client/svelte.config.js");
    // run npm build
    let _exit_status = Command::new("npm")
        .current_dir("client")
        .arg("run")
        .arg("build")
        .status()?;

    Ok(())
}
