pub mod multiplier;

use multiplier::Multiplier;

fn main() {
    let m1 = Multiplier::new(8);
    let m2 = m1.dadda_reduce();
    println!("m2 is {:#?}", m2);
}
