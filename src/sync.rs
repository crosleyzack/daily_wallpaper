use std::env::temp_dir;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use url::Url;

pub struct Args {
    pub target: PathBuf,
    pub url: String,
    pub dry_run: bool,
}

/// pull wallpaper from remote
pub fn sync(args: &Args) -> Result<(), Error> {
    let url = parse_args(args)?;
    // get remote wallpaper file
    let data = reqwest::blocking::get(url)
        .map_err(Error::other)?
        .error_for_status()
        .map_err(Error::other)?
        .bytes()
        .map_err(Error::other)?;
    // write to temp directory
    let temp_file = temp_dir().join(
        args.target
            .file_name()
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, "target has no file name"))?,
    );
    fs::write(&temp_file, data)?;
    if !temp_file.exists() || !temp_file.is_file() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("failed to create temp file: {}", temp_file.display()),
        ));
    }
    let metadata = fs::metadata(&temp_file)?;
    if metadata.len() == 0 {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("failed to download remote wallpaper: {}", args.url),
        ));
    }
    // move file from temp to target
    if !args.dry_run {
        fs::copy(&temp_file, &args.target)?;
        fs::remove_file(&temp_file)?;
    }
    Ok(())
}

fn parse_args(args: &Args) -> Result<Url, Error> {
    if args.url.is_empty() {
        return Err(Error::new(ErrorKind::InvalidInput, "url is empty"));
    }
    match Url::parse(&args.url) {
        Ok(url) => Ok(url),
        Err(e) => Err(Error::new(
            ErrorKind::InvalidInput,
            format!("url is invalid: {e}"),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn sample_args() -> Args {
        Args {
            target: PathBuf::from("/tmp/wallpaper.png"),
            url: "https://example.com/wallpaper.png".to_string(),
            dry_run: false,
        }
    }

    #[test]
    fn parse_args_ok() {
        assert!(parse_args(&sample_args()).is_ok());
    }

    #[test]
    fn parse_args_empty_url() {
        let mut args = sample_args();
        args.url.clear();
        let err = parse_args(&args).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert_eq!(err.to_string(), "url is empty");
    }

    #[test]
    fn parse_args_invalid_url() {
        let mut args = sample_args();
        args.url = "not a url".to_string();
        let err = parse_args(&args).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert!(err.to_string().starts_with("url is invalid: "));
    }
}
