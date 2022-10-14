use clap::Parser;
use rand::{thread_rng, Rng};

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

    let mut rng = thread_rng();
    let mut pos = match args.start {
        None => rng.gen_range(0..=15),
        Some(x) => match x.as_str() {
            "left" | "l" => rng.gen_range(0..=7),
            "right" | "r" => rng.gen_range(8..=15),
            _ => panic!("Param Error")
        }
    };
    for _ in 0..args.len {
        print!("{},", pos%8+1);
        pos = trans[pos][rng.gen_range(0..trans[pos].len())]
    }
}
