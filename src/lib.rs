extern crate regex;
use regex::Regex;

#[allow(unused_must_use)]
pub fn search(
    content: &str,
    string: &str,
    mut writer: impl std::io::Write,
    file: &str,
) {
    let string = string.to_lowercase();

    let regex = Regex::new(&string).unwrap();
    for line in content.lines() {
        if regex.is_match(&line.to_lowercase()) {
            writeln!(writer, "[{}] {}",file, line);
        }
    }
}