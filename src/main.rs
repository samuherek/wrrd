use rand::Rng;

const VOW: &str = "aeiou";
const CON: &str = "bcdfghjklmnpqrstvwxyz";
const LEN: usize = 5;

// <con><vow>
// <con><vow><con>
// <vow><con><vow>
fn gen_word(rng: &mut rand::rngs::ThreadRng, len: usize) -> String {
    let start = rng.gen_range(0..=1);
    let len = rng.gen_range(3..=len);
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
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        let word = gen_word(&mut rng, LEN);
        println!("{}", word);
    }
}
