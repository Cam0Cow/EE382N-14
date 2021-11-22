pub mod multiplier;

use multiplier::Multiplier;

fn main() {
    let m1 = Multiplier::new(8);
    let m2 = m1.dadda_multiplier();
    // println!("m2 is {:#?}", m2);
    println!("Multiplier has {} half adders and {} full adders", m2.half_adders, m2.full_adders);
}
