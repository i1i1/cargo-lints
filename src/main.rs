use eyre::{eyre, Result, WrapErr};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "cargo lints")]
/// Utility for working with specific lints of clippy
enum Args {
    /// Formats lints.toml file
    Fmt,
    /// Runs clippy with lints enabled from lints.toml file
    #[structopt(external_subcommand)]
    Clippy(Vec<String>),
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
struct Lints {
    #[serde(skip)]
    file: Option<PathBuf>,

    #[serde(default)]
    deny: Vec<String>,
    #[serde(default)]
    allow: Vec<String>,
    #[serde(default)]
    warn: Vec<String>,
}

impl Lints {
    fn find_config_file() -> Result<Option<PathBuf>> {
        let mut path =
            env::current_dir().wrap_err("Failed to get current directory")?;
        loop {
            let lints = path.join("lints.toml");
            if !lints.exists() {
                match path.parent() {
                    Some(parent) => path = parent.to_path_buf(),
                    None => return Ok(None),
                }
                continue;
            }
            if !lints.is_file() {
                return Err(eyre!("{:?} supposed to be file", lints));
            }
            return Ok(Some(lints));
        }
    }

    pub fn from_config() -> Result<Self> {
        Self::find_config_file()?
            .map_or_else(|| Ok(Lints::default()), Self::from_config_with_path)
    }

    pub fn from_config_with_path(file: PathBuf) -> Result<Self> {
        let mut lints: Self = fs::read_to_string(&file)
            .wrap_err("Failed to read config")
            .map(|s| toml::from_str(&s))?
            .wrap_err("Failed to parse config")?;
        lints.file = Some(file);
        Ok(lints)
    }

    pub fn file(&self) -> PathBuf {
        self.file.as_ref().expect("Always inited").clone()
    }

    pub fn fmt(&mut self) -> Result<()> {
        self.allow.sort();
        self.deny.sort();
        self.warn.sort();
        toml::to_string_pretty(&self)
            .wrap_err("Failed format toml to string")
            .map(|content| fs::write(self.file(), content))?
            .wrap_err("Failed to write lints to file")
    }

    fn deny_flags(&self) -> Vec<String> {
        self.deny
            .iter()
            .flat_map(|lint| vec!["-D".to_owned(), lint.clone()].into_iter())
            .collect()
    }

    fn warn_flags(&self) -> Vec<String> {
        self.warn
            .iter()
            .flat_map(|lint| vec!["-W".to_owned(), lint.clone()].into_iter())
            .collect()
    }

    fn allow_flags(&self) -> Vec<String> {
        self.allow
            .iter()
            .flat_map(|lint| vec!["-A".to_owned(), lint.clone()].into_iter())
            .collect()
    }

    pub fn clippy(&self, args: &[String]) -> Result<()> {
        Command::new("cargo")
            .arg("clippy")
            .args(args)
            .arg("--")
            .args(self.deny_flags())
            .args(self.warn_flags())
            .args(self.allow_flags())
            .spawn()
            .wrap_err("Failed to start clippy")?
            .wait()
            .wrap_err("Failed to wait till finish of clippy")
            .map(drop)
    }
}

fn main() -> Result<()> {
    let cmd = Args::from_args();
    let mut lints = Lints::from_config()?;
    match cmd {
        Args::Fmt => lints.fmt(),
        Args::Clippy(args) => lints.clippy(&args[1..]),
    }
}
