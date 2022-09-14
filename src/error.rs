pub fn error(line: usize, message: &str) {
    report(line, "", message) 
}

pub fn report(
    line: usize, 
    error_where: &str, 
    message: &str,
) {
    println!(
        "[line {l}] Error {e} where: {m}", 
        l = line, 
        e = error_where, 
        m = message,
    );
}
