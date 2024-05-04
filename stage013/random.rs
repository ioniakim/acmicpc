/// Pseudo random number generator
/// X_i = (multiplier *X_i-1 + increment) % modulus
/// where modulus > 0,
///     0 < multiplier < m,
///     0 <= increment < m,
///     0 <= X_0 < m

// struct PrnParameter<T> {
//     multiplier: T,

// }

trait UnsignedInt<T> {
    fn max_value() -> T;
    fn min_value() -> T;
}

impl UnsignedInt<u8> for u8 {
    fn max_value() -> u8 {
        u8::MAX
    }

    fn min_value() -> u8 {
        u8::MIN
    }
}

fn gen_rand(seed: u32) -> Box<dyn FnMut() -> u32> {
    let multiplier = 1664525;
    let increment = 1013904223;
    // let modulus = std::u32::MAX;
    let mut rand = seed;
    Box::new(move || {
        rand = rand.wrapping_mul(multiplier).wrapping_add(increment); // % modulus; // I guess modulus is not required here because of wrapping_mul/add
        rand
    })
}

fn main() {
    let mut f = gen_rand(1);
    println!("{}", f());
    println!("{}", f());
    println!("u8 max value: {}", u8::max_value());
}
