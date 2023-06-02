use std::{
    ffi::OsString,
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
};

use crate::command::Command;

const LINE_SIZE: u64 = 1024;
const CHAR_SEQ: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn run(command: &Command) {
    init_directory(&command.output);

    for index in 0..command.file_number {
        let mut filename = PathBuf::from(&command.output);
        filename.push(format!("output-{:05}.txt", index));

        fill_file_with_size(command.file_size, filename.as_path());
    }
}

fn init_directory(dir: &OsString) {
    let p = Path::new(dir);
    if p.exists() {
        std::fs::remove_dir_all(p).expect("failed to delete direcotry: output");
    }
    std::fs::create_dir(p).expect("unable to create directory: output");
}

fn fill_file_with_size(size: u64, filename: &Path) {
    let mut total = size;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
        .unwrap();

    while total > LINE_SIZE {
        let s = line_content(LINE_SIZE);
        file.write_all(s.as_bytes()).unwrap();
        total -= LINE_SIZE;
    }
    let s = line_content(total);
    file.write_all(s.as_bytes()).unwrap();
    file.flush().ok();
}

fn line_content(size: u64) -> String {
    let mut length = size as usize;

    let mut line = String::new();
    if length == 0 {
        return line;
    }

    while length > CHAR_SEQ.len() {
        line.push_str(CHAR_SEQ);
        length -= CHAR_SEQ.len();
    }
    line.push_str(&CHAR_SEQ[..length - 1]);
    line.push('\n');

    line
}
