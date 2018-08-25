pub struct Config {
    sort: OrderBy,
    mode: Mode,
    base: String,
}

pub enum OrderBy {
    Size,
    Created,
    Modified,
}

pub enum Mode {
    Move,
    Duplicate,
    Test,
}

impl Config {
    pub fn from_args() -> Self {
        use clap::ArgGroup;

        let app = clap_app!(srn =>
            (version: crate_version!())
            (author: crate_authors!())
            (about: crate_description!())
            (@arg size: -s --size "Sort by file size")
            (@arg created: -c --created "Sort by file creation date")
            (@arg modified: -m --modified "Sort by file modified date")
            (@arg force: -f --force "Rename files")
            (@arg duplicate: -d --duplicate "Duplicate files")
            (@arg name: +required +takes_value)
        );

        let app = app
            .group(ArgGroup::with_name("sort").args(&["size", "created", "modified"]))
            .group(ArgGroup::with_name("mode").args(&["force", "duplicate"]));

        let matches = app.get_matches();

        let sort = if matches.is_present("size") {
            OrderBy::Size
        } else if matches.is_present("modified") {
            OrderBy::Modified
        } else {
            OrderBy::Created
        };

        let mode = if matches.is_present("force") {
            Mode::Move
        } else if matches.is_present("duplicate") {
            Mode::Duplicate
        } else {
            Mode::Test
        };

        Config {
            base: matches.value_of("name").expect("unreachable").into(),
            sort,
            mode,
        }
    }
}
