//
// (c) 2020-2024 Gabriel DiFiore <difioregabe@gmail.com>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
//

use statrs::function::erf::erfc;
use std::process;

pub fn frequency_monobit(content: &str) -> f64 {
    // perform Frequency (Monobit) Test on [String]
    // return p-value as [f64]

    println!("PERFORMING FREQUENCY (MONOBIT) TEST\n");

    println!("Data: {}\n", content);
    let n: usize = content.trim_end().chars().count();

    // if amount if input data is insufficient (n<100), quit the program early
    if n < 100 {
        println!("Insufficient data: {}", n);
        println!("Input must be at least n >= 100");
        process::exit(1);
    }

    println!("Data Length: {}", n);

    // +1 for every 1, -1 for every 0
    let mut count: i32 = 0;
    for c in content.chars() {
        if c == '0' {
            count -= 1;
        } else {
            count += 1;
        }
    }

    println!("S100 = {}", count);

    // calculate sobs value
    let real_len: f64 = n as f64;
    let real_count: f64 = count as f64;
    let sobs: f64 = real_count.abs() / real_len.sqrt();
    println!("sobs = {}", sobs);

    // calculate p-value
    let two: f64 = 2.0;
    let p_value: f64 = erfc(sobs / two.sqrt());
    println!("P-value = {}", p_value);

    // if p-value is greater than or equal to 0.01, it is consifered random
    if p_value >= 0.01 {
        println!("Input sequence IS accepted as random\n\n");
    } else {
        println!("Input sequence IS NOT accepted as random\n\n");
    }

    p_value
}
