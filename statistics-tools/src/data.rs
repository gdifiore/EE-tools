//
// (c) 2020-2023 Gabriel DiFiore <difioregabe@gmail.com>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
//

use chrono::Local;
use plotlib::page::Page;
use plotlib::repr::BarChart;
use plotlib::style::BoxStyle;
use plotlib::view::CategoricalView;
use std::collections::BTreeMap;
use std::fs;
use std::process;

pub fn count_data(data: Vec<f64>) -> BTreeMap<String, i32> {
    // count occurences of p-values (between 2 values e.g. 0.0-0.1)

    // create vector to hold counts of each p-value range
    let mut counts: Vec<i32> = vec![0; 10];

    for i in 0..data.len() {
        if data[i] >= 0.0 && 0.1 >= data[i] {
            counts[0] += 1;
        } else if data[i] >= 0.1 && 0.2 >= data[i] {
            counts [1]+= 1;
        } else if data[i] >= 0.2 && 0.3 >= data[i] {
            counts[2] += 1;
        } else if data[i] >= 0.3 && 0.4 >= data[i] {
            counts[3] += 1;
        } else if data[i] >= 0.4 && 0.5 >= data[i] {
            counts[4] += 1;
        } else if data[i] >= 0.5 && 0.6 >= data[i] {
            counts[5] += 1;
        } else if data[i] >= 0.6 && 0.7 >= data[i] {
            counts[6] += 1;
        } else if data[i] >= 0.7 && 0.8 >= data[i] {
            counts[7] += 1;
        } else if data[i] >= 0.8 && 0.9 >= data[i] {
            counts[8] += 1;
        } else if data[i] >= 0.9 && 1.0 >= data[i] {
            counts[9] += 1;
        } else {
            println!("[ERROR] Number not between 0.0 and 1.0: {}", data[i]);
            println!("Exiting program");
            process::exit(1);
        }
    }

    let mut p_value_count = BTreeMap::new();

    p_value_count.insert(String::from("0.1"), counts[0]);
    p_value_count.insert(String::from("0.2"), counts[1]);
    p_value_count.insert(String::from("0.3"), counts[2]);
    p_value_count.insert(String::from("0.4"), counts[3]);
    p_value_count.insert(String::from("0.5"), counts[4]);
    p_value_count.insert(String::from("0.6"), counts[5]);
    p_value_count.insert(String::from("0.7"), counts[6]);
    p_value_count.insert(String::from("0.8"), counts[7]);
    p_value_count.insert(String::from("0.9"), counts[8]);
    p_value_count.insert(String::from("1.0"), counts[9]);

    p_value_count
}

pub fn plot_data(p_value_count: BTreeMap<String, i32>, test_name: String, source: &str) {
    // plot p-value data on a bar graph
    let b1 = BarChart::new(p_value_count["0.1"].into())
        .label("0-.1")
        .style(&BoxStyle::new().fill("red"));
    let b2 = BarChart::new(p_value_count["0.2"].into())
        .label(".1-.2")
        .style(&BoxStyle::new().fill("red"));
    let b3 = BarChart::new(p_value_count["0.3"].into())
        .label(".2-.3")
        .style(&BoxStyle::new().fill("red"));
    let b4 = BarChart::new(p_value_count["0.4"].into())
        .label(".3-.4")
        .style(&BoxStyle::new().fill("red"));
    let b5 = BarChart::new(p_value_count["0.5"].into())
        .label(".4-.5")
        .style(&BoxStyle::new().fill("red"));
    let b6 = BarChart::new(p_value_count["0.6"].into())
        .label(".5-.6")
        .style(&BoxStyle::new().fill("red"));
    let b7 = BarChart::new(p_value_count["0.7"].into())
        .label(".6-.7")
        .style(&BoxStyle::new().fill("red"));
    let b8 = BarChart::new(p_value_count["0.8"].into())
        .label(".7-.8")
        .style(&BoxStyle::new().fill("red"));
    let b9 = BarChart::new(p_value_count["0.9"].into())
        .label(".8-.9")
        .style(&BoxStyle::new().fill("red"));
    let b10 = BarChart::new(p_value_count["1.0"].into())
        .label(".9-1.0")
        .style(&BoxStyle::new().fill("red"));

    let v = CategoricalView::new()
        .add(b1)
        .add(b2)
        .add(b3)
        .add(b4)
        .add(b5)
        .add(b6)
        .add(b7)
        .add(b8)
        .add(b9)
        .add(b10)
        .x_label("P-Value")
        .y_label("Frequency");

    let _dir = fs::create_dir_all("charts/");
    let date = Local::now().format("%c").to_string().replace(':', "-");
    let filepath = format!("{}{} {}{}{}{}", "charts/", source, test_name, " - ", date, ".svg");
    let _file_name = Page::single(&v).save(filepath).expect("saving svg");
}
