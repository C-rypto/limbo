use std::io::Write;

use value_reader::ValueReader;

use crate::common::{compile_time::ast_types::Root, error::ErrorType, run_time::env::Environment};

mod value_reader;

pub fn compute(root: Root, prev_env: Option<Box<Environment>>) -> Result<(), ErrorType> {
    let mut reader = ValueReader::new(prev_env);

    if let Some((value, _)) = reader.run(root)? {
        print!("{}", value);
        std::io::stdout().flush().unwrap();
    }

    Ok(())
}
