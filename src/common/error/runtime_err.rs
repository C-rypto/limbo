use colored::Colorize;

#[macro_export]
macro_rules! runtime_err {
	($msg: expr) => {
		crate::common::error::runtime_err::report($msg, file!(), line!())
	};
}

pub fn report<MSG>(msg: MSG, file: &'static str, line: u32) -> !
where
    MSG: ToString,
{
    panic!(
        "{} at {}:{} {}",
        "Runtime Error".red().bold(),
        file,
        line,
        msg.to_string().white()
    )
}
