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
    let matches = App::new("EE-tools for IB CompSci Extended Essay")
        .version("0.2.0")
        .author("Gabriel DiFiore <difioregabe@gmail.com>")
        .about("Can run a variety of NIST randomness tests, write/read/plot data and automatically run a series of tests")
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
            Arg::with_name("data")
                .short("a")
                .long("data")
                .takes_value(true)
                .value_name("DATA")
                .required(false)
                .help("File containing witten data"),
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
            Arg::with_name("all")
                .long("all")
                .required(false)
                .takes_value(false)
                .help("Runs all tests on given data"),
        )
        .arg(
            Arg::with_name("data_plot")
                .long("data_plot")
                .required(false)
                .takes_value(false)
                .help("for testing data plot functions on given data"),
        )
        .arg(
            Arg::with_name("write")
                .short("w")
                .required(false)
                .takes_value(false)
                .help("If supplied data will be written to file"),
        )
        .arg(
            Arg::with_name("plot")
                .short("p")
                .required(false)
                .takes_value(false)
                .help("If supplied data will be automatically plotted and outputted"),
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

                if matches.is_present("write") {
                    let _p_value = frequency_monobit(content, true);
                } else {
                    let _p_value = frequency_monobit(content, false);
                }

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

                if matches.is_present("write") {
                    let _p_value = block_frequency(content, true);
                } else {
                    let _p_value = block_frequency(content, false);
                }

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
    } else if matches.is_present("all") {
        match File::open(matches.value_of("input").unwrap()) {
            // The file is open (no error).
            Ok(mut file) => {
                // move outside match statement so file isn't mandatory
                let mut content = String::new();

                // Read all the file content into a variable (ignoring the result of the operation).
                file.read_to_string(&mut content).unwrap();
                let content_two = content.clone();
                let content_three = content.clone();

                if matches.is_present("write") {
                    let _p_value = frequency_monobit(content, true);
                    let _p_value = block_frequency(content_two, true);
                    let _p_value = runs_test(content_three, true);
                } else {
                    let _p_value = frequency_monobit(content, false);
                    let _p_value = block_frequency(content_two, false);
                    let _p_value = runs_test(content_three, false);
                }

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
        let data = read_data("data/data.txt".to_string());
        //println!("{:?}", data);
        let float_data = convert_data_vector(data.unwrap());
        //println!("{:?}", float_data);
        let sorted_data = sort_data(float_data.unwrap());
        let counted_data = count_data(sorted_data.unwrap());
        plot_data(counted_data.unwrap());
    }
}
