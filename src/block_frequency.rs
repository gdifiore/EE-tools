//
// (c) 2020 Gabriel DiFiore <difioregabe@gmail.com>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
//

use crate::utilities::sub_strings;
use math::round;
use statrs::function::gamma::gamma_ur;
use std::process;

pub fn block_frequency(content: String) {
    // perform Frequency Within a Block Test on [String]

    println!("Data: {}\n", content);
    let n = content.trim_end().chars().count();

    // if amount if input data is insufficient (n<100), quit the program early
    if n < 100 {
        println!("Insufficient data: {}", n);
        println!("Input must be at least n >= 100");
        process::exit(1);
    }

    println!("Data Length: {}", n);

    // set block size to 10
    let mut M: f64 = 10.0;
    let n_MFrac: f64 = n as f64 / M;
    let mut N = round::floor(n_MFrac, 0);

    if N > 99 as f64 {
        N = 99 as f64;
        let n_NFrac: f64 = n as f64 / N;
        M = round::floor(n_NFrac, 0);
    }

    let mut proportion_sum: f64 = 0.0;
    // set number of blocks and size of said blocks
    let num_of_blocks: f64 = N as f64;
    let block_size: f64 = M as f64;

    // if more data is availible than is able to make even blocks, remove extra data from vector
    let mut blocks: Vec<String> = sub_strings(&content, M as usize);
    let block_to_delete: f64 = n as f64 / block_size as f64;
    let delete_block = round::ceil(block_to_delete, 0);

    if block_to_delete != delete_block {
        blocks.remove(delete_block as usize - 1);
    }

    println!("Number of blocks: {}", num_of_blocks);

    // count 1s in data list
    for i in 0..blocks.len() {
        let mut ones_count: i32 = 0;

        for c in blocks[i as usize].chars() {
            if c == '1' {
                ones_count += 1;
            }
        }

        let pi: f64 = ones_count as f64 / block_size as f64;
        let new_pi = pi - 0.5;
        proportion_sum += new_pi.powf(2.0);
    }

    println!("Proportion Sum: {}", proportion_sum);
    // calculate p-value
    let chi_squared = 4 as f64 * block_size as f64 * proportion_sum;
    println!("Chi-Squared Value: {}", chi_squared);
    let p_value = gamma_ur(num_of_blocks / 2.0 as f64, chi_squared / 2.0);

    println!("P-value = {}", p_value);

    // if p-value is greater than or equal to 0.01, it is consifered random
    if p_value >= 0.01 {
        println!("Input sequence IS accepted as random");
    } else {
        println!("Input sequence IS NOT accepted as random");
    }
}
