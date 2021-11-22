

#[derive(Debug)]
pub struct Multiplier {
    data: Vec<u32>,
    pub half_adders: u64,
    pub full_adders: u64,
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

    // Get the next height a dadda reduction should reduce to
    pub fn get_nearest_dadda_height(n: u32) -> u32 {
        let mut a : u32 = 3; // Dadda height
        let mut b : u32 = 2; // previous dadda height

        while a < n {
            b = a; // keep the previous value of a
            a = (a * 3) / 2; // Find the next dadda height
        }

        b // The return the largest value still less than n
    }

    /*
     * Given a multiplier, returns a multiplier reduced by one dadda stage with full and half adders accumulated
    */
    pub fn dadda_reduce(&self) -> Multiplier {
        let mut data : Vec<u32> = vec![0; self.data.len()];
        let new_height = Multiplier::get_nearest_dadda_height(self.get_height());
        let mut half_adders: u64 = 0; // half adders for this reduction only
        let mut full_adders: u64 = 0; // full adders for this reduction only

        for (i, &elem) in self.data.iter().enumerate() {
            // println!("elem is {}, data[{}] is {}, available space is {}", elem, i, data[i], new_height - data[i]);
            if elem <= new_height - data[i] {
                data[i] += elem; // copy down elements without reduction if possbile
            } else {
                let diff = elem + data[i] - new_height;
                data[i] = new_height; // The column will be full
                // println!("\tDiff in spaces is {}", diff);

                if diff % 2 == 1 { // If the difference is an odd number, we need a half adder
                    half_adders += 1;
                    data[i+1] += 1; // include carry into next column
                    // println!("\tAdded half adder");
                }

                let fulls = diff / 2; // number of full adders needed for this column
                data[i+1] += fulls;
                full_adders += fulls as u64;
                // println!("\tAdded {} full adders", fulls);
            }
        }


        Multiplier {
            data: data,
            half_adders: self.half_adders + half_adders,
            full_adders: self.full_adders + full_adders
        }
    }

    pub fn dadda_multiplier(self) -> Multiplier {
        let mut mul = self;
        while mul.get_height() > 2 {
            mul = mul.dadda_reduce();
        }

        mul
    }

}

/*
pub fn wallace_reduce(&mut self) {
        if self.get_height() < 3 {return;} // If less than 3 lines then we're done!

        for i in (0..self.data.len()).rev() {
            let bits = self.data[i];
            let count = bits / 3; // How many groups of 3 do we have?
            let remainder = bits % 3; // How large is the leftover group?
            let mut sum_total : u32 = 0; // New number of bits after the reduction
            let mut sum_carry : u32 = 0; // How many carry bits are generated?

            self.full_adders += count as u64; // For each group of 3 we have a half adder
            sum_total += count; // Each group of 3 yields 1 bit
            sum_carry += count; // Each group of 3 yields 1 carry

            /*
            If we have an extra group of 0 or 1, they just get propogated down
            A group of two will go through a half adder
            */
            match remainder {
                0 | 1 => sum_total += remainder,
                2 => {
                    self.half_adders += 1;
                    sum_total += 1;
                    sum_carry += 1; // A half adder has a carry
                },
                _ => unreachable!()
            }

            self.data[i] = sum_total; // Set new number of bits
            if i+1 < self.data.len() {
                self.data[i+1] += sum_carry;
            } else if sum_carry > 0{
                self.data.push(sum_carry);
            }
        }
    }

    pub fn wallace_multiply(&mut self) {
        while self.get_height() > 2 {
            self.wallace_reduce();
        }
    }
*/