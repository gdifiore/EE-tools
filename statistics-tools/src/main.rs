//
// (c) 2020 Gabriel DiFiore <difioregabe@gmail.com>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
//

pub mod statistics;

mod data;
mod test_handler;
mod utilities;

use std::time::Instant;
use crate::statistics::*;
use clap::{App, Arg};
use test_handler::test_handler;
use utilities::file_to_vector;

fn main() {
    let matches = App::new("statistics-tools (Part of EE-tools for IB CompSci Extended Essay)")
        .version("1.1.0")
        .author("Gabriel DiFiore <difioregabe@gmail.com>")
        .about("Program to automatically run a series of NIST randomness tests from an input file and plot the data")
        .arg(
            Arg::with_name("source")
                .short("s")
                .long("source")
                .required(true)
                .takes_value(true)
                .value_name("source")
                .display_order(3)
                .help("Source of Data (random.org, HotBits, etc.)")
        )
        .arg(
            Arg::with_name("input")
                .short("f")
                .long("file")
                .takes_value(true)
                .value_name("file")
                .required(true)
                .display_order(1)
                .help("File containing input data for randomness tests"),
        )
        .arg(
            Arg::with_name("test")
                .short("t")
                .long("test")
                .required(true)
                .takes_value(true)
                .value_name("test_name")
                .display_order(2)
                .help("Test to perform n trials of (frequency_monobit, block_frequency, runs_test)")
        )
        .get_matches();

    let now = Instant::now();

    if matches.is_present("test") {
        let data = file_to_vector(matches.value_of("input").unwrap());
        let copy_data = data.unwrap().clone();
        let n_tests: i32 = copy_data.len() as i32;
        let source = matches.value_of("source");

        if matches.value_of("test").unwrap() == "frequency_monobit_test" {
            test_handler("frequency_monobit_test", n_tests, copy_data, source.unwrap());
        } else if matches.value_of("test").unwrap() == "block_frequency_test" {
            test_handler("block_frequency_test", n_tests, copy_data, source.unwrap());
        } else if matches.value_of("test").unwrap() == "runs_test" {
            test_handler("runs_test", n_tests, copy_data, source.unwrap());
        }

    }

    let elapsed = now.elapsed();
    println!("Elapsed: {} ms",
             (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);
}
