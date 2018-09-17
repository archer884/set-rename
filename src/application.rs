use std::fs;
use std::io;
use std::path::PathBuf;

use crate::{
    config::{Config, OrderBy},
    entry::FileMeta,
};

pub struct Application {
    config: Config,
    working_directory: PathBuf,
}

impl Application {
    pub fn new(config: Config) -> io::Result<Self> {
        use std::env;

        Ok(Self {
            config,
            working_directory: env::current_dir()?,
        })
    }

    pub fn run(self) -> io::Result<()> {
        let Config { sort, mode, base } = self.config;
        let mut entries: Vec<_> = fs::read_dir(&self.working_directory)?
            .filter_map(|entry| FileMeta::try_from_entry(entry).ok())
            .collect();

        match sort {
            OrderBy::Created => entries.sort_by_key(|x| x.created),
            OrderBy::Modified => entries.sort_by_key(|x| x.modified),
            OrderBy::Size => entries.sort_by_key(|x| x.length),
        };

        unimplemented!()
    }
}
