use std::{process::Command, fs};
use crate::config::{self, Config};

fn open_editor(filename: &str) {
    let editor_program = config::read_config(Config::EditorProgram).unwrap();
    let editor_args = config::read_config(Config::EditorArgs)
        .unwrap()
        .split(" ")
        .map(|arg| arg.replace("%1", filename))
        .collect::<Vec<String>>();

    // TODO: Fix -- this is not waiting for the editor to close
    Command::new(editor_program)
        .args(editor_args)
        .status()
        .expect("Failed to open editor");
}

pub fn get_input(request: String) -> String {
    let temp_filename = std::env::temp_dir().join("input.txt");

    fs::write(&temp_filename, request)
        .expect("Failed to write input file");

    open_editor(temp_filename.to_str().unwrap());

    fs::read_to_string(temp_filename)
        .expect("Failed to read input file")
}