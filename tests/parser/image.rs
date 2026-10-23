use std::assert_eq;

use qf::{
    compiler::model::site::{Block, Image, ImageAlt},
    parser::block::parse,
};

fn lines(input: &str) -> Vec<&str> {
    input.lines().collect()
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

#[test]
fn valid_decorative() {
    let blocks = parse(&lines(include_str!("fixture/image_valid_decorative.txt")));
    assert!(blocks.is_ok(), "expected parse to succeed");

    let image = assert_image(blocks.unwrap());
    assert_source(&image, "example.png");
    assert_decorative(&image);
    assert_caption(&image, None);
}
