mod application;
mod entry;
mod name;
mod options;

use std::io;

fn main() -> io::Result<()> {
    application::Application::new(options::from_args())?.run()
}
