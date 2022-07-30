use clap::{App, Arg, ArgMatches, Command};

pub fn setup_argument_parser() -> ArgMatches {
    App::new("radium")
        .version("1.0.0")
        .author("avolgha")
        .about("simple package manager")
        .subcommand(
            Command::new("add-mirror")
                .about("add a mirror to install packages from")
                .arg(Arg::new("url").required(true)),
        )
        .subcommand(
            Command::new("remove-mirror")
                .about("remove a mirror from the mirror list")
                .arg(Arg::new("index").required(true)),
        )
        .subcommand(Command::new("list-mirrors").about("list all mirrors"))
        .subcommand(
            Command::new("install")
                .about("install a package from any mirror")
                .arg(Arg::new("package").required(true)),
        ).get_matches()
}
