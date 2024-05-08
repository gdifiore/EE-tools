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
    // start timer
    let now = Instant::now();

    let matches = parse_cli_args();

    let (data, source, test_name) = extract_cli_args(&matches);
    let n_tests = data.len() as i32;

    run_test(&test_name, n_tests, data, source);

    print_elapsed_time(now);
}

fn parse_cli_args() -> clap::ArgMatches<'static> {
    App::new("statistics-tools (Part of EE-tools for IB CompSci Extended Essay)")
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
                .help("Source of Data (random.org, HotBits, etc.)"),
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
                .help("Test to perform n trials of (frequency_monobit, block_frequency, runs_test)"),
        )
        .get_matches()
}

fn extract_cli_args<'a>(matches: &'a clap::ArgMatches) -> (Vec<String>, &'a str, &'a str) {
    let data = file_to_vector(matches.value_of("input").unwrap()).unwrap();
    let source = matches.value_of("source").unwrap();
    let test_name = matches.value_of("test").unwrap();

    (data, source, test_name)
}

fn run_test(test_name: &str, n_tests: i32, data: Vec<String>, source: &str) {
    if test_name == "frequency_monobit_test" {
        test_handler("frequency_monobit_test", n_tests, data, source);
    } else if test_name == "block_frequency_test" {
        test_handler("block_frequency_test", n_tests, data, source);
    } else if test_name == "runs_test" {
        test_handler("runs_test", n_tests, data, source);
    }
}

fn print_elapsed_time(now: Instant) {
    let elapsed = now.elapsed();
    println!(
        "Elapsed: {} ms",
        (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64
    );
}