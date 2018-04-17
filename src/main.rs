struct Rand {
    state: u32,
}

impl Rand {
    fn with_seed(seed: u32) -> Rand {
        Rand { state: seed }
    }

    fn next_u32(&mut self) -> u32 {
        // xorshift32 - https://en.wikipedia.org/wiki/Xorshift
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }

    fn rps(&mut self) -> RPS {
        let r = self.next_u32() & 0xFF;
        if r < 85 {
            RPS::Rock
        } else if r < 170 {
            RPS::Paper
        } else {
            RPS::Scissors
        }
    }
}

#[derive(Debug)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

fn main() {
    let mut rand = Rand::with_seed(1234);
    for _ in 0..20 {
        println!("rps: {:?}", rand.rps());
    }
}
