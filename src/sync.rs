use std::env::temp_dir;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

pub struct Args {
    pub target: PathBuf,
    pub repository: String,
    pub branch: String,
    pub file: String,
    pub dry_run: bool,
}

/// pull wallpaper from remote
pub fn sync(args: Args) -> Result<(), Error> {
    validate_args(&args)?;
    // get remote wallpaper file
    let raw_url = format!(
        "https://raw.githubusercontent.com/{}/{}/{}",
        args.repository, args.branch, args.file
    );
    let data = reqwest::blocking::get(&raw_url)
        .map_err(Error::other)?
        .error_for_status()
        .map_err(Error::other)?
        .text()
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
            format!("failed to create temp file",),
        ));
    }
    let metadata = fs::metadata(&temp_file)?;
    if metadata.len() == 0 {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("failed to download remote wallpaper",),
        ));
    }
    // move file from temp to target
    fs::rename(&temp_file, args.target)?;
    Ok(())
}

fn validate_args(args: &Args) -> Result<(), Error> {
    if args.repository.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("repository is empty",),
        ));
    }
    if args.branch.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("branch is empty",),
        ));
    }
    if args.file.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("file is empty",),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn sample_args() -> Args {
        Args {
            target: PathBuf::from("/tmp/wallpaper.png"),
            repository: "crosleyzack/daily_wallpaper".to_string(),
            branch: "main".to_string(),
            file: "assets/quotes.json".to_string(),
            dry_run: false,
        }
    }

    #[test]
    fn validate_args_ok() {
        assert!(validate_args(&sample_args()).is_ok());
    }

    #[test]
    fn validate_args_empty_repository() {
        let mut args = sample_args();
        args.repository.clear();
        let err = validate_args(&args).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert_eq!(err.to_string(), "repository is empty");
    }

    #[test]
    fn validate_args_empty_branch() {
        let mut args = sample_args();
        args.branch.clear();
        let err = validate_args(&args).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert_eq!(err.to_string(), "branch is empty");
    }

    #[test]
    fn validate_args_empty_file() {
        let mut args = sample_args();
        args.file.clear();
        let err = validate_args(&args).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert_eq!(err.to_string(), "file is empty");
    }
}
