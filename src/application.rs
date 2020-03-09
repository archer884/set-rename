use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::{
    entry::FileMeta,
    name::NameGenerator,
    options::{Opt, SortMode},
};

pub struct Application {
    options: Opt,
    working_directory: PathBuf,
}

impl Application {
    pub fn new(options: Opt) -> io::Result<Self> {
        use std::env;

        Ok(Self {
            options,
            working_directory: env::current_dir()?,
        })
    }

    pub fn run(self) -> io::Result<()> {
        let Opt {
            name,
            sort,
            force,
            pattern,
        } = self.options;

        let mut entries: Vec<_> = fs::read_dir(&self.working_directory)?
            .filter_map(|entry| FileMeta::try_from_entry(entry).ok())
            .filter(|entry| {
                pattern
                    .as_ref()
                    .map(|pattern| entry.is_match(pattern))
                    .unwrap_or(true)
            })
            .collect();

        match sort.unwrap_or(SortMode::Created) {
            SortMode::Created => entries.sort_by_key(|x| x.created),
            SortMode::Modified => entries.sort_by_key(|x| x.modified),
            SortMode::Size => entries.sort_by_key(|x| x.length),
        };

        let padding = entries.len().to_string().len();
        let generator = NameGenerator::new(&name, padding);

        if force {
            apply(entries, generator.names(), rename)
        } else {
            apply(entries, generator.names(), list)
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
