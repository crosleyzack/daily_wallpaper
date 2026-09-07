use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[cfg(feature = "imgmagick")]
mod create;
mod set;
mod sync;

#[derive(Parser)]
#[command(name = "wallpaper", about = "utility for creating daily wallpapers")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// create daily wallpaper
    #[cfg(feature = "imgmagick")]
    #[cfg(feature = "imgmagick")]
    Create {
        /// directory of base wallpapers
        #[arg(
            long,
            default_value = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/wallpapers"),
        )]
        wallpapers_dir: PathBuf,

        /// quotes file to pull from
        #[arg(
            long,
            default_value = concat!(env!("CARGO_MANIFEST_DIR"), "/data/quotes.json"),
        )]
        quotes_file: PathBuf,

        /// where to write resulting wallpaper file
        #[arg(
            long = "name",
            default_value = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/wallpaper.png"),
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
    Set {
        /// file to set as wallpaper
        #[arg(
            long,
            default_value = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/wallpaper.png"),
        )]
        wallpaper: PathBuf,

        /// type of system we are on
        #[arg(long, default_value = "linux-gnome")]
        os: set::OS,

        /// report what would change without operating
        #[arg(long)]
        dry_run: bool,
    },
    /// fetch wallpaper from remote
    Sync {
        /// wallpaper file to write
        #[arg(
            long,
            default_value = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/wallpaper.png"),
        )]
        target: PathBuf,

        /// location to pull wallpapers from
        #[arg(
            long,
            default_value = "https://raw.githubusercontent.com/crosleyzack/daily_wallpaper/main/assets/wallpaper.png"
        )]
        url: String,

        /// report what would change without operating
        #[arg(long)]
        dry_run: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        #[cfg(feature = "imgmagick")]
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
        Commands::Set {
            wallpaper,
            os,
            dry_run,
        } => {
            let args = set::Args {
                wallpaper,
                os,
                dry_run,
            };
            match set::set(&args) {
                Ok(()) => println!("set wallpaper successful"),
                Err(e) => println!("failed to set wallpaper: {e}"),
            }
        }
        Commands::Sync {
            target,
            url,
            dry_run,
        } => {
            let args = sync::Args {
                target,
                url,
                dry_run,
            };
            match sync::sync(&args) {
                Ok(()) => println!("sync wallpaper successful"),
                Err(e) => println!("failed to sync wallpaper: {e}"),
            }
        }
    }
}
