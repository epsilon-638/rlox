pub enum ErrorType {
    UnterminatedString,
    UnexpectedEndOfFile,
    UnexpectedChar,
}

pub fn parser_error(line: usize, error_type: ErrorType) {
    match error_type {
        ErrorType::UnterminatedString => {
            report(line, "", "Unterminated String");
        },
        ErrorType::UnexpectedEndOfFile => {
            report(line, "", "Unexpected End Of File");
        },
        ErrorType::UnexpectedChar => {
            report(line, "", "Unexpected Character");
        },
    }
}

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
