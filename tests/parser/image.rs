use std::assert_eq;

use qf::{
    compiler::model::site::{Block, Image, ImageAlt},
    error::QFError,
    parser::block::parse,
};

fn lines(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn assert_err_str(blocks: Result<Vec<Block>, QFError>, error: &str) {
    let parse_error = blocks.expect_err("expected parse to fail").to_string();
    assert!(
        parse_error.ends_with(error),
        "expected parse error to be {error}, got {parse_error}"
    );
}
fn assert_image(blocks: Vec<Block>) -> Image {
    assert!(
        blocks.len() == 1,
        "expected 1 block parsed, got {}",
        blocks.len()
    );
    let block = blocks[0].to_owned();
    match block {
        Block::Image(image) => image,
        _ => panic!("expected block to be an Image"),
    }
}
fn assert_source(image: &Image, src: &str) {
    assert_eq!(image.source, src, "expected image source to be {src}");
}
fn assert_alt(image: &Image, alt: &str) {
    match &image.alt {
        ImageAlt::Decorative => panic!("expected image to have alt text, got decorative"),
        ImageAlt::Description(desc) => {
            assert_eq!(desc, alt, "expected image alt to be {alt}, got {desc}")
        }
    }
}
fn assert_decorative(image: &Image) {
    assert!(
        matches!(image.alt, ImageAlt::Decorative),
        "expected image to be decorative"
    );
}
fn assert_caption(image: &Image, caption: Option<String>) {
    match &image.caption {
        Some(actual_caption) => match &caption {
            Some(caption) => {
                assert_eq!(
                    actual_caption, caption,
                    "expected caption to be {caption}, got {actual_caption}"
                );
            }
            None => {
                panic!("expected no caption, got {actual_caption}");
            }
        },
        None => {
            if let Some(caption) = caption {
                panic!("expected caption to be {caption}, got none");
            }
        }
    }
}

enum S {
    Alt,
    Decorative,
    Caption,
}

const IMAGE_SOURCE: &str = "example.png";
const IMAGE_ALT: &str = "alt text";
const IMAGE_CAPTION: &str = "caption text";

fn synth(opts: &Vec<S>) -> Result<Vec<Block>, QFError> {
    let header = format!("@image {}\n", &IMAGE_SOURCE);
    let alt = format!("alt=\"{}\"\n", &IMAGE_ALT);
    let caption = format!("caption=\"{}\"\n", &IMAGE_CAPTION);
    let mut lines: Vec<&str> = vec![header.as_str()];
    for opt in opts {
        match opt {
            S::Alt => lines.push(alt.as_str()),
            S::Decorative => lines.push("decorative\n"),
            S::Caption => lines.push(caption.as_str()),
        }
    }
    parse(&lines)
}

#[test]
fn decorative() {
    let blocks = synth(&vec![S::Decorative]);
    assert!(blocks.is_ok(), "expected parse to succeed");

    let image = assert_image(blocks.unwrap());
    assert_source(&image, "example.png");
    assert_decorative(&image);
    assert_caption(&image, None);
}

#[test]
fn decorative_caption() {
    let blocks = synth(&vec![S::Decorative, S::Caption]);
    assert_err_str(
        blocks,
        "Image already marked decorative and cannot have a caption",
    );
}

#[test]
fn caption_decorative() {
    let blocks = synth(&vec![S::Caption, S::Decorative]);
    assert_err_str(
        blocks,
        "Image already has a caption and cannot be marked decorative",
    );
}
