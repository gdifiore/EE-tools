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
mod test_handler;
mod utilities;

use clap::{App, Arg};
use test_handler::test_handler;
use utilities::file_to_vector;

fn main() {
    let matches = App::new("EE-tools for IB CompSci Extended Essay")
        .version("1.0.1")
        .author("Gabriel DiFiore <difioregabe@gmail.com>")
        .about("Program to automatically run a series of NIST randomness tests from an input file and plot the data")
        .arg(
            Arg::with_name("test")
                .short("t")
                .long("test")
                .required(true)
                .takes_value(true)
                .value_name("test_name")
                .help("Test to perform n trials of (frequency_monobit, block_frequency, runs_test)")
        )
        .arg(
            Arg::with_name("input")
                .short("f")
                .long("file")
                .takes_value(true)
                .value_name("file")
                .required(true)
                .help("File containing input data for randomness tests"),
        )
        .get_matches();

    if matches.is_present("test") {
        let data = file_to_vector(matches.value_of("input").unwrap());
        let copy_data = data.unwrap().clone();
        let n_tests: i32 = copy_data.len() as i32;

        if matches.value_of("test").unwrap() == "frequency_monobit_test" {
            test_handler("frequency_monobit_test", n_tests, copy_data);
        } else if matches.value_of("test").unwrap() == "block_frequency_test" {
            test_handler("block_frequency_test", n_tests, copy_data);
        } else if matches.value_of("test").unwrap() == "runs_test" {
            test_handler("runs_test", n_tests, copy_data);
        }
    }
}
