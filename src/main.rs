use clap::{ArgMatches, Command, arg};
use std::io::Error;
use std::path::Path;

mod create;

fn cli() -> Command {
    Command::new("wallpaper")
        .about("utility for creating daily wallpapers")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("create")
                .about("create daily wallpaper")
                // <foo> indicates a required positional arugment
                // [foo] indicates a optional positional arugment
                .arg(arg!(<ASSET_DIR> "The asset directory with base wallpapers"))
                .arg(arg!(<DATA_DIR> "The data directory with quotes.json"))
                .arg(arg!(-n --name [NAME] "The name to use for the new wallpaper"))
                .arg(arg!(--author [AUTHOR] "the quote author to use"))
                .arg(arg!(--wallpaper [WALLPAPER] "the wallpaper to use"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("set")
                .about("set the gnome desktop wallpaper")
                .arg(arg!(<WALLPAPER_FILE> "The wallpaper image file to use"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("sync")
                .about("fetch wallpaper from remote")
                .arg(arg!(<WALLPAPER_FILE> "The wallpaper image file to download to"))
                .arg_required_else_help(true),
        )
}

fn main() {
    let matches = cli().try_get_matches().unwrap_or_else(|error| error.exit());

    match matches.subcommand() {
        Some(("create", s)) => {
            match run_create(s) {
                Ok(()) => println!("create wallpaper finished successfully"),
                Err(e) => println!("failed to create wallpaper: {}", e),
            };
        }
        Some(("set", sub_matches)) => {
            // TODO
            let color = sub_matches
                .get_one::<String>("color")
                .map(|s| s.as_str())
                .expect("defaulted in clap");

            let mut base = sub_matches.get_one::<String>("base").map(|s| s.as_str());
            let mut head = sub_matches.get_one::<String>("head").map(|s| s.as_str());
            let mut path = sub_matches.get_one::<String>("path").map(|s| s.as_str());
            if path.is_none() {
                path = head;
                head = None;
                if path.is_none() {
                    path = base;
                    base = None;
                }
            }
            let base = base.unwrap_or("stage");
            let head = head.unwrap_or("worktree");
            let path = path.unwrap_or("");
            println!("Diffing {base}..{head} {path} (color={color})");
        }
        Some(("sync", sub_matches)) => {
            println!(
                "Pushing to {}",
                sub_matches.get_one::<String>("REMOTE").expect("required")
            );
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }

    // Continued program logic goes here...
}

fn run_create(sub_matches: &ArgMatches) -> Result<(), Error> {
    let asset_dir = sub_matches
        .get_one::<String>("ASSET_DIR")
        .expect("required in clap");
    let data_dir = sub_matches
        .get_one::<String>("DATA_DIR")
        .expect("required in clap");
    let conf = create::Config::new()
        .wallpapers_dir(Path::new(asset_dir).join("wallpapers"))
        .quotes_file(Path::new(data_dir).join("quotes.json"));

    let conf = match sub_matches.get_one::<String>("name") {
        Some(name) => conf.out_file(Path::new(name).to_path_buf()),
        None => conf,
    };
    let conf = match sub_matches.get_one::<String>("author") {
        Some(author) => conf.author(author),
        None => conf,
    };
    let conf = match sub_matches.get_one::<String>("wallpaper") {
        Some(wp) => conf.wallpaper(wp),
        None => conf,
    };

    create::create(conf)
}
