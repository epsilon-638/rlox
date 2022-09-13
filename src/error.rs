pub fn error(line: i32, message: String) {
    report(line, String::from(""), message) 
}

pub fn report(
    line: i32, 
    error_where: String, 
    message: String,
) {
    println!(
        "[line {l}] Error {e} where: {m}", 
        l = line, 
        e = error_where, 
        m = message,
    );
}
