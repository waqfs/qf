use crate::{cli::CLIArguments, compiler};

pub fn run(arguments: CLIArguments) -> Result<(), String> {
    let _ = compiler::build(&arguments.root)?;
    Ok(())
}
