pub enum ErrorCode {
    UnrecognizedArgument = 1,
    HomeDirAccess,
    StoreFileParseError,
    VerificationError,
    ImpossibleCommand,
    AddCommandPathNotFound,
    AddCommandOptionMatchFailed
}

pub fn print_error_and_exit(error_msg : String, error_code : ErrorCode) {
    eprintln!("{}", error_msg);
    std::process::exit(error_code as i32);
}
