use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, num_args = 1..)]
    pub files: Vec<String>,

    #[arg(short, long, default_value_t = String::from(" "))]
    pub separator: String,

    #[arg(short, long, default_value_t = 1)]
    pub count: i32
}