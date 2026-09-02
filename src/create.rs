use img_mgk::generate_wp;
use rand::seq::{IndexedRandom, IteratorRandom};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::{File, read_dir};
use std::io::{BufReader, Error, ErrorKind};
use std::path::{Path, PathBuf};

mod img_mgk;

pub struct Args {
    pub wallpapers_dir: PathBuf,
    pub quotes_file: PathBuf,
    pub out_file: PathBuf,
    pub author: Option<String>,
    pub wallpaper: Option<String>,
    pub dry_run: bool,
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
pub fn create(args: Args) -> Result<(), Error> {
    validate_args(&args)?;
    let quote = random_quote(
        args.quotes_file.as_ref(),
        &QuoteFilter {
            author: args.author,
            wallpaper: args.wallpaper,
        },
    )?;
    let wallpaper: PathBuf = get_wallpaper(args.wallpapers_dir.as_ref(), &quote)?;
    if args.dry_run {
        println!(
            "dry_run: would generate {} with quote {}",
            wallpaper.display(),
            quote.quote
        );
        return Ok(());
    }
    generate_wp(quote, &wallpaper, &args.out_file)
}

fn validate_args(args: &Args) -> Result<(), Error> {
    if !args.wallpapers_dir.exists() || !args.wallpapers_dir.is_dir() {
        return Err(Error::new(
            ErrorKind::NotADirectory,
            format!(
                "wallpapers dir does not exist: {}",
                args.wallpapers_dir.to_str().unwrap_or("")
            ),
        ));
    }
    if !args.quotes_file.exists() || !args.quotes_file.is_file() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!(
                "quotes file does not exist: {}",
                args.quotes_file.to_str().unwrap_or("")
            ),
        ));
    }
    Ok(())
}

/// get the path to a random, matching wallpaper for the quote
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
fn random_quote(file: &Path, filter: &QuoteFilter) -> Result<Quote, Error> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicUsize, Ordering};

    static DIR_SEQ: AtomicUsize = AtomicUsize::new(0);

    struct TestDir {
        path: PathBuf,
    }

    impl TestDir {
        fn new() -> Self {
            let seq = DIR_SEQ.fetch_add(1, Ordering::Relaxed);
            let path =
                std::env::temp_dir().join(format!("wallpaper_test_{}_{}", std::process::id(), seq));
            fs::create_dir_all(&path).unwrap();
            TestDir { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }

        fn write(&self, name: &str, contents: &str) -> PathBuf {
            let file = self.path.join(name);
            fs::write(&file, contents).unwrap();
            file
        }

        fn subdir(&self, name: &str) -> PathBuf {
            let dir = self.path.join(name);
            fs::create_dir_all(&dir).unwrap();
            dir
        }
    }

    impl Drop for TestDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn sample_args(dir: &TestDir) -> Args {
        Args {
            wallpapers_dir: dir.subdir("wallpapers"),
            quotes_file: dir.write("quotes.json", sample_quotes_json()),
            out_file: dir.path.join("wallpaper.png"),
            author: None,
            wallpaper: None,
            dry_run: false,
        }
    }

    fn sample_quotes_json() -> &'static str {
        r#"[
            {
                "author": "Marcus Aurelius",
                "fileRegex": "landscape",
                "quote": "Each day provides its own gifts"
            },
            {
                "author": "Seneca",
                "fileRegex": "landscape",
                "wordsPerLine": 8,
                "quote": "Life is long, if you know how to use it"
            },
            {
                "author": "Gautama Buddha",
                "fileRegex": "buddha",
                "fontSize": 110,
                "quote": "Peace comes from within. Do not seek it without"
            }
        ]"#
    }

    fn quote_with(author: &str, file_regex: Option<&str>, quote: &str) -> Quote {
        Quote {
            quote: quote.to_string(),
            author: author.to_string(),
            file_regex: file_regex.map(ToString::to_string),
            words_per_line: None,
            font_size: None,
            font: None,
            font_color: None,
            gravity: None,
            annotate: None,
        }
    }

    #[test]
    fn validate_args_ok() {
        let dir = TestDir::new();
        let args = sample_args(&dir);
        assert!(validate_args(&args).is_ok());
    }

    #[test]
    fn validate_args_missing_wallpapers_dir() {
        let dir = TestDir::new();
        let mut args = sample_args(&dir);
        args.wallpapers_dir = dir.path().join("does_not_exist");
        let err = validate_args(&args).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::NotADirectory);
        assert!(err.to_string().contains("wallpapers dir does not exist"));
    }

    #[test]
    fn validate_args_wallpapers_dir_is_file() {
        let dir = TestDir::new();
        let mut args = sample_args(&dir);
        args.wallpapers_dir = dir.write("not_a_dir.png", "x");
        let err = validate_args(&args).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::NotADirectory);
    }

    #[test]
    fn validate_args_missing_quotes_file() {
        let dir = TestDir::new();
        let mut args = sample_args(&dir);
        args.quotes_file = dir.path().join("does_not_exist.json");
        let err = validate_args(&args).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::NotFound);
        assert!(err.to_string().contains("quotes file does not exist"));
    }

    #[test]
    fn validate_args_quotes_file_is_dir() {
        let dir = TestDir::new();
        let mut args = sample_args(&dir);
        args.quotes_file = dir.subdir("quotes_dir");
        let err = validate_args(&args).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::NotFound);
    }

    #[test]
    fn read_quotes_file_valid() {
        let dir = TestDir::new();
        let file = dir.write("quotes.json", sample_quotes_json());
        let quotes = read_quotes_file(&file).unwrap();
        assert_eq!(quotes.len(), 3);
        assert_eq!(quotes[0].author, "Marcus Aurelius");
        assert_eq!(quotes[1].words_per_line, Some(8));
        assert_eq!(quotes[2].file_regex.as_deref(), Some("buddha"));
        assert_eq!(quotes[2].font_size, Some(110.0));
    }

    #[test]
    fn read_quotes_file_empty_array() {
        let dir = TestDir::new();
        let file = dir.write("quotes.json", "[]");
        assert!(read_quotes_file(&file).unwrap().is_empty());
    }

    #[test]
    fn read_quotes_file_invalid_json() {
        let dir = TestDir::new();
        let file = dir.write("quotes.json", "{ not valid json");
        assert!(read_quotes_file(&file).is_err());
    }

    #[test]
    fn read_quotes_file_missing() {
        let dir = TestDir::new();
        let err = read_quotes_file(&dir.path().join("nope.json")).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::NotFound);
    }

    #[test]
    fn random_quote_no_filter() {
        let dir = TestDir::new();
        let file = dir.write("quotes.json", sample_quotes_json());
        let quote = random_quote(
            &file,
            &QuoteFilter {
                author: None,
                wallpaper: None,
            },
        )
        .unwrap();
        let authors = ["Marcus Aurelius", "Seneca", "Gautama Buddha"];
        assert!(authors.contains(&quote.author.as_str()));
    }

    #[test]
    fn random_quote_author_filter() {
        let dir = TestDir::new();
        let file = dir.write("quotes.json", sample_quotes_json());
        let filter = QuoteFilter {
            author: Some("Seneca".to_string()),
            wallpaper: None,
        };
        let quote = random_quote(&file, &filter).unwrap();
        assert_eq!(quote.author, "Seneca");
    }

    #[test]
    fn random_quote_author_no_match() {
        let dir = TestDir::new();
        let file = dir.write("quotes.json", sample_quotes_json());
        let filter = QuoteFilter {
            author: Some("Nobody".to_string()),
            wallpaper: None,
        };
        let err = random_quote(&file, &filter).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::NotFound);
        assert_eq!(err.to_string(), "no matching quotes found");
    }

    #[test]
    fn random_quote_wallpaper_filter() {
        let dir = TestDir::new();
        let file = dir.write("quotes.json", sample_quotes_json());
        let filter = QuoteFilter {
            author: None,
            wallpaper: Some("buddha.png".to_string()),
        };
        let quote = random_quote(&file, &filter).unwrap();
        let regex = Regex::new(quote.file_regex.as_deref().unwrap()).unwrap();
        assert!(regex.is_match("buddha.png"));
    }

    #[test]
    fn random_quote_missing_file() {
        let dir = TestDir::new();
        let filter = QuoteFilter {
            author: None,
            wallpaper: None,
        };
        let err = random_quote(&dir.path().join("nope.json"), &filter).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::NotFound);
    }

    #[test]
    fn get_wallpaper_no_regex() {
        let dir = TestDir::new();
        let wallpapers = dir.subdir("wallpapers");
        for name in ["landscape1.png", "buddha.png", "cross.png"] {
            fs::write(wallpapers.join(name), "x").unwrap();
        }
        let quote = quote_with("Author", None, "A quote");
        let wallpaper = get_wallpaper(&wallpapers, &quote).unwrap();
        assert!(wallpaper.starts_with(&wallpapers));
    }

    #[test]
    fn get_wallpaper_regex_match() {
        let dir = TestDir::new();
        let wallpapers = dir.subdir("wallpapers");
        for name in ["landscape1.png", "buddha.png", "cross.png"] {
            fs::write(wallpapers.join(name), "x").unwrap();
        }
        let quote = quote_with("Author", Some("buddha"), "A quote");
        let wallpaper = get_wallpaper(&wallpapers, &quote).unwrap();
        assert!(wallpaper.to_string_lossy().contains("buddha"));
    }

    #[test]
    fn get_wallpaper_no_match() {
        let dir = TestDir::new();
        let wallpapers = dir.subdir("wallpapers");
        for name in ["landscape1.png", "buddha.png"] {
            fs::write(wallpapers.join(name), "x").unwrap();
        }
        let quote = quote_with("Author", Some("nomatch"), "A quote");
        let err = get_wallpaper(&wallpapers, &quote).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::NotFound);
        assert_eq!(err.to_string(), "no quotes matched");
    }

    #[test]
    fn get_wallpaper_empty_dir() {
        let dir = TestDir::new();
        let wallpapers = dir.subdir("wallpapers");
        let quote = quote_with("Author", None, "A quote");
        let err = get_wallpaper(&wallpapers, &quote).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::NotFound);
        assert_eq!(err.to_string(), "no quotes matched");
    }

    #[test]
    fn get_wallpaper_missing_dir() {
        let dir = TestDir::new();
        let quote = quote_with("Author", None, "A quote");
        let err = get_wallpaper(&dir.path().join("nope"), &quote).unwrap_err();
        assert!(err.kind() == ErrorKind::NotFound || err.kind() == ErrorKind::PermissionDenied);
    }

    #[test]
    fn create_dry_run_writes_nothing() {
        let dir = TestDir::new();
        let wallpapers = dir.subdir("wallpapers");
        fs::write(wallpapers.join("landscape1.png"), "x").unwrap();
        let mut args = sample_args(&dir);
        args.wallpapers_dir = wallpapers;
        args.author = Some("Marcus Aurelius".to_string());
        args.wallpaper = Some("landscape1.png".to_string());
        args.dry_run = true;
        let out = args.out_file.clone();
        create(args).unwrap();
        assert!(!out.exists());
    }
}
