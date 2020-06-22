//
// (c) 2020 Gabriel DiFiore <difioregabe@gmail.com>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
//

use itertools::Itertools;

pub fn sub_strings(source: &str, sub_size: usize) -> Vec<String> {
    // split string [&str] every [usize] characters

    source.chars()
        .chunks(sub_size).into_iter()
        .map(|chunk| chunk.collect::<String>())
        .collect::<Vec<_>>()
}