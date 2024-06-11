use chrono::Local;
use log::info;
use std::fs;

pub fn write_output(output: &String) {
    let current_timestamp = Local::now().format("%Y-%m-%d-%H-%M-%S").to_string();
    let filename = format!("./output/output-{}.md", current_timestamp);
    info!("Writing output to file: {}", filename);
    fs::write(filename, output).unwrap();
}
