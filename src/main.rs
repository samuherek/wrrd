use clap::{Parser, Subcommand};
use rand::Rng;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    #[arg(long)]
    min: Option<u8>,
    /// Optional name to operate on
    #[arg(long)]
    max: Option<u8>,
    ///
    #[arg(short, long)]
    count: Option<usize>,
    // /// Turn debugging information on
    // #[arg(short, long, action = clap::ArgAction::Count)]
    // debug: u8,
}

const VOW: &str = "aeiou";
const CON: &str = "bcdfghjklmnpqrstvwxyz";

// <con><vow>
// <con><vow><con>
// <vow><con><vow>
fn gen_word(rng: &mut rand::rngs::ThreadRng, min: usize, max: usize) -> String {
    let start = rng.gen_range(0..=1);
    let len = rng.gen_range(min..=max);
    let mut buf = String::new();

    for i in 0..len {
        if i % 2 == start {
            let idx = rng.gen_range(0..VOW.len());
            let c = VOW.chars().nth(idx).expect("To get the char");
            buf.push(c);
        } else {
            let idx = rng.gen_range(0..CON.len());
            let c = CON.chars().nth(idx).expect("To get the char");
            buf.push(c);
        }
    }

    buf
}

fn main() {
    let cli = Cli::parse();
    let min = cli.min.unwrap_or(3);
    let max = cli.max.unwrap_or(6);
    let len = cli.count.unwrap_or(10);
    let mut rng = rand::thread_rng();

    for _ in 0..len {
        let word = gen_word(&mut rng, min.into(), max.into());
        println!("{}", word);
    }
}
