#[macro_use]
extern crate clap;

mod application;
mod config;
mod entry;
mod name;

use std::io;

fn main() -> io::Result<()> {
    use crate::{application::Application, config::Config};

    Application::new(Config::from_args())?.run()
}
