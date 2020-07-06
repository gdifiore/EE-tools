//
// (c) 2020 Gabriel DiFiore <difioregabe@gmail.com>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
//

use special_fun::cephes_single::igamc;
use std::collections::BTreeMap;
use std::io;

pub fn uniform_distribution(data: BTreeMap<String, i32>, n_tests: i32) -> io::Result<f32> {
    // determine if data [BTreeMap<String, i32>] is uniformly distributed on histogram
    // return p-value as [f64]

    let n_point_1: f32 = data["0.1"] as f32;
    let n_point_2: f32 = data["0.2"] as f32;
    let n_point_3: f32 = data["0.3"] as f32;
    let n_point_4: f32 = data["0.4"] as f32;
    let n_point_5: f32 = data["0.5"] as f32;
    let n_point_6: f32 = data["0.6"] as f32;
    let n_point_7: f32 = data["0.7"] as f32;
    let n_point_8: f32 = data["0.8"] as f32;
    let n_point_9: f32 = data["0.9"] as f32;
    let n_1_point_0: f32 = data["1.0"] as f32;

    let sum_point_1: f32 =
        ((n_point_1 - (n_tests as f32 / 10.0)).powf(2.0)) / (n_tests as f32 / 10.0);
    let sum_point_2: f32 =
        ((n_point_2 - (n_tests as f32 / 10.0)).powf(2.0)) / (n_tests as f32 / 10.0);
    let sum_point_3: f32 =
        ((n_point_3 - (n_tests as f32 / 10.0)).powf(2.0)) / (n_tests as f32 / 10.0);
    let sum_point_4: f32 =
        ((n_point_4 - (n_tests as f32 / 10.0)).powf(2.0)) / (n_tests as f32 / 10.0);
    let sum_point_5: f32 =
        ((n_point_5 - (n_tests as f32 / 10.0)).powf(2.0)) / (n_tests as f32 / 10.0);
    let sum_point_6: f32 =
        ((n_point_6 - (n_tests as f32 / 10.0)).powf(2.0)) / (n_tests as f32 / 10.0);
    let sum_point_7: f32 =
        ((n_point_7 - (n_tests as f32 / 10.0)).powf(2.0)) / (n_tests as f32 / 10.0);
    let sum_point_8: f32 =
        ((n_point_8 - (n_tests as f32 / 10.0)).powf(2.0)) / (n_tests as f32 / 10.0);
    let sum_point_9: f32 =
        ((n_point_9 - (n_tests as f32 / 10.0)).powf(2.0)) / (n_tests as f32 / 10.0);
    let sum_1_point_0: f32 =
        ((n_1_point_0 - (n_tests as f32 / 10.0)).powf(2.0)) / (n_tests as f32 / 10.0);

    let big_x_squared: f32 = (sum_point_1
        + sum_point_2
        + sum_point_3
        + sum_point_4
        + sum_point_5
        + sum_point_6
        + sum_point_7
        + sum_point_8
        + sum_point_9
        + sum_1_point_0)
        .sqrt();

    let p_value: f32 = igamc(9.0 / 2.0, big_x_squared / 2.0);

    println!("Uniform Distribution P-value = {}", p_value);

    if p_value >= 0.0001 {
        println!("The data is considered uniformly distributed\n\n");
    } else {
        println!("The data IS NOT considered uniformly distributed\n\n");
    }

    return Ok(p_value);
}
