//
// (c) 2020 Gabriel DiFiore <difioregabe@gmail.com>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
//

mod block_frequency;
mod data;
mod frequency_monobit;
mod runs_test;
mod utilities;

use block_frequency::block_frequency;
use clap::{App, Arg};
use data::*;
use frequency_monobit::frequency_monobit;
use runs_test::runs_test;
use std::fs::File;
use std::io::Read;

fn main() {
    let matches = App::new("Partial NIST Randomness Testing Suite (in Rust)")
        .version("0.1.0")
        .author("Gabriel DiFiore <difioregabe@gmail.com>")
        .about("Runs a specified NIST Randomness test on given data")
        .arg(
            Arg::with_name("input")
                .short("f")
                .long("file")
                .takes_value(true)
                .value_name("FILE")
                .required(false)
                .help("File containing input data"),
        )
        .arg(
            Arg::with_name("frequency_monobit_test")
                .long("frequency_monobit_test")
                .required(false)
                .takes_value(false)
                .help("Runs Frequency (Monobit) Test on given data"),
        )
        .arg(
            Arg::with_name("block_frequency_test")
                .long("block_frequency_test")
                .required(false)
                .takes_value(false)
                .help("Runs Frequency Test within a Block on given data"),
        )
        .arg(
            Arg::with_name("runs_test")
                .long("runs_test")
                .required(false)
                .takes_value(false)
                .help("Runs Runs Test on given data"),
        )
        .arg(
            Arg::with_name("data_plot")
                .long("data_plot")
                .required(false)
                .takes_value(false)
                .help("Runs Runs Test on given data"),
        )
        .get_matches();

    if matches.is_present("frequency_monobit_test") {
        match File::open(matches.value_of("input").unwrap()) {
            // The file is open (no error).
            Ok(mut file) => {
                // move outside match statement so file isn't mandatory
                let mut content = String::new();

                // Read all the file content into a variable (ignoring the result of the operation).
                file.read_to_string(&mut content).unwrap();

                frequency_monobit(content);

                // The file is automatically closed when is goes out of scope.
            }
            // Error handling.
            Err(error) => {
                println!(
                    "Error opening file {}: {}",
                    matches.value_of("input").unwrap(),
                    error
                );
            }
        }
    } else if matches.is_present("block_frequency_test") {
        match File::open(matches.value_of("input").unwrap()) {
            // The file is open (no error).
            Ok(mut file) => {
                // move outside match statement so file isn't mandatory
                let mut content = String::new();

                // Read all the file content into a variable (ignoring the result of the operation).
                file.read_to_string(&mut content).unwrap();

                block_frequency(content);

                // The file is automatically closed when is goes out of scope.
            }
            // Error handling.
            Err(error) => {
                println!(
                    "Error opening file {}: {}",
                    matches.value_of("input").unwrap(),
                    error
                );
            }
        }
    } else if matches.is_present("runs_test") {
        match File::open(matches.value_of("input").unwrap()) {
            // The file is open (no error).
            Ok(mut file) => {
                // move outside match statement so file isn't mandatory
                let mut content = String::new();

                // Read all the file content into a variable (ignoring the result of the operation).
                file.read_to_string(&mut content).unwrap();

                runs_test(content);
                // The file is automatically closed when is goes out of scope.
            }
            // Error handling.
            Err(error) => {
                println!(
                    "Error opening file {}: {}",
                    matches.value_of("input").unwrap(),
                    error
                );
            }
        }
    } else if matches.is_present("data_plot") {
        let data = read_data("test.txt".to_string());
        let float_data = convert_data_vector(data.unwrap());
        //let sorted_data = sort_data(float_data.unwrap());
        let counted_data = count_data(float_data.unwrap());
        plot_data(counted_data.unwrap());
    }
}
