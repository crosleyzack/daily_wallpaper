use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod create;

#[derive(Parser)]
#[command(name = "wallpaper", about = "utility for creating daily wallpapers")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// create daily wallpaper
    Create {
        /// directory of base wallpapers
        #[arg(
            long,
            default_value = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/wallpapers")
        )]
        wallpapers_dir: PathBuf,

        /// quotes file to pull from
        #[arg(
            long,
            default_value = concat!(env!("CARGO_MANIFEST_DIR"), "/data/quotes.json")
        )]
        quotes_file: PathBuf,

        /// where to write resulting wallpaper file
        #[arg(
            long = "name",
            default_value = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/wallpaper.png")
        )]
        out_file: PathBuf,

        /// specific author to pull quote from
        #[arg(long)]
        author: Option<String>,

        /// specific base wallpaper to generate with
        #[arg(long)]
        wallpaper: Option<String>,

        /// report what would change without writing files
        #[arg(long)]
        dry_run: bool,
    },
    /// set the gnome desktop wallpaper
    Set,
    /// fetch wallpaper from remote
    Sync,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create {
            wallpapers_dir,
            quotes_file,
            out_file,
            author,
            wallpaper,
            dry_run,
        } => {
            let args = create::Args {
                wallpapers_dir,
                quotes_file,
                out_file,
                author,
                wallpaper,
                dry_run,
            };
            match create::create(args) {
                Ok(()) => println!("create wallpaper finished successfully"),
                Err(e) => println!("failed to create wallpaper: {e}"),
            }
        }
        Commands::Set => println!("not implemented"),
        Commands::Sync => println!("not implemented"),
    }
}
