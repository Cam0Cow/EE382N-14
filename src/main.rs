pub mod multiplier;

use multiplier::Multiplier;

fn main() {
    let mult = Multiplier::new(10);
    println!("Product height is currently {}", mult.get_height());
}
