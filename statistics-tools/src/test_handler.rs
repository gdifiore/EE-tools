//
// (c) 2020 Gabriel DiFiore <difioregabe@gmail.com>
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

    let n_of_tests_copy = n_of_tests.clone();
    let n_of_tests_copy_copy = n_of_tests.clone();

    let mut p_values: Vec<f64> = Vec::new();
    if test_name == "frequency_monobit_test" {
        for i in 0..n_of_tests {
            let content = &data[i as usize];
            let p_value = frequency_monobit(content);
            p_values.push(p_value.unwrap());
        }

        let p_values_copy = p_values.clone();

        let counted_data = count_data(p_values).unwrap();
        let copy_of_counted_data = counted_data.clone();

        plot_data(counted_data, test_name.to_string(), source);

        let p_value_of_proportion_of_sequences = proportion_of_sequences(n_of_tests_copy_copy, p_values_copy);
        println!("{:?}", p_value_of_proportion_of_sequences.unwrap());

        let p_value_of_distribution = uniform_distribution(copy_of_counted_data, n_of_tests_copy);
        println!("{:?}", p_value_of_distribution.unwrap());

    } else if test_name == "block_frequency_test" {
        for i in 0..n_of_tests {
            let content = &data[i as usize];
            let p_value = block_frequency(content);
            p_values.push(p_value.unwrap());
        }

        let p_values_copy = p_values.clone();

        let counted_data = count_data(p_values).unwrap();
        let copy_of_counted_data = counted_data.clone();

        plot_data(counted_data, test_name.to_string(), source);

        let p_value_of_proportion_of_sequences = proportion_of_sequences(n_of_tests_copy_copy, p_values_copy);
        println!("{:?}", p_value_of_proportion_of_sequences.unwrap());

        let p_value_of_distribution = uniform_distribution(copy_of_counted_data, n_of_tests_copy);
        println!("{:?}", p_value_of_distribution.unwrap());

    } else if test_name == "runs_test" {
        for i in 0..n_of_tests {
            let content = &data[i as usize];
            let p_value = runs_test(content);
            println!("{:?}", p_value);
            p_values.push(p_value.unwrap());
        }

        let p_values_copy = p_values.clone();

        let counted_data = count_data(p_values).unwrap();
        let copy_of_counted_data = counted_data.clone();

        plot_data(counted_data, test_name.to_string(), source);

        let p_value_of_proportion_of_sequences = proportion_of_sequences(n_of_tests_copy_copy, p_values_copy);
        println!("{:?}", p_value_of_proportion_of_sequences.unwrap());

        let p_value_of_distribution = uniform_distribution(copy_of_counted_data, n_of_tests_copy);
        println!("{:?}", p_value_of_distribution.unwrap());
    }
}
