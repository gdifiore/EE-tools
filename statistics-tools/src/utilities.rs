//
// (c) 2020 Gabriel DiFiore <difioregabe@gmail.com>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
//

use itertools::Itertools;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

pub fn sub_strings(source: &str, sub_size: usize) -> Vec<String> {
    // split string [&str] every [usize] characters

    source
        .chars()
        .chunks(sub_size)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .collect::<Vec<_>>()
}

pub fn file_to_vector(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    // read file [filename]line by line and output as a vector [Vec<String>]

    BufReader::new(File::open(filename)?).lines().collect()
}
