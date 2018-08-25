use std::fs::DirEntry;
use std::io;
use std::path::PathBuf;
use std::time::SystemTime;

pub struct FileMeta {
    path: PathBuf,
    length: u64,
    created: SystemTime,
    modified: SystemTime,
}

impl FileMeta {
    pub fn from_entry(entry: DirEntry) -> io::Result<Self> {
        let meta = entry.metadata()?;

        if !meta.file_type().is_file() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "FileMeta source must be a file",
            ));
        }

        Ok(FileMeta {
            path: entry.path(),
            length: meta.len(),
            created: meta.created()?,
            modified: meta.modified()?,
        })
    }

    pub fn try_from_entry(entry: io::Result<DirEntry>) -> io::Result<Self> {
        Self::from_entry(entry?)
    }
}
