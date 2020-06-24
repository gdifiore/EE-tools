//
// (c) 2020 Gabriel DiFiore <difioregabe@gmail.com>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
//

use crate::utilities::sub_strings;
use math::round;
use statrs::function::gamma::gamma_ur;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::process;

pub fn block_frequency(content: String) -> io::Result<f64> {
    // perform Frequency Within a Block Test on [String]
    // return p-value as [f64]

    println!("PERFORMING FREQUENCY WITHIN A BLOCK TEST \n");

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
    let mut big_m: f64 = 10.0;
    let n_m_frac: f64 = n as f64 / big_m;
    let mut big_n = round::floor(n_m_frac, 0);

    if big_n > 99 as f64 {
        big_n = 99 as f64;
        let n_n_frac: f64 = n as f64 / big_n;
        big_m = round::floor(n_n_frac, 0);
    }

    let mut proportion_sum: f64 = 0.0;
    // set number of blocks and size of said blocks
    let num_of_blocks: f64 = big_n as f64;
    let block_size: f64 = big_m as f64;

    // if more data is availible than is able to make even blocks, remove extra data from vector
    let mut blocks: Vec<String> = sub_strings(&content, big_m as usize);
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
        println!("Input sequence IS accepted as random\n\n");
    } else {
        println!("Input sequence IS NOT accepted as random\n\n");
    }

    // writing to file
    let filename = "data/data.txt";

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)
        .unwrap();

    let newline = "\n";
    let mut p_value_string = p_value.to_string();
    p_value_string = format!("{}{}", newline, p_value_string);
    file.write_all(p_value_string.as_bytes())
        .expect("could not write p-value to file");

    Ok(p_value)
}
