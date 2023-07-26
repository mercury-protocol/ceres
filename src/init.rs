use core::panic;
use std::{fs, io::Write, path::Path};

pub fn init(main_name: &String) {
    match fs::create_dir(main_name) {
        Ok(_) => println!("Created {} folder", main_name),
        Err(err) => panic!("Error creating folder: {}", err),
    }

    let collector_dir_path = Path::new(main_name).join("collector");

    match fs::create_dir(collector_dir_path) {
        Ok(_) => println!("Created collector folder"),
        Err(err) => panic!("Error creating folder: {}", err),
    }

    let verifier_dir_path = Path::new(main_name).join("verifier");

    match fs::create_dir(&verifier_dir_path) {
        Ok(_) => println!("Created verifier folder"),
        Err(err) => panic!("Error creating folder: {}", err),
    }

    match fs::File::create(&verifier_dir_path.join("Cargo.toml")) {
        Ok(mut file) => {
            let data = format!(
                "[package]\nname = \"{}\"\nversion = \"0.1.0\"\n edition = \"2021\"\n\n[dependencies]\n# Host - install host dependencies here, don't delete this line\n\n# Guest - install guest dependencies here, don't delete this line",
                main_name
            );
            match file.write_all(data.as_bytes()) {
                Ok(_) => println!("File written"),
                Err(err) => eprintln!("Error writing file: {}", err),
            }
        }
        Err(err) => panic!("Error creating file: {}", err),
    }

    match fs::File::create(&verifier_dir_path.join("README.md")) {
        Ok(_) => println!("README.md file created"),
        Err(err) => panic!("Error creating file: {}", err),
    }

    match fs::create_dir(&verifier_dir_path.join("src")) {
        Ok(_) => println!("Created verifier src folder"),
        Err(err) => panic!("Error creating folder: {}", err),
    }

    match fs::File::create(&verifier_dir_path.join("src").join("hostlib.rs")) {
        Ok(mut file) => {
            let data = format!(
                "pub fn prepare(args: Vec<String>) -> Vec<u8> {{ \n //write your host code here \n }}"
            );
            match file.write_all(data.as_bytes()) {
                Ok(_) => println!("File written"),
                Err(err) => eprintln!("Error writing file: {}", err),
            }
        }
        Err(err) => panic!("Error creating file: {}", err),
    }

    match fs::File::create(&verifier_dir_path.join("src").join("guestlib.rs")) {
        Ok(mut file) => {
            let data =
                format!("pub fn verify(data: &Vec<u8>) -> bool {{ \n //write your code here \n }}");
            match file.write_all(data.as_bytes()) {
                Ok(_) => println!("File written"),
                Err(err) => eprintln!("Error writing file: {}", err),
            }
        }
        Err(err) => panic!("Error creating file: {}", err),
    }

    match fs::File::create(&verifier_dir_path.join("src").join("main.rs")) {
        Ok(mut file) => {
            let data = format!("mod guestlib;\nmod hostlib;\n\nuse std::env;\n\nfn main() {{\n    let args: Vec<String> = env::args().collect();\n\n    //testing the host code\n    let file_bytes: Vec<u8> = hostlib::prepare(args);\n\n    //testing the guest code\n    let guest_verification_result: bool = guestlib::verify(&file_bytes);\n    println!(\"Guest verification result: {{:?}}\", guest_verification_result);\n}}");
            match file.write_all(data.as_bytes()) {
                Ok(_) => println!("File written"),
                Err(err) => eprintln!("Error writing file: {}", err),
            }
        }
        Err(err) => panic!("Error creating file: {}", err),
    }
}
