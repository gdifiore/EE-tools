//
// (c) 2020 Gabriel DiFiore <difioregabe@gmail.com>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
//

use std::io;

pub fn proportion_of_sequences(n_tests: i32, p_values: Vec<f64>) -> io::Result<f64> {

    let mut accepted_p_values: i32 = 0;

    for i in 0..p_values.len() {
        if i as f64 >= 0.01 {
            accepted_p_values += 1;
        }
    }

    let real_proportion: f64 = accepted_p_values as f64/n_tests as f64;

    let p = 1.0 - 0.01;
    let proportion_pt_two = 3.0 * (p * (1.0 - p) / n_tests as f64);

    let theoretical_proportion = p - proportion_pt_two;

    if real_proportion >= theoretical_proportion {
        println!("Expected amount of sequences had an acceptable P-Value");
        println!("Real Proportion: {}", real_proportion);
        println!("Theoretical Proportion: {} ", theoretical_proportion);
    }
    else {
        println!("Unexpected amount of sequences had an acceptable P-Value");
        println!("Real Proportion: {}", real_proportion);
        println!("Theoretical Proportion: {} ", theoretical_proportion);
    }

    Ok(real_proportion)
}
