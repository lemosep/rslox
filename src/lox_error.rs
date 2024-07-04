pub fn error(line: i32, message: &str) {
    report(line, "", message);
}

pub fn report(line: i32, location: &str, message: &str) {
    println!("[line {line}] Error {location}: {message}");
}
