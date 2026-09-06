use std::io::{Error, ErrorKind};
use std::path::PathBuf;

#[derive(clap::ValueEnum, Clone)]
pub enum OS {
    LinuxGnome,
    Mac,
    Windows,
}

pub struct Args {
    pub wallpaper: PathBuf,
    pub os: OS,
    pub dry_run: bool,
}

/// set wallpaper to file
pub fn set(args: &Args) -> Result<(), Error> {
    validate_args(args)?;
    if args.dry_run {
        return Ok(());
    }
    match args.os {
        OS::LinuxGnome => {
            let uri = format!("file://{}", args.wallpaper.display());
            let uri = uri.as_str();
            for (name, value) in [
                ("picture-uri", uri),
                ("picture-uri-dark", uri),
                ("picture-options", "spanned"),
            ] {
                match std::process::Command::new("gsettings")
                    .arg("set")
                    .arg("org.gnome.desktop.background")
                    .arg(name)
                    .arg(value)
                    .status()
                {
                    Ok(_) => {}
                    Err(e) => print!("unable to set {name} to {value}: {e}"),
                }
            }
        }
        OS::Windows => {
            panic!("windows not implemented")
        }
        OS::Mac => {
            panic!("macos not implemented")
        }
    }
    Ok(())
}

fn validate_args(args: &Args) -> Result<(), Error> {
    if !args.wallpaper.exists() || !args.wallpaper.is_file() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!(
                "wallpaper file does not exist: {}",
                args.wallpaper.to_str().unwrap_or("")
            ),
        ));
    }
    Ok(())
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
            let path = std::env::temp_dir().join(format!(
                "wallpaper_set_test_{}_{}",
                std::process::id(),
                seq
            ));
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

        fn wallpaper(&self) -> PathBuf {
            self.write("wallpaper.png", "x")
        }
    }

    impl Drop for TestDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn sample_args(dir: &TestDir) -> Args {
        Args {
            wallpaper: dir.wallpaper(),
            os: OS::LinuxGnome,
            dry_run: false,
        }
    }

    #[test]
    fn set_ok() {
        let dir = TestDir::new();
        let args = sample_args(&dir);
        assert!(set(&args).is_ok());
    }

    #[test]
    fn set_missing_wallpaper() {
        let dir = TestDir::new();
        let mut args = sample_args(&dir);
        args.wallpaper = dir.path().join("does_not_exist.png");
        let err = set(&args).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::NotFound);
        assert!(err.to_string().contains("wallpaper file does not exist"));
    }

    #[test]
    fn set_wallpaper_is_dir() {
        let dir = TestDir::new();
        let mut args = sample_args(&dir);
        args.wallpaper = dir.path().to_path_buf();
        let err = set(&args).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::NotFound);
    }

    #[test]
    fn validate_args_ok() {
        let dir = TestDir::new();
        assert!(validate_args(&sample_args(&dir)).is_ok());
    }

    #[test]
    fn validate_args_missing_wallpaper() {
        let dir = TestDir::new();
        let mut args = sample_args(&dir);
        args.wallpaper = dir.path().join("nope.png");
        assert!(validate_args(&args).is_err());
    }
}
