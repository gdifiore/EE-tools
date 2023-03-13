//
// (c) 2020-2023 Gabriel DiFiore <difioregabe@gmail.com>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
//

pub fn proportion_of_sequences(n_tests: i32, p_values: Vec<f64>) -> f64 {

    let mut accepted_p_values: i32 = 0;

    for i in 0..p_values.len() {
        if p_values[i] as f64 >= 0.01 {
            accepted_p_values += 1;
        }
    }

    let real_proportion: f64 = accepted_p_values as f64/n_tests as f64;


    let p = 1.0 - 0.01;
    let proportion_pt_two: f64 = 3.0 * ((p * (1.0 - p)) / n_tests as f64);

    let theo_proportion = 0.99 - proportion_pt_two;

    if real_proportion >= theo_proportion {
        println!("\nExpected amount of sequences had an acceptable P-Value");
        println!("Real Proportion: {}", real_proportion);
        println!("Theoretical Proportion: .99 +/- {}\n",proportion_pt_two);

        println!("{}", theo_proportion);
    }
    else {
        println!("\nUnexpected amount of sequences had an acceptable P-Value");
        println!("Real Proportion: {}", real_proportion);
        println!("Theoretical Proportion: .99 +/- {}\n", proportion_pt_two);

        let theo_proportion = 0.99 - proportion_pt_two;
        println!("{}", theo_proportion);
    }

    real_proportion
}
