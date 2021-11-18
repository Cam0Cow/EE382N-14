

#[derive(Debug)]
pub struct Multiplier {
    data: Vec<u32>,
    half_adders: u64,
    full_adders: u64,
}

impl Multiplier {
    pub fn new(n: u32) -> Multiplier {
        let mut data : Vec<u32> = Vec::new();
        let offset = 2 * n;
        for i in 1..=(2*n-1) {
            if i > n {
                data.push(offset - i);
            } else {
                data.push(i);
            }
        }
        Multiplier {data: data, half_adders: 0, full_adders: 0}
    }

    pub fn get_height(&self) -> u32 {
        self.data.iter().fold(0u32, |a,&b| if a > b {a} else {b})
    }

    pub fn wallace_reduce(&mut self) {
        if self.get_height() < 3 {return;} // If less than 3 lines then we're done!

        for i in 0..self.data.len() {
            let bits = self.data[i];
            let count = bits / 3; // How many groups of 3 do we have?
            let remainder = bits % 3; // How large is the leftover group?
            let mut sum_total : u32 = 0; // New number of bits after the reduction
            let mut sum_carry : u32 = 0; // How many carry bits are generated?

            self.full_adders += count as u64; // For each group of 3 we have a half adder
            sum_total += count; // Each group of 3 yields 1 bit
            sum_carry += count; // Each group of 3 yields 1 carry

            /*
            If we have a group of 0 or 1, they just get propogated down
            A group of two will go through a half adder
            */
            match remainder {
                0 | 1 => sum_total += remainder,
                2 => {self.half_adders += 1; sum_total += 1},
                _ => unreachable!()
            }

            self.data[i] = sum_total; // Set new number of bits
            if i+1 < self.data.len() {
                self.data[i+1] += sum_carry;
            }
        }
    }
}