use std::{process::Command, fmt::Error};

// Dev only build
fn main() -> std::io::Result<()> {
    // build client as static files
    //test
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=client/src/**");
    println!("cargo:rerun-if-changed=client/static/**");
    println!("cargo:rerun-if-changed=client/svelte.config.js");
    println!("cargo:rerun-if-changed=.env");

    if !check_program_installed("npm") {
        panic!("npm is not installed! install it first.");
    }

    let node_modules = std::path::Path::new("client/node_modules");
    if !node_modules.exists() {
        let _exit_status = Command::new("npm")
            .current_dir("client")
            .arg("install")
            .status()?;
    }

    // run npm build
    let _exit_status = Command::new("npm")
        .current_dir("client")
        .arg("run")
        .arg("build")
        .status()?;

    let env_file = std::path::Path::new(".env");
    if !env_file.exists() {
        let current_dir = std::env::current_dir()?;
        std::fs::write(
            ".env",
            format!(
                "DATABASE_URL = {0}\nSTATIC_FILE_PATH = {1}",
                current_dir.join("db.sqlite3").display(),
                current_dir.join("client/build").display()
            ),
        )?;
    }
    Ok(())
}

#[cfg(windows)]
fn check_program_installed(program: &str) -> bool {
    let output = Command::new("where")
        .arg(program)
        .output()
        .expect("failed to execute process");
    output.status.success()
}

#[cfg(unix)]
fn check_program_installed(program: &str) -> bool {
    let output = Command::new("which")
        .arg(program)
        .output()
        .expect("failed to execute process");
    output.status.success()
}