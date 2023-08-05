use clap::{arg, Arg, ArgAction, Command};
use core::panic;

mod build;
mod gen;
mod init;
mod pr;

#[tokio::main]
async fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            let folder_name = sub_matches.get_one::<String>("NAME").expect("required");
            let lang: init::CollectorLang;

            if sub_matches.get_flag("go") {
                lang = init::CollectorLang::GO;
            } else if sub_matches.get_flag("js") {
                lang = init::CollectorLang::JS;
            } else if sub_matches.get_flag("py") {
                lang = init::CollectorLang::PY;
            } else {
                panic!("No lang specified")
            }

            init::init(folder_name, lang).await;
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
            let pr_file = sub_matches
                .get_one::<String>("PR_FILE")
                .expect("Please provide the path to the pr.md file for the collector verifier");
            pr::add_pr(pr_file);
        }
        _ => panic!("Unknown subcommand"),
    }
}

// Ceres
fn cli() -> Command {
    //     .get_matches_from(vec!["init", "gen", "build", "test", "push"])
    Command::new("ceres")
    .about("Ceres is an SDK for creating data collectors and verifiers for any data type to be sold on the Mercury Protocol")
    .subcommand_required(true)
    .allow_external_subcommands(true)
    .subcommand(
        Command::new("init")
        .about("Creates a new empty collector-verifier project folder")
        .arg(arg!(<NAME> "The name of the project"))
        .arg_required_else_help(true)
        .arg(
            Arg::new("go")
            .long("go")
            .action(ArgAction::SetTrue)
            .help("Initialize a Go collector package"),
        )
        .arg(
            Arg::new("js")
            .long("js")
            .action(ArgAction::SetTrue)
            .help("Initialize a JavaScript collector package")
        )
        .arg(
            Arg::new("py")
            .long("py")
            .action(ArgAction::SetTrue)
            .help("Initialize a Python collector package")
        )
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
        Command::new("new-pr")
        .about("Prepare a pull request to the Mercury data collectors repo")
    )
    .subcommand(
        Command::new("add-pr")
        .about("Updates the Mercury data collectors repo with a new PR")
        .arg(arg!(<PR_FILE> "Path to a pr.md file for a collector verifier project"))
        .arg_required_else_help(true)
    )
}
