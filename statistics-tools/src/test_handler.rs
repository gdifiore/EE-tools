//
// (c) 2020-2023 Gabriel DiFiore <difioregabe@gmail.com>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
//

use crate::block_frequency::block_frequency;
use crate::data::*;
use crate::frequency_monobit::frequency_monobit;
use crate::runs_test::runs_test;
use crate::uniform_distribution::uniform_distribution;
use crate::proportion_of_sequences::proportion_of_sequences;

pub fn test_handler(test_name: &str, n_of_tests: i32, data: Vec<String>, source: &str) {
    // automated testing handler

    let mut n_of_failed_tests: i32 = 0;

    let mut p_values: Vec<f64> = Vec::new();
    if test_name == "frequency_monobit_test" {
        for i in 0..n_of_tests {
            let content = &data[i as usize];
            let p_value = frequency_monobit(content);
            if p_value.clone() < 0.01 {
                n_of_failed_tests = n_of_failed_tests + 1;
            }
            p_values.push(p_value);
        }

        let counted_data = count_data(p_values.clone());

        plot_data(counted_data.clone(), test_name.to_string(), source);

        println!("{} tests failed", n_of_failed_tests);

        let p_value_of_proportion_of_sequences = proportion_of_sequences(n_of_tests, p_values);
        println!("{:?}", p_value_of_proportion_of_sequences);

        let p_value_of_distribution = uniform_distribution(counted_data, n_of_tests);
        println!("{:?}", p_value_of_distribution);

    } else if test_name == "block_frequency_test" {
        for i in 0..n_of_tests {
            let content = &data[i as usize];
            let p_value = block_frequency(content);
            let copy = p_value.clone();
            if copy < 0.01 {
                n_of_failed_tests = n_of_failed_tests + 1;
            }
            p_values.push(p_value);
        }

        let counted_data = count_data(p_values.clone());

        plot_data(counted_data.clone(), test_name.to_string(), source);

        println!("{} tests failed", n_of_failed_tests);

        let p_value_of_proportion_of_sequences = proportion_of_sequences(n_of_tests, p_values);
        println!("{:?}", p_value_of_proportion_of_sequences);

        let p_value_of_distribution = uniform_distribution(counted_data, n_of_tests);
        println!("{:?}", p_value_of_distribution);

    } else if test_name == "runs_test" {
        for i in 0..n_of_tests {
            let content = &data[i as usize];
            let p_value = runs_test(content);
            let copy = p_value.clone();
            if copy < 0.01 {
                n_of_failed_tests = n_of_failed_tests + 1;
            }
            p_values.push(p_value);
        }

        let counted_data = count_data(p_values.clone());

        plot_data(counted_data.clone(), test_name.to_string(), source);

        println!("{} tests failed", n_of_failed_tests);

        let p_value_of_proportion_of_sequences = proportion_of_sequences(n_of_tests, p_values);
        println!("{:?}", p_value_of_proportion_of_sequences);

        let p_value_of_distribution = uniform_distribution(counted_data, n_of_tests);
        println!("{:?}", p_value_of_distribution);
    } else {
        println!("Test not found.");
    }
}
