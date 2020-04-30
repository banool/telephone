use factorial::Factorial;
extern crate num_bigint as bigint;

use bigint::{ToBigUint, BigUint};

// x: number of lines
// y: rate of traffic
fn loss(x: u64, y: f64) -> BigUint {
    let x_factorial = x.to_biguint().expect("couldn't make BIG").checked_factorial().expect("failed to factorial x");
    let mut denom = vec![];
    for i in 0..=x {
        let i_factorial = i.to_biguint().expect("could not").checked_factorial().expect("failed to factorial y");
        let blah = y.powi(i as i32).to_biguint().expect("why") / i_factorial;
        denom.push(blah);
    }
    let d: BigUint= denom.iter().sum();
    (y.powi(x as i32).to_biguint().expect("FUCK") / x_factorial).to_biguint().expect("no big") / d
}

fn main() {
    let x = 16000;
    let y: f64 = 7200.0;
    let l = loss(x, y);
    println!("Loss is: {}", l);
}
