pub mod multiplier;

use multiplier::Multiplier;

fn main() {
    let mut mult = Multiplier::new(8);
    mult.wallace_multiply();
    println!("Product matrix is currently {:#?}", mult);
}
