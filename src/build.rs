/*
- Create the executable binary from the code
- generate the image ID for the guest
*/
use std::{env, fs, path::Path, process::Command};
use execute::Execute;

pub fn build() {
    let pwd: String = get_pwd_name();
    let substrings: Vec<&str> = pwd.split('/').collect();
    let project_name = substrings.last().unwrap();

    if let Err(err) = env::set_current_dir(Path::new("verifier").join("out").join(project_name)) {
        eprintln!("Failed to change working directory: {}", err);
    }

    let mut command = Command::new("cargo");
    command.arg("run").arg("--release").arg("get-img-id");

    let output = command.execute_output().unwrap();

    if let Some(exit_code) = output.status.code() {
        if exit_code == 0 {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        } else {
            panic!("Failed to build project");
        }
    }

    println!("Please save the IMAGE ID. You will need it when submitting the collector-verifier to Mercury");

    let old_name = "target/release/host";
    let new_name = format!("target/release/{}", project_name);
    fs::rename(old_name, new_name).expect("failed to rename executable");
    println!("Data verifier executable is located at ./verifier/out/{}/target/release/{}", project_name, project_name);

}

fn get_pwd_name() -> String {
    match env::current_dir() {
        Ok(current_dir) => {
            return String::from(current_dir.to_str().unwrap());
        }
        Err(_) => {
            return String::from("");
        }
    }
}