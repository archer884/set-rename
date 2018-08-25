#[macro_use]
extern crate clap;

mod application;
mod config;
mod entry;
mod name;

use std::io;

fn main() -> io::Result<()> {
    use application::Application;
    use config::Config;

    Application::new(Config::from_args())?.run()
}
