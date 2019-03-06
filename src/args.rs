
use clap::*;



pub fn build_cli() -> App<'static, 'static> {
    let check = Arg::with_name("check-extension")
        .help("Check file extension before load")
        .short("c")
        .long("check-extension");
    let dhash = Arg::with_name("dhash")
        .help("Compute dhash")
        .short("d")
        .long("--dhash");

    app_from_crate!()
        .arg(Arg::with_name("database-name")
             .help("Database name")
             .short("n")
             .long("name")
             .takes_value(true))
        .arg(Arg::with_name("database-path")
             .help("Path to *.sqlite")
             .short("p")
             .long("path")
             .takes_value(true))
        .subcommand(SubCommand::with_name("alias")
                    .alias("a")
                    .about("Define expression alias")
                    .arg(Arg::with_name("local")
                         .help("Database local alias")
                         .short("l")
                         .long("local")
                         .takes_value(false))
                    .arg(Arg::with_name("recursive")
                         .help("Recursive")
                         .short("r")
                         .long("recursive")
                         .takes_value(false))
                    .arg(Arg::with_name("name"))
                    .arg(Arg::with_name("expression")
                         .min_values(0)))
        .subcommand(SubCommand::with_name("completions")
                    .about("Generates completion scripts for your shell")
                    .arg(Arg::with_name("shell")
                         .required(true)
                         .possible_values(&["bash", "fish", "zsh"])
                         .help("The shell to generate the script for")))
        .subcommand(SubCommand::with_name("expand")
                    .about("Show alias expanded expression")
                    .arg(Arg::with_name("full")
                         .help("Full")
                         .short("f")
                         .long("full")
                         .takes_value(false))
                    .arg(Arg::with_name("expression")
                         .required(true)))
        .subcommand(SubCommand::with_name("get")
                    .about("Get image information")
                    .arg(Arg::with_name("path")
                         .required(true)))
        .subcommand(SubCommand::with_name("load")
                    .alias("l")
                    .about("Load directory or file")
                    .arg(Arg::with_name("update")
                         .help("Update exising files")
                         .short("u")
                         .long("update")
                         .takes_value(false))
                    .arg(Arg::with_name("tag-script")
                         .help("Tag generator script")
                         .short("t")
                         .long("--tag-script")
                         .takes_value(true))
                    .arg(Arg::with_name("path")
                         .required(true)
                         .min_values(1))
                    .arg(check.clone())
                    .arg(dhash.clone()))
        .subcommand(SubCommand::with_name("load-list")
                    .alias("l")
                    .about("Load from list file")
                    .arg(Arg::with_name("update")
                         .help("Update exising files")
                         .short("u")
                         .long("update")
                         .takes_value(false))
                    .arg(Arg::with_name("tag-script")
                         .help("Tag generator script")
                         .short("t")
                         .long("--tag-script")
                         .takes_value(true))
                    .arg(Arg::with_name("list-file")
                         .required(true)
                         .min_values(0))
                    .arg(check)
                    .arg(dhash))
        .subcommand(SubCommand::with_name("path")
                    .about("Show database path"))
        .subcommand(SubCommand::with_name("reset")
                    .about("Clear all data"))
        .subcommand(SubCommand::with_name("select")
                    .alias("s")
                    .about("Select SQL")
                    .arg(Arg::with_name("vacuum")
                         .help("Remove entries that do not exist")
                         .short("v")
                         .long("vacuum")
                         .takes_value(false))
                    .arg(Arg::with_name("where")
                         .required(true)
                         .min_values(1)))
        .subcommand(SubCommand::with_name("tag")
                    .alias("t")
                    .about("Manage tags")
                    .subcommand(SubCommand::with_name("add")
                                .alias("a")
                                .about("Add tags")
                                .arg(Arg::with_name("path")
                                     .required(true))
                                .arg(Arg::with_name("tag")
                                     .required(true)
                                     .min_values(1)))
                    .subcommand(SubCommand::with_name("clear")
                                .alias("c")
                                .about("Clear tags")
                                .arg(Arg::with_name("path")
                                     .required(true)))
                    .subcommand(SubCommand::with_name("remove")
                                .alias("r")
                                .about("Remove tags")
                                .arg(Arg::with_name("path")
                                     .required(true))
                                .arg(Arg::with_name("tag")
                                     .required(true)
                                     .min_values(1)))
                    .subcommand(SubCommand::with_name("set")
                                .alias("s")
                                .about("Set tags")
                                .arg(Arg::with_name("path")
                                     .required(true))
                                .arg(Arg::with_name("tag")
                                     .min_values(0))))
        .subcommand(SubCommand::with_name("unalias")
                    .alias("s")
                    .about("Unalias")
                    .arg(Arg::with_name("local")
                         .help("Database local alias")
                         .short("l")
                         .long("local")
                         .takes_value(false))
                    .arg(Arg::with_name("name")
                         .required(true)))
}
