use clap::{Command};

pub fn build_cli() -> Command {
    Command::new("rimcache")
        .about("Tools for managing cache and otimization of rimworld")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("build")
                .about("Builds the cache for the mods ")
        )
        .subcommand(
            Command::new("clean")
                .about("Cleans the cache for the mods")
        )        
}