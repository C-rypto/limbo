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

    eprint!("\n{} at {}\n\n", header, msg);
    exit(-1)
}

pub fn unwrap<T>(value: Result<T, ErrorType>) -> T {
    match value {
        Ok(result) => result,
        Err(err) => err_report!(err),
    }
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

impl From<std::io::Error> for ErrorType {
    fn from(value: std::io::Error) -> Self {
        Self::Compile(CompileErr::from(value))
    }
}
