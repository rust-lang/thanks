use std::io::Write;
use tempfile::NamedTempFile;

pub struct Config {
    cfg: git2::Config,
    file: NamedTempFile,
}

impl std::ops::Deref for Config {
    type Target = git2::Config;
    fn deref(&self) -> &Self::Target {
        &self.cfg
    }
}

impl Config {
    pub fn parse(cfg: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let mut file = NamedTempFile::new()?;
        file.write_all(cfg.as_bytes())?;
        let cfg = git2::Config::open(file.path())?;
        Ok(Config { cfg, file })
    }
}
