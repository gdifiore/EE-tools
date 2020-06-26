//
// (c) 2020 Gabriel DiFiore <difioregabe@gmail.com>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
//

use statrs::function::erf::erfc;
use std::io;
use std::process;

pub fn runs_test(content: &String) -> io::Result<f64> {
    // perform Runs Test on [String]
    // return p-value as [f64]

    println!("PERFORMING RUNS TEST\n");

    println!("Data: {}\n", content);
    let n = content.trim_end().chars().count();

    // if amount if input data is insufficient (n<100), quit the program early
    if n < 100 {
        println!("Insufficient data: {}", n);
        println!("Input must be at least n >= 100");
        process::exit(1);
    }

    println!("Data Length: {}", n);

    // count number of ones
    let mut ones: f64 = 0.0;

    for c in content.chars() {
        if c == '1' {
            ones += 1.0;
        }
    }

    // calculate pre-test proportion value
    let proportion: f64 = ones / n as f64;
    println!("Pre-test proportion: {}", proportion);

    // calculate tau
    let n_float: f64 = n as f64;
    let tau: f64 = 2.0 / (n_float.sqrt());
    println!("tau: {}", tau);

    // test if proportion - 1/2 is greater than or equal to tau
    // if it is, we end the test as it has failed
    let proportion_minus_point_five = proportion as f64 - 0.5;
    println!("{}", proportion_minus_point_five.abs());

    if proportion_minus_point_five.abs() >= tau {
        println!("The absolute value of the proportion - 1/2 is greater than or equal to tau");
        println!("This data set has failed the Frequency (Monobit) Test, and the Runs Test will not be run");
        let p_value: f64 = 0.0;
        return Ok(p_value);
    }

    let mut vobs: f64 = 1.0;
    let char_vec: Vec<char> = content.chars().collect();

    // calculate how many times a character repeats in a row
    for i in 0..content.len() - 1 {
        if char_vec[i as usize] != char_vec[i as usize + 1 as usize] {
            vobs += 1.0;
        }
    }

    println!("vobs is: {}", vobs);

    // calculate the p-value
    let erfc_math = (vobs - 2.0 * n_float * proportion * (1.0 - proportion))
        / (2.0 * (2.0 * n_float).sqrt() * proportion * (1.0 - proportion));
    let p_value = erfc(erfc_math);
    println!("P-value = {}", p_value);

    // if p-value is greater than or equal to 0.01, it is consifered random
    if p_value >= 0.01 {
        println!("Input sequence IS accepted as random\n\n");
    } else {
        println!("Input sequence IS NOT accepted as random\n\n");
    }

    Ok(p_value)
}
