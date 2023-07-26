use core::panic;
use execute::Execute;
use std::{
    env, fs,
    fs::OpenOptions,
    io::BufWriter,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};
/*
This function should take the code from the host and the guest and generate a fully working RiscZero program with the predefined code in the host/guest.
TODO: Check that there are no errors & that everyting builds
*/
pub fn gen() {
    // 1. check that cargo-risczero is installed
    let is_risczero_installed = check_risczero_install();
    println!("Risc 0 installed: {:?}", is_risczero_installed);

    if !is_risczero_installed {
        panic!("Run cargo install cargo-risczero")
    }

    // 2. create an out directory
    // check that verifier dir exists, if not we're not in a Ceres project
    let dir_path = Path::new("verifier");
    if !dir_path.exists() || !dir_path.is_dir() {
        panic!("This does not seem to be a Ceres project directory");
    }

    let out_dir_path = dir_path.join("out");
    match fs::create_dir(&out_dir_path) {
        Ok(_) => println!("Created out folder"),
        Err(err) => panic!("Error creating folder: {}", err),
    }

    // create a risczero project
    let pwd: String = get_pwd_name();
    let substrings: Vec<&str> = pwd.split('/').collect();
    let project_name = substrings.last().unwrap();

    println!("Project name: {}", project_name);

    if let Err(err) = env::set_current_dir(&out_dir_path) {
        eprintln!("Failed to change working directory: {}", err);
    }

    let mut command = Command::new("cargo");

    command.arg("risczero").arg("new").arg(project_name);

    if command.execute_check_exit_status_code(0).is_err() {
        panic!("Failed to create risczero project")
    }

    // 3. update the Cargo.toml files with the correct name, from the main project dir name
    println!("Risc zero created");

    let file_path = Path::new(project_name).join("methods").join("Cargo.toml");

    let modified_name = format!("name = \"{}-methods\"", project_name);

    change_line(&file_path, 1, modified_name.as_str());

    let host_cargo_file_path = Path::new(project_name).join("host").join("Cargo.toml");
    update_host_method_import(&host_cargo_file_path, &project_name);
    
    // - methods/guest/Cargo.toml --> name = "<project-name>"
    let file_guest_path = Path::new(project_name)
    .join("methods")
    .join("guest")
    .join("Cargo.toml");

    let modified_content = format!("name = \"{}\"", project_name);
    
    change_line(&file_guest_path, 2, modified_content.as_str());
    use_std(&file_guest_path);
    
    // - host/src/main.rs --> raname use <project_name>-methods::{PROJECT_NAME-ELF, PROJECT_NAME_ID}
    let file_host_path = Path::new(project_name)
        .join("host")
        .join("src")
        .join("main.rs");

    let modified_host_line = format!(
        "use {}_methods::{{{}_ELF, {}_ID}};",
        project_name,
        project_name.to_uppercase(),
        project_name.to_uppercase()
    );

    change_line(&file_host_path, 3, modified_host_line.as_str());

    // 4. check the dependencies installed by the user -> install them to out
    let current_dir = env::current_dir().unwrap();
    let parent_dir = current_dir.parent().expect("Parent directory not found");

    if let Err(err) = env::set_current_dir(parent_dir) {
        eprintln!("Failed to change working directory: {}", err);
    }

    let installed_packages = get_installed_packages(Path::new("Cargo.toml"));
    add_installed_packages(
        &Path::new("out")
            .join(project_name)
            .join("host")
            .join("Cargo.toml"),
        installed_packages.host,
    );
    add_installed_packages(
        &Path::new("out")
            .join(project_name)
            .join("methods")
            .join("guest")
            .join("Cargo.toml"),
        installed_packages.guest,
    );

    // 5. Add premade code to host & guest
    prepare_guest_host_code(project_name);
    // 6. Add code from the user to host & guest
    add_guest_host_code(project_name);
}

fn check_risczero_install() -> bool {
    let crate_name = "cargo-risczero";
    let output = Command::new("cargo")
        .arg("install")
        .arg("--list")
        .output()
        .expect("Failed to check for cargo-risczero tool. Is cargo installed?");

    let installed_crates = String::from_utf8_lossy(&output.stdout);

    return installed_crates.contains(crate_name);
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

fn change_line(file_path: &PathBuf, line_num: usize, modified_line: &str) {
    let file_content: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect();

    let modified_content: Vec<String> = file_content
        .iter()
        .enumerate()
        .map(|(line_number, line)| {
            if line_number == line_num {
                modified_line.to_string()
            } else {
                line.to_string()
            }
        })
        .collect();

    let file = fs::File::create(&file_path).unwrap();
    let mut writer = BufWriter::new(file);

    // Write the modified content back to the file
    for line in modified_content {
        writeln!(writer, "{}", line).unwrap();
    }
}

fn use_std(file_path: &PathBuf) {
    // packages.guest.push(line.replace("default-features = false }", "default-features = false, features = [ \"std\" ] }"));
    let file_content: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect();

    let file = fs::File::create(&file_path).unwrap();
    let mut writer = BufWriter::new(file);

    for line in file_content {
        println!("Line contente: {}", line);

        if line.contains("risc0-zkvm") {
            writeln!(writer, "{}", line.replace("default-features = false", "default-features = false, features = [ \"std\" ]")).unwrap();
        } else {
            writeln!(writer, "{}", line).unwrap();
        }
    }
}

fn update_host_method_import(file_path: &Path, project_name: &str) {
    let file_content: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect();

    let file = fs::File::create(&file_path).unwrap();
    let mut writer = BufWriter::new(file);

    for line in file_content {
        if line.contains("methods = ") {
            writeln!(writer, "{}", line.replace("methods", &format!("{}-methods", project_name))).unwrap();
        } else {
            writeln!(writer, "{}", line).unwrap();
        }
    }
}

#[derive(Default)]
struct Packages {
    pub host: Vec<String>,
    pub guest: Vec<String>,
}

fn get_installed_packages(file_path: &Path) -> Packages {
    let file_content: Vec<String> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect();

    let mut packages = Packages::default();

    let mut collecting_host: bool = false;
    let mut collecting_guest: bool = false;

    for line in file_content {
        if line.contains("Host") {
            collecting_host = true;
        } else if line.contains("Guest") {
            collecting_host = false;
            collecting_guest = true;
            continue;
        }

        if collecting_host {
            packages.host.push(line);
        } else if collecting_guest {
            packages.guest.push(line);
        }
    }
    if !packages.guest.contains(&"cid = \"0.7.0\"".to_string()) {
        packages.guest.push("cid = \"0.7.0\"".to_string());
    }

    return packages;
}

fn add_installed_packages(file_path: &PathBuf, packages: Vec<String>) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)
        .unwrap();

    for package in packages {
        file.write_all(format!("{}\n", package).as_bytes())
            .expect("Failed to add package");
    }
}

fn prepare_guest_host_code(project_name: &str) {
    let host_file_path = &Path::new("out")
        .join(project_name)
        .join("host")
        .join("src")
        .join("main.rs");

    let host_file_content: Vec<String> = fs::read_to_string(host_file_path)
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect();

    let host_file = fs::File::create(&host_file_path).unwrap();
    let mut host_writer = BufWriter::new(host_file);

    let mut skip = false;

    // Write the modified content back to the file
    for line in host_file_content {
        if line.contains("fn main()") {
            writeln!(host_writer, "{}", line).unwrap();
            writeln!(host_writer, "{}", "    let args: Vec<String> = env::args().collect();").unwrap();
            writeln!(host_writer, "{}", "    let data: Vec<u8> = hostlib::prepare(args);\n").unwrap();
            writeln!(host_writer, "{}", "    let env = ExecutorEnv::builder().add_input(&to_vec(&data.as_slice()).unwrap()).build();\n").unwrap();
            let exec_cmd = format!("    let mut exec = Executor::from_elf(env, {}_ELF).unwrap();\n", project_name.to_uppercase());
            writeln!(host_writer, "{}", exec_cmd).unwrap();
            writeln!(host_writer, "{}", "    let session = exec.run().unwrap();").unwrap();
            writeln!(host_writer, "{}", "    let receipt = session.prove().unwrap();").unwrap();
            writeln!(host_writer, "{}", "    let cid: String = from_slice(&receipt.journal).unwrap();").unwrap();
            writeln!(host_writer, "{}", "    println!(\"Verified data with CID: {}\", cid);").unwrap();

            skip = true;
            continue;
        }

        if line.contains("}") {
            skip = false;
        }
        
        if skip {
            continue;
        }

        writeln!(host_writer, "{}", line).unwrap();

        if line.contains("ELF") && line.contains("ID") {
            writeln!(host_writer, "{}", "use std::env;").unwrap();
            writeln!(host_writer, "{}", "mod hostlib;").unwrap();
        }
    }

    let guest_file_path = &Path::new("out")
        .join(project_name)
        .join("methods")
        .join("guest")
        .join("src")
        .join("main.rs");
    let guest_file_content: Vec<String> = fs::read_to_string(guest_file_path)
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect();

    let guest_file = fs::File::create(&guest_file_path).unwrap();
    let mut guest_writer = BufWriter::new(guest_file);

    for line in guest_file_content {
        if line.contains("fn main()") {
            writeln!(guest_writer, "{}", line).unwrap();
            writeln!(guest_writer, "{}", "    let data: Vec<u8> = env::read();").unwrap();
            writeln!(guest_writer, "{}", "    guestlib::verify(&data);\n").unwrap();
            writeln!(guest_writer, "{}", "    const RAW: u64 = 0x55;").unwrap();
            writeln!(guest_writer, "{}", "    let h = Code::Sha2_256.digest(&data);").unwrap();
            writeln!(guest_writer, "{}", "    let cid = Cid::new_v1(RAW, h);").unwrap();
            writeln!(guest_writer, "{}", "    env::commit(&cid.to_string());").unwrap();
            writeln!(guest_writer, "{}", "    return ();").unwrap();

            skip = true;
            continue;
        }

        if line.contains("}") {
            skip = false;
        }
        
        if skip {
            continue;
        }

        if line.contains("no_std") {
            continue;
        }
        
        writeln!(guest_writer, "{}", line).unwrap();

        if line.contains("use risc0_zkvm::guest::env;") {
            writeln!(guest_writer, "{}", "use cid::multihash::{Code, MultihashDigest};\nuse cid::Cid;").unwrap();
        }

        if line.contains("risc0_zkvm::guest::entry!(main);") {
            writeln!(guest_writer, "{}", "mod guestlib;").unwrap();
        }
    }
}

fn add_guest_host_code(project_name: &str) {
    let guestlib_path = "src/guestlib.rs";
    let hostlib_path = "src/hostlib.rs";

    let host_dest_path = format!("out/{}/host/src/hostlib.rs", project_name);
    let guest_dest_path = format!("out/{}/methods/guest/src/guestlib.rs", project_name);

    fs::copy(guestlib_path, guest_dest_path).expect("Failed to copy guestlib");
    fs::copy(hostlib_path, host_dest_path).expect("Failed to copy hostlib");
}
