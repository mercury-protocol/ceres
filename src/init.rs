use core::panic;
use std::{env, fs, io::Write, path::{Path, PathBuf}};
use serde::Deserialize;
use serde_json;
use reqwest::{self, header::{HeaderMap, HeaderValue, USER_AGENT}};

#[derive(Debug, PartialEq, Eq)]
pub enum CollectorLang {
    GO,
    JS,
    PY
}

pub async fn init(main_name: &String, lang: CollectorLang) {
    match fs::create_dir(main_name) {
        Ok(_) => println!("Created {} folder", main_name),
        Err(err) => panic!("Error creating folder: {}", err),
    }
    let collector_dir_path = Path::new(main_name).join("collector");
    
    match fs::create_dir(&collector_dir_path) {
        Ok(_) => println!("Created collector folder"),
        Err(err) => panic!("Error creating folder: {}", err),
    }
    pull_code(lang, &collector_dir_path).await;

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

#[derive(Debug, Deserialize)]
struct RepositoryContents {
    name: String,
    download_url: String,
}

async fn pull_code(lang: CollectorLang, path: &PathBuf) {
    let current_dir = env::current_dir().unwrap();

    if let Err(err) = env::set_current_dir(&path) {
        eprintln!("Failed to change working directory: {}", err);
    }

    let repo_owner = "mercury-protocol";
    let repo_name = "ceres-p2p-helpers";
    let folder_path: &str;

    if lang == CollectorLang::GO {
        folder_path = "ceres-go";
    } else if lang == CollectorLang::JS {
        folder_path = "ceres-js";
    } else if lang == CollectorLang::PY {
        folder_path = "ceres-go";
    } else {
        panic!("unimplemented lang")
    }

    match fs::create_dir(folder_path) {
        Ok(_) => {
            if let Err(err) = env::set_current_dir(folder_path) {
                eprintln!("Failed to change working directory: {}", err);
            }
        },
        Err(err) => panic!("Error creating folder: {}", err),
    }

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("ceres/1.0"));


    let client = reqwest::Client::builder().default_headers(headers).build().unwrap();

    let url = format!(
        "https://api.github.com/repos/{}/{}/contents/{}",
        repo_owner, repo_name, folder_path
    );

    let response = client.get(&url).send().await.expect("error getting code folder");

    if response.status().is_success() {
        let json_str = response.text().await.unwrap();
        let contents: Vec<RepositoryContents> = serde_json::from_str(&json_str).expect("failed to deserialize repo contents");

        for item in contents {
            let file_response = client.get(&item.download_url).send().await.expect("failed to pull file");
            if file_response.status().is_success() {
                let file_content = file_response.bytes().await.unwrap().to_vec();
                let mut file = fs::File::create(&item.name).expect("failed to create file");
                file.write_all(&file_content).expect("failed to write file content");
            }
        }
    }

    if let Err(err) = env::set_current_dir(&current_dir) {
        eprintln!("Failed to change working directory: {}", err);
    }
}