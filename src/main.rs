use clap::{arg, Command};
use core::panic;

mod init;
mod gen;
mod build;
mod pr;

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
        Some(("new-pr", _)) => {
            pr::new_pr();
        }
        Some(("add-pr", sub_matches)) => {
            let repo = sub_matches.get_one::<String>("REPO").expect("Please provide the path to the pr.md file for the collector verifier");
            pr::add_pr(repo);
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
        Command::new("new-pr")
        .about("Prepare a pull request to the Mercury data collectors repo")
    )
    .subcommand(
        Command::new("add-pr")
        .about("Updates the Mercury data collectors repo with a new PR")
        .arg(arg!(<REPO> "Path to a pr.md file for a collector verifier project"))
        .arg_required_else_help(true)
    )
}
