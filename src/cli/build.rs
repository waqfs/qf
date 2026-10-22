use crate::{cli::CLIArguments, compiler, error::QFError};

pub fn run(arguments: CLIArguments) -> Result<(), QFError> {
    compiler::build(&arguments.root)?;
    Ok(())
}
