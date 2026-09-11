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

pub fn sort_index(site: &mut Site) {
    site.articles.sort_by(|a, b| {
        let first_date = site.documents[*a].metadata.date.as_deref().unwrap_or("");
        let second_date = site.documents[*b].metadata.date.as_deref().unwrap_or("");
        first_date.cmp(second_date)
    });

    let routes: Vec<String> = site
        .articles
        .iter()
        .map(|index| site.documents[*index].route.clone())
        .collect();

    for (pos, index) in site.articles.clone().into_iter().enumerate() {
        site.documents[index].previous = pos.checked_sub(1).map(|p| routes[p].clone());
        site.documents[index].next = routes.get(pos + 1).cloned();
    }
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
