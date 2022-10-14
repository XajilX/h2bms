use clap::Parser;
use rand::

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Length of scatter
    len: u32,
    
    /// Start hand
    #[arg(short, long, value_name="HAND", value_parser=["left", "right", "l", "r"])]
    start: Option<String>
}

fn main() {
    let trans = [
        vec![ 9, 10, 11, 12],
        vec![10, 11],
        vec![ 8,  9, 14, 15],
        vec![ 8,  9, 10, 14, 15],
        vec![ 8,  9, 10, 11, 14, 15],
        vec![ 8,  9, 10, 11, 12, 14, 15],
        vec![ 8,  9, 10, 11, 12, 13, 15],
        vec![ 8,  9, 10, 11, 12],
        vec![ 3,  4,  5,  6,  7],
        vec![ 0,  2,  3,  4,  5,  6,  7],
        vec![ 0,  1,  3,  4,  5,  6,  7],
        vec![ 0,  1,  4,  5,  6,  7],
        vec![ 0,  1,  5,  6,  7],
        vec![ 0,  1,  6,  7],
        vec![ 4,  5],
        vec![ 3,  4,  5,  6],
    ];
    let args = Args::parse();
}
