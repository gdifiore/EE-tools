use crate::block_frequency::block_frequency;
use crate::data::*;
use crate::frequency_monobit::frequency_monobit;
use crate::runs_test::runs_test;
use crate::uniform_distribution::uniform_distribution;
use crate::proportion_of_sequences::proportion_of_sequences;

pub fn test_handler(test_name: &str, n_of_tests: i32, data: Vec<String>, source: &str) {
    let mut n_of_failed_tests: i32 = 0;
    let mut p_values: Vec<f64> = Vec::new();
    let test_func: fn(&str) -> f64;

    match test_name {
        "frequency_monobit_test" => {
            test_func = frequency_monobit;
        }
        "block_frequency_test" => {
            test_func = block_frequency;
        }
        "runs_test" => {
            test_func = runs_test;
        }
        _ => {
            println!("Test not found.");
            return;
        }
    }

    for i in 0..n_of_tests {
        let content = &data[i as usize];
        let p_value = test_func(content);
        if p_value < 0.01 {
            n_of_failed_tests += 1;
        }
        p_values.push(p_value);
    }

    let counted_data = count_data(&p_values.clone());

    plot_data(counted_data.clone(), test_name.to_string(), source);
    println!("{} tests failed", n_of_failed_tests);

    let p_value_of_proportion_of_sequences = proportion_of_sequences(n_of_tests, p_values);
    println!("{:?}", p_value_of_proportion_of_sequences);

    let p_value_of_distribution = uniform_distribution(counted_data.clone(), n_of_tests);
    println!("{:?}", p_value_of_distribution);
}