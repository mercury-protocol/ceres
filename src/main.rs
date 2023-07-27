use clap::{arg, Command};
use core::panic;

mod init;
mod gen;
mod build;

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            let folder_name = sub_matches.get_one::<String>("NAME").expect("required");
            init::init(folder_name);
        }
        Some(("gen", _)) => {
            gen::gen();
        }
        Some(("build", _)) => {
            build::build();
        }
        Some(("test", sub_matches)) => {
            println!("{:?}", sub_matches);
        }
        Some(("push", sub_matches)) => {
            println!("{:?}", sub_matches);
        }
        _ => panic!("Unknown subcommand"),
    }
}

// Ceres
fn cli() -> Command {
    //     .get_matches_from(vec!["init", "gen", "build", "test", "push"])
    Command::new("mcycv")
    .about("Mercury Collector Verifier SDK for creating data collectors and verifiers for any data type")
    .subcommand_required(true)
    .allow_external_subcommands(true)
    .subcommand(
        Command::new("init")
        .about("Creates a new empty collector-verifier project folder")
        .arg(arg!(<NAME> "The name of the project"))
        .arg_required_else_help(true)
    )
    .subcommand(
        Command::new("gen")
        .about("Takes the verifier code and outputs a complete Risc0 program that can be used with Mercury")
    )
    .subcommand(
        Command::new("build")
        .about("Creates an executable binary from the verifier code")
    )
    .subcommand(
        Command::new("test")
        .about("Tests the verifier with the given data")
        .arg(arg!(<PATH> "Path to the data"))
        .arg_required_else_help(true)
    )
    .subcommand(
        Command::new("push")
        .about("Prepare and push a pull request the Mercury data collectors repo")
    )
}
