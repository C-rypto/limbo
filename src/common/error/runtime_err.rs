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

pub enum RuntimeError {
    TypeError,
    UndeclaredIdt(String),
}

impl core::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TypeError => write!(f, "不知道怎么写总之是类型错误"),
            Self::UndeclaredIdt(idt) => write!(f, "`{}` 这玩意没定义就想用？", idt),
        }
    }
}
