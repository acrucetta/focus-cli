use clap::{arg, command, Command};

mod config;
mod focus;
mod server;

fn main() {
    // Load the config file
    let config = config::load_config();

    let matches = command!()
        .subcommand_required(true)
        .subcommand(
            Command::new("start")
                .about("Start timing a task")
                .arg(arg!([TIME])),
        )
        .subcommand(Command::new("kill").about("Kill the focus timer"))
        .get_matches();

    let subcommand = matches.subcommand();
    let (subcommand, sub_m) = if let Some(subc) = subcommand {
        subc
    } else {
        eprintln!("Missing subcommand.");
        return;
    };

    match subcommand {
        "start" => {
            let time = sub_m.get_one::<String>("TIME").unwrap();
            let time = time.parse::<u32>().unwrap();
            focus::start_timer(time);
        }
        "kill" => {
            focus::kill_timer();
        }
        otherwise => {
            eprintln!("Unknown subcommand: {}", otherwise);
        }
    }
}
