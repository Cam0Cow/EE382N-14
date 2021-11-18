pub mod multiplier;

use multiplier::Multiplier;

fn main() {
    println!("The closets Dadda height to 16-bits is {}", Multiplier::get_nearest_dadda_height(16));
}
