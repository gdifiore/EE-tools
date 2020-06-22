//
// (c) 2020 Gabriel DiFiore <difioregabe@gmail.com>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
//

mod frequency_monobit;
mod block_frequency;
mod utilities;
mod runs_test;

use frequency_monobit::frequency_monobit;
use block_frequency::block_frequency;
use runs_test::runs_test;
use clap::{App, Arg};
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
                .required(true)
                .help("File containing input data")
        )
        .arg(
            Arg::with_name("frequency_monobit_test")
            .long("frequency_monobit_test")
            .required(false)
            .takes_value(false)
            .help("Runs Frequency (Monobit) Test on given data")
        )
        .arg (
            Arg::with_name("block_frequency_test")
            .long("block_frequency_test")
            .required(false)
            .takes_value(false)
            .help("Runs Frequency Test within a Block on given data")
        )
        .arg (
            Arg::with_name("runs_test")
            .long("runs_test")
            .required(false)
            .takes_value(false)
            .help("Runs Runs Test on given data")
        )
        .get_matches();


    match File::open(matches.value_of("input").unwrap()) {
        // The file is open (no error).
        Ok(mut file) => {
            let mut content = String::new();

            // Read all the file content into a variable (ignoring the result of the operation).
            file.read_to_string(&mut content).unwrap();

            if matches.is_present("frequency_monobit_test") {
                println!("Running Frequency (Monobit) Test");
                frequency_monobit(content);
            }
            else if matches.is_present("block_frequency_test") {
                block_frequency(content);
            }
            else if matches.is_present("runs_test") {
                runs_test(content);
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
}