//
// (c) 2020-2024 Gabriel DiFiore <difioregabe@gmail.com>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
//

use chrono::Local;
use plotlib::page::Page;
use plotlib::repr::BarChart;
use plotlib::style::BoxStyle;
use plotlib::view::CategoricalView;
use std::fs;
use std::collections::BTreeMap;

pub fn count_data(data: &[f64]) -> BTreeMap<String, i32> {
    // count occurrences of p-values [0.0, 1.0]

    let mut p_value_count = BTreeMap::new();
    let ranges = [
        (0.0, 0.1),
        (0.1, 0.2),
        (0.2, 0.3),
        (0.3, 0.4),
        (0.4, 0.5),
        (0.5, 0.6),
        (0.6, 0.7),
        (0.7, 0.8),
        (0.8, 0.9),
        (0.9, 1.0),
    ];

    for &(start, end) in &ranges {
        let count = data
            .iter()
            .filter(|&&x| x >= start && x < end)
            .count() as i32;
        let range_str = format!("{}-{}", start, end);
        p_value_count.insert(range_str, count);
    }

    p_value_count
}

pub fn plot_data(p_value_count: BTreeMap<String, i32>, test_name: String, source: &str) {
    // plot p-value data on a bar graph
    let mut bar_charts = Vec::new();
    let mut labels = Vec::new();

    for (key, value) in &p_value_count {
        let range = key.replace(".", "-");
        let label = format!("{}-{}", range, range.split("-").last().unwrap());
        let bar_chart = BarChart::new((*value).into()).label(&label).style(&BoxStyle::new().fill("red"));
        bar_charts.push(bar_chart);
        labels.push(range);
    }

    let mut v = CategoricalView::new().x_label("P-Value").y_label("Frequency");

    for chart in bar_charts {
        v = v.add(chart);
    }

    let dir = fs::create_dir_all("charts");
    if let Err(e) = dir {
        println!("Error creating directory: {}", e);
        return;
    }

    let date = Local::now().format("%c").to_string().replace(':', "-");
    let filepath = format!("charts/{} {} - {}.svg", source, test_name, date);

    if let Err(e) = Page::single(&v).save(filepath) {
        println!("Error saving svg: {}", e);
    }
}
