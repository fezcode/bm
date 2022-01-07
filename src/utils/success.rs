#[derive(Debug)]
pub struct ExecutionResult {
    pub success: bool,
    pub write_to_file: bool
}

impl Default for ExecutionResult {
    fn default() -> ExecutionResult {
        ExecutionResult {
            success: false,
            write_to_file: false,
        }
    }
}

impl ExecutionResult {
    pub fn print(&self) {
        println!("[DBG|Result:{:?}]", self);
    }
}