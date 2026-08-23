use img_mgk::generate_wp;
use rand::seq::{IndexedRandom, IteratorRandom};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::{File, read_dir};
use std::io::{BufReader, Error, ErrorKind};
use std::path::{Path, PathBuf};

mod img_mgk;

/// default file locations relative to cargo.toml
const DEFAULT_QUOTE_FILE: &str = "data/quotes.json";
const DEFAULT_WALLPAPER_FILE: &str = "assets/wallpaper.png";
const DEFAULT_WALLPAPERS_DIR: &str = "assets/wallpapers";

/// Settings for creating the wallpaper
pub struct Config {
    wallpapers_dir: PathBuf,
    out_file: PathBuf,
    quotes_file: PathBuf,
    author: Option<String>,
    wallpaper: Option<String>,
}

impl Config {
    pub fn new() -> Config {
        let base = Path::new(env!("CARGO_MANIFEST_DIR"));
        Config {
            wallpapers_dir: base.join(DEFAULT_WALLPAPERS_DIR).to_path_buf(),
            out_file: base.join(DEFAULT_WALLPAPER_FILE).to_path_buf(),
            quotes_file: base.join(DEFAULT_QUOTE_FILE).to_path_buf(),
            author: None,
            wallpaper: None,
        }
    }

    pub fn out_file(mut self, file: PathBuf) -> Self {
        self.out_file = file;
        self
    }

    pub fn wallpapers_dir(mut self, dir: PathBuf) -> Self {
        self.wallpapers_dir = dir;
        self
    }

    pub fn quotes_file(mut self, file: PathBuf) -> Self {
        self.quotes_file = file;
        self
    }

    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    pub fn wallpaper(mut self, wallpaper: impl Into<String>) -> Self {
        self.wallpaper = Some(wallpaper.into());
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub quote: String,
    pub author: String,
    pub file_regex: Option<String>,
    pub words_per_line: Option<i32>,
    pub font_size: Option<f64>,
    pub font: Option<String>,
    pub font_color: Option<String>,
    pub gravity: Option<String>,
    pub annotate: Option<String>,
}

/// generates a new wallpaper file
pub fn create(conf: Config) -> Result<(), Error> {
    validate_conf(&conf)?;
    let quote = random_quote(
        conf.quotes_file.as_ref(),
        QuoteFilter {
            author: conf.author,
            wallpaper: conf.wallpaper,
        },
    )?;
    let wallpaper: PathBuf = get_wallpaper(conf.wallpapers_dir.as_ref(), &quote)?;
    generate_wp(quote, wallpaper, conf.out_file)
}

fn validate_conf(conf: &Config) -> Result<(), Error> {
    if !conf.wallpapers_dir.exists() || !conf.wallpapers_dir.is_dir() {
        return Err(Error::new(
            ErrorKind::NotADirectory,
            format!(
                "wallpapers dir does not exist: {}",
                conf.wallpapers_dir.to_str().unwrap_or("")
            ),
        ));
    }
    if !conf.quotes_file.exists() || !conf.quotes_file.is_file() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!(
                "quotes file does not exist: {}",
                conf.quotes_file.to_str().unwrap_or("")
            ),
        ));
    }
    Ok(())
}

/// get_wallpaper get the path to a random, matching wallpaper for the quote
fn get_wallpaper(dir: &Path, quote: &Quote) -> Result<PathBuf, Error> {
    // get file from wallpaper dir by matching quote regex
    let dir: Vec<PathBuf> = read_dir(dir)?
        .flatten()
        // only files are valid wallpaper candidates
        .filter(|entry| entry.file_type().is_ok_and(|t| t.is_file()))
        .map(|entry| entry.path())
        .collect();
    let wallpapers: Vec<PathBuf> = match &quote.file_regex {
        // filter quotes by author if set
        Some(r) => {
            let regex = Regex::new(r).map_err(Error::other)?;
            dir.into_iter()
                .filter(|f| regex.is_match(&f.to_string_lossy()))
                .collect()
        }
        None => dir,
    };
    let wallpaper = wallpapers
        .choose(&mut rand::rng())
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "no quotes matched"))?;
    Ok(wallpaper.clone())
}

struct QuoteFilter {
    author: Option<String>,
    wallpaper: Option<String>,
}

/// get a random quote from the quotes file
fn random_quote(file: &Path, filter: QuoteFilter) -> Result<Quote, Error> {
    let all_quotes = read_quotes_file(file)?;
    let quote = all_quotes
        .into_iter()
        // apply author and wallpaper filters if supplied
        .filter(|q| filter.author.as_deref().is_none_or(|a| q.author == a))
        .filter(|q| {
            filter.wallpaper.as_deref().is_none_or(|w| {
                q.file_regex
                    .as_deref()
                    .is_none_or(|r| Regex::new(r).is_ok_and(|re| re.is_match(w)))
            })
        })
        .choose(&mut rand::rng())
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "no matching quotes found"))?;
    Ok(quote)
}

/// Reads and deserializes the quotes file
fn read_quotes_file(path: &Path) -> Result<Vec<Quote>, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let quotes: Vec<Quote> = serde_json::from_reader(reader)?;
    Ok(quotes)
}
