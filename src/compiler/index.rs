use std::{collections::BTreeMap, path::Path};

use crate::{
    compiler::model::{
        document::Site,
        site::{Document, DocumentType},
    },
    config::Config,
};

pub fn build_index(
    root: &Path,
    config: Config,
    mut documents: Vec<Document>,
) -> Result<Site, String> {
    let mut routes = BTreeMap::new();
    let mut articles = Vec::new();
    let mut projects = Vec::new();

    for (index, document) in documents.iter_mut().enumerate() {
        document.route = route(document, root, &config);
        if routes.insert(document.route.clone(), index).is_some() {
            return Err(format!(
                "Error indexing routes, duplicates not allowed: {}",
                document.route
            ));
        }

        match document.metadata.doc_type {
            DocumentType::Article => articles.push(index),
            DocumentType::Project => projects.push(index),
            DocumentType::Page => {}
        }
    }

    Ok(Site {
        config,
        documents,
        routes,
        articles,
        projects,
    })
}

fn route(document: &Document, root: &Path, config: &Config) -> String {
    let content_root = root.join(&config.content_dir);
    let relative = document
        .source_path
        .strip_prefix(&content_root)
        .unwrap_or(&document.source_path);

    let mut parts: Vec<String> = relative
        .components()
        .map(|component| component.as_os_str().to_string_lossy().to_string())
        .collect();

    if let Some(last) = parts.last_mut() {
        *last = last.trim_end_matches(".qf").to_string();
        if last == "index" {
            parts.pop();
        }
    }

    normalize_route(&format!("/{}", parts.join("/")))
}

fn normalize_route(route: &str) -> String {
    let mut route = format!("/{}", route.trim_matches('/'));
    if route != "/" {
        route.push('/');
    }
    route
}
