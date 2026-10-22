use crate::{cli::CLIArguments, compiler, error::QFError};

pub fn run(arguments: CLIArguments) -> Result<(), QFError> {
    let _ = compiler::build(&arguments.root)?;
    Ok(())
}
