mod cli;
mod app;

use cli::commands::build_cli;
use app::build_cache;

fn main() {
    let  matches = build_cli().get_matches();

    match matches.subcommand(){
        Some(("build", _sub_matches)) => {

            build_cache::run(); // TODO: Implement build command
        }
        Some(("clean", _sub_matches)) => {
            println!("Cleaning cache");//TODO: Implement clean command
        }
        _ => {
            println!("use --help to see available commands");
        }
    }
}
