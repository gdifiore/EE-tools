//
// (c) 2020-2023 Gabriel DiFiore <difioregabe@gmail.com>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
//

use special_fun::cephes_single::igamc;
use std::collections::BTreeMap;

pub fn uniform_distribution(data: BTreeMap<String, i32>, n_tests: i32) -> f32 {
    // determine if data [BTreeMap<String, i32>] is uniformly distributed on histogram
    // return p-value as [f64]

    let mut sums: Vec<f32> = vec![0.0; 10];

    let mut i = 0;
    for (_key, value) in data {
        sums[i] = ((value as f32 - (n_tests as f32 / 10.0)).powf(2.0)) / (n_tests as f32 / 10.0);
        i += 1;
    }


    let mut big_x_squared: f32 = 0.0;
    for i in sums {
        big_x_squared += i;
    }
    big_x_squared = big_x_squared.sqrt();


    let p_value: f32 = igamc(9.0 / 2.0, big_x_squared / 2.0);

    println!("Uniform Distribution P-value = {}", p_value);

    if p_value >= 0.0001 {
        println!("The data is considered uniformly distributed\n\n");
    } else {
        println!("The data IS NOT considered uniformly distributed\n\n");
    }

    p_value
}
