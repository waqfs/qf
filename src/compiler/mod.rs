pub mod index;
pub mod model;
pub mod source;

use crate::{
    cli::CLIArguments,
    compiler::{
        index::{build_index, process_index},
        model::document::Site,
    },
    config::Config,
    error::QFError,
    output::outputs,
    parser, staging,
};

pub fn build(args: &CLIArguments) -> Result<(), QFError> {
    let (config, site) = compile(args)?;
    let stage = staging::init_stage(&args.root, &config)?;

    for output in outputs() {
        output.write(&site, &config, &stage)?;
    }

    staging::insert_static(&args.root, &config, &stage)?;
    staging::commit_stage(&args.root, &config, &stage)?;
    Ok(())
}

fn compile(args: &CLIArguments) -> Result<(Config, Site), QFError> {
    let config = Config::load(&args.root)?;
    let files = source::get_portfolio_files(&args.root, &config)?;
    let mut documents = Vec::new();

    for file in files {
        match source::get_raw(&file).and_then(|file| parser::parse_document(&file, &config)) {
            Ok(document) => {
                if document.metadata.is_draft && !args.include_drafts {
                    continue;
                }
                documents.push(document);
            }
            Err(error) => {
                return Err(QFError::Message(format!("{}: {error}", file.display())));
            }
        }
    }

    let mut site: Site = build_index(&args.root, config.clone(), documents)?;
    process_index(&config, &mut site)?;
    Ok((config, site))
}
