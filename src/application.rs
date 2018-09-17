use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::{
    config::{Config, Mode, OrderBy},
    entry::FileMeta,
    name::NameGenerator,
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

        let padding = entries.len().to_string().len();
        let generator = NameGenerator::new(&base, padding);

        match mode {
            Mode::Test => apply(entries, generator.names(), list),
            Mode::Duplicate => apply(entries, generator.names(), copy),
            Mode::Move => apply(entries, generator.names(), rename),
        }
    }
}

fn apply(
    entries: impl IntoIterator<Item = FileMeta>,
    names: impl IntoIterator<Item = String>,
    f: fn(&Path, &Path) -> io::Result<()>,
) -> io::Result<()> {
    let pairs = entries.into_iter().zip(names);
    for (left, right) in pairs {
        let right = build_new_path(&left.path, &right);
        f(&left.path, &right)?
    }
    Ok(())
}

fn list(left: &Path, right: &Path) -> io::Result<()> {
    println!("{}\n  -> {}", left.display(), right.display());
    Ok(())
}

fn copy(left: &Path, right: &Path) -> io::Result<()> {
    fs::copy(left, right)?;
    Ok(())
}

fn rename(left: &Path, right: &Path) -> io::Result<()> {
    fs::rename(left, right)
}

fn build_new_path(path: impl AsRef<Path>, name: &str) -> PathBuf {
    let path = path.as_ref();
    let extension = path.extension();

    let mut path = path.to_owned();
    path.set_file_name(name);

    if let Some(extension) = extension {
        path.set_extension(extension);
    }

    path
}
