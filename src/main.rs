use clap::Parser;
use rand::Rng;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Minimum allowed characters. Default 3
    #[arg(long)]
    min: Option<u8>,
    /// Maximum allowed characters. Default 6
    #[arg(long)]
    max: Option<u8>,
    /// How many words to generate. Default 10
    #[arg(short, long)]
    count: Option<usize>,
}

const VOW: &str = "aeiou";
const CON: &str = "bcdfghjklmnpqrstvwxyz";
// const PATTERNS: &[&str] = &[
//     "CCV", "CVC", "VCV", "VVC", "CVV", "VCC", "CVCV", "VCVC", "CVCC", "VCCV", "CVVC", "CCVV",
// ];

fn gen_word(rng: &mut rand::rngs::ThreadRng, config: &WordConfig) -> String {
    let first_vow = rng.gen_range(0..=1);
    let len = rng.gen_range(config.min..=config.max);
    let mut buf = String::new();

    while buf.len() < len {
        let c = if buf.len() % 2 == first_vow {
            let idx = rng.gen_range(0..VOW.len());
            VOW.chars().nth(idx).expect("To get the char")
        } else {
            let idx = rng.gen_range(0..CON.len());
            CON.chars().nth(idx).expect("To get the char")
        };

        buf.push(c);

        if config.with_double && rng.gen_bool(0.05) {
            buf.push(c);
        }
    }

    buf
}

struct WordConfig {
    min: usize,
    max: usize,
    with_double: bool,
}

impl WordConfig {
    fn new() -> Self {
        WordConfig {
            min: 3,
            max: 6,
            with_double: true,
        }
    }

    fn with_min(&mut self, min: usize) -> &mut Self {
        self.min = min;
        self
    }

    fn with_max(&mut self, max: usize) -> &mut Self {
        self.max = max;
        self
    }

    fn with_double(&mut self, with_double: bool) -> &mut Self {
        self.with_double = with_double;
        self
    }
}

fn main() {
    let cli = Cli::parse();
    let count = cli.count.unwrap_or(10);
    let mut rng = rand::thread_rng();
    let mut config = WordConfig::new();
    config
        .with_min(cli.min.unwrap_or(3).into())
        .with_max(cli.max.unwrap_or(6).into())
        .with_double(true);

    for _ in 0..count {
        let word = gen_word(&mut rng, &config);
        println!("{}", word);
    }
}
