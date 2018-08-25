use config::Config;
use std::io;
use std::path::PathBuf;

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
        unimplemented!()
    }
}
