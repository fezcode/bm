use std::collections::HashMap;
use crate::utils::error;
use crate::utils::success::ExecutionResult;
use std::io::Write;
/// bm show
/// bm show NAME
pub fn show(params: &Option<Vec<String>>, store: &mut HashMap<String, String>) -> ExecutionResult {

    // unsafe {
    //     libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    // }

    let name : String;
    match params {
        // Show all
        None => {
            for x in store {
                println!("{} {}", x.0, x.1);
            }
        }
        Some(p) => {
            name = p[0].to_owned();
            if store.contains_key(name.as_str()) {
                let prnt =store[&name].to_owned();
                // https://github.com/rust-lang/rust/issues/46016#issuecomment-415444138
                // writeln!(std::io::stdout(), "{}", store[&name]).unwrap();
                // println!("{}", store[&name]);
                write!(std::io::stdout(), "{: <width$}\n", prnt , width=prnt.len());

                return ExecutionResult{ success: true, write_to_file: false };
            } else {
                error::print_error_and_exit(format!("Given key `{}` does not exist.", name),
                                            error::ErrorCode::ShowCommandNameNotFound)
            }
        }
    }
    return ExecutionResult{ success: false, write_to_file: false };
}