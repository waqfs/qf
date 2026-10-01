use std::collections::BTreeMap;

use crate::{compiler::model::site::Document, config::Config};

#[derive(Debug, Clone)]
pub struct Site {
    pub config: Config,
    pub documents: Vec<Document>,
    pub routes: BTreeMap<String, usize>,
    pub articles: Vec<usize>,
    pub projects: Vec<usize>,
    pub article_index: Option<usize>,
    pub project_index: Option<usize>,
    pub handles: BTreeMap<String, usize>,
}
