pub fn error(line: i32, message: String) {
    report(line, "", message);
}

pub fn report(line: i32, location: &str, message: String) {
    println!("[line {line}] Error {location}: {message}");
}
