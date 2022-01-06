pub enum ErrorCode {
    UnrecognizedArgument = 1,
    NoArgumentProvided = 2,
    HomeDirAccess = 3,
    StoreFileParseError = 4,
    StoreFileSerializationError = 5,
    UnableToWriteToFile = 6,
    CommandVerificationError = 7,
    ImpossibleCommand = 8,
    AddCommandPathNotFound = 9,
    AddCommandOptionMatchFailed = 10,
    AddCommandNotOverwritable = 11,
    HelpPrinted = 255
}

pub fn print_error_and_exit(error_msg : String, error_code : ErrorCode) {
    eprintln!("{}", error_msg);
    std::process::exit(error_code as i32);
}
