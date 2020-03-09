use regex::Regex;
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
pub struct Opt {
    pub name: String,
    #[structopt(subcommand)]
    pub sort: Option<SortMode>,
    #[structopt(short = "p", long = "pattern", parse(try_from_str = Regex::new))]
    pub pattern: Option<Regex>,
    #[structopt(short = "f", long = "force")]
    pub force: bool,
}

#[derive(Clone, Debug, StructOpt)]
pub enum SortMode {
    Created,
    Modified,
    Size,
}

pub fn from_args() -> Opt {
    StructOpt::from_args()
}
