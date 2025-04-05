use std::process::exit;

use colored::Colorize;

mod compile_err;
mod runtime_err;

pub use {compile_err::CompileErr, runtime_err::RuntimeErr};

#[macro_export]
macro_rules! err_report {
    ($err: expr) => {
        crate::common::error::report($err)
    };
}

pub fn report(err: ErrorType) -> ! {
    let (header, msg) = {
        match err {
            ErrorType::Compile(compile) => ("Compile Error".red().bold(), compile.to_string()),
            ErrorType::Runtime(runtime) => ("Runtime Error".red().bold(), runtime.to_string()),
        }
    };

    eprint!("\n{}\n    {}\n\n", header, msg);
    exit(-1)
}

pub enum ErrorType {
    Compile(CompileErr),
    Runtime(RuntimeErr),
}

impl From<CompileErr> for ErrorType {
    fn from(value: CompileErr) -> Self {
        ErrorType::Compile(value)
    }
}

impl From<RuntimeErr> for ErrorType {
    fn from(value: RuntimeErr) -> Self {
        ErrorType::Runtime(value)
    }
}
