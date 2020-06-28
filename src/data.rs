//
// (c) 2020 Gabriel DiFiore <difioregabe@gmail.com>
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
use std::io;
use std::process;

pub fn count_data(data: Vec<f64>) -> io::Result<BTreeMap<String, i32>> {
    // count occurences of p-values (between 2 values e.g. 0.0-0.1)

    let mut point_1: i32 = 0;
    let mut point_2: i32 = 0;
    let mut point_3: i32 = 0;
    let mut point_4: i32 = 0;
    let mut point_5: i32 = 0;
    let mut point_6: i32 = 0;
    let mut point_7: i32 = 0;
    let mut point_8: i32 = 0;
    let mut point_9: i32 = 0;
    let mut one_point_zero: i32 = 0;

    for i in 0..data.len() {
        if data[i] >= 0.0 && 0.1 >= data[i] {
            point_1 += 1;
        } else if data[i] >= 0.1 && 0.2 >= data[i] {
            point_2 += 1;
        } else if data[i] >= 0.2 && 0.3 >= data[i] {
            point_3 += 1;
        } else if data[i] >= 0.3 && 0.4 >= data[i] {
            point_4 += 1;
        } else if data[i] >= 0.4 && 0.5 >= data[i] {
            point_5 += 1;
        } else if data[i] >= 0.5 && 0.6 >= data[i] {
            point_6 += 1;
        } else if data[i] >= 0.6 && 0.7 >= data[i] {
            point_7 += 1;
        } else if data[i] >= 0.7 && 0.8 >= data[i] {
            point_8 += 1;
        } else if data[i] >= 0.8 && 0.9 >= data[i] {
            point_9 += 1;
        } else if data[i] >= 0.9 && 1.0 >= data[i] {
            one_point_zero += 1;
        } else {
            println!("[ERROR] number not between 0.0 and 1.0: {}", data[i]);
            println!("Exiting program");
            process::exit(1);
        }
    }

    let mut p_value_count = BTreeMap::new();

    p_value_count.insert(String::from("0.1"), point_1);
    p_value_count.insert(String::from("0.2"), point_2);
    p_value_count.insert(String::from("0.3"), point_3);
    p_value_count.insert(String::from("0.4"), point_4);
    p_value_count.insert(String::from("0.5"), point_5);
    p_value_count.insert(String::from("0.6"), point_6);
    p_value_count.insert(String::from("0.7"), point_7);
    p_value_count.insert(String::from("0.8"), point_8);
    p_value_count.insert(String::from("0.9"), point_9);
    p_value_count.insert(String::from("1.0"), one_point_zero);

    return Ok(p_value_count);
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
