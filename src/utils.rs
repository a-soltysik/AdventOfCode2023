use std::fs::File;
use std::io::Write;

pub fn write_output<T: std::fmt::Display>(output: &mut File, value: T) {
    write!(output, "{}\n", value).unwrap();
}
