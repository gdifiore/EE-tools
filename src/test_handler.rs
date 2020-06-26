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

pub fn test_handler(test_name: &str, n_of_tests: i32, data: Vec<String>) {
    let mut p_values: Vec<f64> = Vec::new();
    if test_name == "frequency_monobit_test" {
        for i in 0..n_of_tests {
            let content = &data[i as usize];
            let p_value = frequency_monobit(content);
            p_values.push(p_value.unwrap());
        }
        let counted_data = count_data(p_values);
        plot_data(counted_data.unwrap(), test_name.to_string());
    } else if test_name == "block_frequency_test" {
        for i in 0..n_of_tests {
            let content = &data[i as usize];
            let p_value = block_frequency(content);
            p_values.push(p_value.unwrap());
        }
        let counted_data = count_data(p_values);
        plot_data(counted_data.unwrap(), test_name.to_string());
    } else if test_name == "runs_test" {
        for i in 0..n_of_tests {
            let content = &data[i as usize];
            let p_value = runs_test(content);
            println!("{:?}", p_value);
            p_values.push(p_value.unwrap());
        }
        let counted_data = count_data(p_values);
        plot_data(counted_data.unwrap(), test_name.to_string());
    }
}
