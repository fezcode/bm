pub enum ErrorCode {
    UnrecognizedArgument = 1,
    HomeDirAccess,
    StoreFileParseError,
    VerificationError,
    ImpossibleCommand
}

pub fn print_error(error_msg : String, error_code : ErrorCode) {
    eprintln!("{}", error_msg);
    std::process::exit(error_code as i32);
}
