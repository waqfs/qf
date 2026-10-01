use std::{cmp::Ordering, collections::BTreeMap, path::Path, process::exit, usize};

use crate::{
    compiler::model::{
        document::Site,
        site::{Block, Document, DocumentType, Inline, LinkedIndex, MetadataFields},
    },
    config::Config,
    parser,
};

pub fn build_index(
    root: &Path,
    config: Config,
    mut documents: Vec<Document>,
) -> Result<Site, String> {
    let mut routes = BTreeMap::new();
    let mut articles = Vec::new();
    let mut projects = Vec::new();
    let mut article_index: Option<usize> = None;
    let mut project_index: Option<usize> = None;

    for (index, document) in documents.iter_mut().enumerate() {
        document.route = route(document, root, &config);
        if routes.insert(document.route.clone(), index).is_some() {
            return Err(format!(
                "Error indexing routes, duplicates not allowed: {}",
                document.route
            ));
        }

        if document.metadata.is_index {
            match document.metadata.doc_type {
                DocumentType::Article => article_index = Some(index),
                DocumentType::Project => project_index = Some(index),
                DocumentType::Page => {}
            }
        } else {
            match document.metadata.doc_type {
                DocumentType::Article => articles.push(index),
                DocumentType::Project => projects.push(index),
                DocumentType::Page => {}
            }
        }
    }

    Ok(Site {
        config,
        documents,
        routes,
        articles,
        projects,
        article_index,
        project_index,
    })
}

pub fn process_index(site: &mut Site) {
    site.articles.sort_by(|a, b| {
        parser::date::sort(
            site.documents[*a].metadata.date,
            site.documents[*b].metadata.date,
        )
    });
    site.projects.sort_by(|a, b| {
        parser::date::sort(
            site.documents[*a].metadata.date,
            site.documents[*b].metadata.date,
        )
    });

    let article_routes: Vec<String> = site
        .articles
        .iter()
        .map(|index| site.documents[*index].route.clone())
        .collect();

    let project_routes: Vec<String> = site
        .projects
        .iter()
        .map(|index| site.documents[*index].route.clone())
        .collect();

    for (pos, index) in site.articles.clone().into_iter().enumerate() {
        site.documents[index].next = article_routes.get(pos + 1).cloned();
        site.documents[index].all = site.article_index.map_or(None, |all_index| {
            Some(site.documents[all_index].route.clone())
        });
        site.documents[index].previous = pos.checked_sub(1).map(|p| article_routes[p].clone());
    }
    for (pos, index) in site.projects.clone().into_iter().enumerate() {
        site.documents[index].next = project_routes.get(pos + 1).cloned();
        site.documents[index].all = site.project_index.map_or(None, |all_index| {
            Some(site.documents[all_index].route.clone())
        });
        site.documents[index].previous = pos.checked_sub(1).map(|p| project_routes[p].clone());
    }

    for index in 0..site.documents.len() {
        let blocks: Vec<(usize, Vec<Vec<Inline>>)> = site.documents[index]
            .blocks
            .iter()
            .enumerate()
            .filter_map(|(pos, block)| match block {
                Block::LinkedIndex(linked_index) => {
                    Some((pos, build_local_index(site, linked_index)))
                }
                _ => None,
            })
            .collect();
        for (pos, items) in blocks {
            if let Block::LinkedIndex(linked_index) = &mut site.documents[index].blocks[pos] {
                linked_index.items = items;
            }
        }
    }
}

fn build_local_index(site: &Site, index: &LinkedIndex) -> Vec<Vec<Inline>> {
    let indexed_documents = match index.doc_type {
        DocumentType::Article => &site.articles,
        DocumentType::Project => &site.projects,
        DocumentType::Page => return Vec::new(),
    };

    let mut documents: Vec<&Document> = indexed_documents
        .iter()
        .map(|index| &site.documents[*index])
        .filter(|document| {
            index
                .starred
                .map_or(true, |starred| document.metadata.starred == starred)
        })
        .collect();

    documents.sort_by(|a, b| {
        parser::date::sort(b.metadata.date, a.metadata.date).then_with(|| a.route.cmp(&b.route))
    });

    documents
        .into_iter()
        .take(index.limit.unwrap_or(usize::MAX))
        .map(|document| {
            let mut item = Vec::new();
            for field in &index.fields {
                let value: Option<Inline> = match field {
                    MetadataFields::Title => Some(Inline::Link {
                        label: vec![Inline::Text(document.metadata.title.clone())],
                        href: document.route.clone(),
                    }),
                    MetadataFields::Summary => document
                        .metadata
                        .summary
                        .as_ref()
                        .map(|v| Inline::Span(v.clone())),
                    MetadataFields::Date => document
                        .metadata
                        .date
                        .as_ref()
                        .map(|v| Inline::Span(v.to_rfc2822())),
                };
                if let Some(value) = value {
                    if !item.is_empty() {
                        item.push(Inline::Text(" - ".into()));
                    }
                    item.push(value);
                }
            }
            item
        })
        .collect()
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
