use std::{fs, fs::OpenOptions, io, io::BufWriter, io::Write};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Default, Serialize, Deserialize, Debug)]
struct Pr {
    pub name: String,
    pub description: String,
    pub data_description: String,
    pub data_source: String,
    pub data_usefulness: String,
    pub code_explanation: String,
    pub image_id: String,
    pub email: String,
    pub source_code: String,
}

pub fn new_pr() {
    let stdin = io::stdin();
    let mut pr = Pr::default();

    println!("Please enter the name of your collector-verifier:");
    let mut name = String::new();
    stdin.read_line(&mut name).expect("Failed to read input");
    name = name.trim().to_string();
    pr.name = name;
    println!("\n");

    let mut length: usize = 0;
    while length == 0 || length > 280 {
        let mut desc = String::new();
        println!("Please add a short (max. 280 characters) description of what this collector-verifier does:");
        stdin.read_line(&mut desc).expect("Failed to read input");
        desc = desc.trim().to_string();
        length = desc.chars().count();
        pr.description = desc;
    }
    println!("\n");

    println!(
        "Please explain what data will be collected (type, structure, file format, size, etc.):"
    );
    let mut data_desc = String::new();
    stdin
        .read_line(&mut data_desc)
        .expect("Failed to read input");
    data_desc = data_desc.trim().to_string();
    pr.data_description = data_desc;
    println!("\n");

    println!("Please explain where this data will be collected from:");
    let mut data_source = String::new();
    stdin
        .read_line(&mut data_source)
        .expect("Failed to read input");
    data_source = data_source.trim().to_string();
    pr.data_source = data_source;
    println!("\n");

    println!("Please explain why collecting this data is useful ane where can it be used:");
    let mut usefulness = String::new();
    stdin
        .read_line(&mut usefulness)
        .expect("Failed to read input");
    usefulness = usefulness.trim().to_string();
    pr.data_usefulness = usefulness;
    println!("\n");

    println!("Please briefly explain how the collector and verifier programs work: ");
    let mut code_expl = String::new();
    stdin
        .read_line(&mut code_expl)
        .expect("Failed to read input");
    code_expl = code_expl.trim().to_string();
    pr.code_explanation = code_expl;
    println!("\n");

    println!("Please enter the Image ID of the verifier program:");
    let mut img_id = String::new();
    stdin.read_line(&mut img_id).expect("Failed to read input");
    img_id = img_id.trim().to_string();
    pr.image_id = img_id;
    println!("\n");

    println!("Please enter the link to a GitHub repo with the source code:");
    let mut source_code = String::new();
    stdin
        .read_line(&mut source_code)
        .expect("Failed to read input");
    source_code = source_code.trim().to_string();
    pr.source_code = source_code;
    println!("\n");

    println!("Please add an email where people can reach out about this collector-verifier:");
    let mut email = String::new();
    stdin.read_line(&mut email).expect("Failed to read input");
    email = email.trim().to_string();
    pr.email = email;
    println!("\n");

    let pr_file = fs::File::create("PR.md").unwrap();
    let mut writer = BufWriter::new(pr_file);
    writeln!(writer, "# {}   \n", pr.name).expect("failed to write line");
    writeln!(writer, "Image ID: {}   \n", pr.image_id).expect("failed to write");
    writeln!(writer, "Contact email: {}   \n", pr.email).expect("failed to write");

    writeln!(writer, "## Collector-verifier description:   ").expect("failed to write");
    writeln!(writer, "{}   \n", pr.description).expect("failed to write");

    writeln!(
        writer,
        "## Data to be collected: type, structure, file format, size, etc.:   "
    )
    .expect("failed to write");
    writeln!(writer, "{}    \n", pr.data_description).expect("failed to write");

    writeln!(writer, "## Data will be collected from:    ").expect("failed to write");
    writeln!(writer, "{}    \n", pr.data_source).expect("failed to write");

    writeln!(writer, "## This data is worth collecting because:   ").expect("failed to write");
    writeln!(writer, "{}    \n", pr.data_usefulness).expect("failed to write");

    writeln!(writer, "## Explanation of code:    ").expect("failed to write");
    writeln!(writer, "{}    \n", pr.code_explanation).expect("failed to write");

    let serialized_pr = serde_json::to_string(&pr).unwrap();
    let mut pr_info = fs::File::create(".prinfo").unwrap();
    pr_info.write_all(serialized_pr.as_bytes()).unwrap();


    println!("Success! PR.md file generated. Now please follow the following steps: ");
    println!("1. Run \"git clone https://github.com/mercury-protocol/mcy-data-collectors.git\"");
    println!("2. Go into the cloned repo and run \"ceres add-pr <PATH TO YOUR COLLECTOR REPO>\"");
    println!("3. Push your changes and submit a pull request where you paste in the contents of the PR.md file");
}

pub fn add_pr(repo: &String) {
    // open PR file and read the relevant lines
    let pr_info = fs::read_to_string(format!("{}/.prinfo", repo)).unwrap();

    let pr: Pr = serde_json::from_str(&pr_info).unwrap();

    // create the string that will be written 
    let pr_str = format!("--- \n\n# {} \n**ID**: {}\n**Image ID**: {}\n**Source code**: {}\n**Full PR text**: {}\n**Description**: {}", pr.name, " ",pr.image_id,pr.source_code," ", pr.description);

    // open the data-collectors.md file
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("data-collectors.md")
        .expect("Cant open data-collectors.md. Are you in the correct repo?");

    // write the string to the file
    file.write_all(pr_str.as_bytes()).expect("Failed to write to file");

    println!("Data collector list updated");
}