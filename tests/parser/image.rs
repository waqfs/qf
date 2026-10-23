use std::assert_eq;

use qf::{
    compiler::model::site::{Block, Image, ImageAlt},
    error::QFError,
    parser::block::parse,
};

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

#[derive(Debug, PartialEq)]
enum S {
    Alt,
    Decorative,
    Caption,
}

const IMAGE_SOURCE: &str = "example.png";
const IMAGE_ALT: &str = "alt text";
const IMAGE_CAPTION: &str = "caption text";

fn synth(opts: &Vec<S>) -> Result<Vec<Block>, QFError> {
    let header = format!("@image {}\n", IMAGE_SOURCE);
    let alt = format!("alt=\"{}\"\n", IMAGE_ALT);
    let caption = format!("caption=\"{}\"\n", IMAGE_CAPTION);
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
fn valid() {
    let cases = [
        ("decorative", &vec![S::Decorative]),
        ("alt", &vec![S::Alt]),
        ("alt-caption", &vec![S::Alt, S::Caption]),
        ("caption-alt", &vec![S::Caption, S::Alt]),
    ];
    for (name, config) in cases {
        let blocks = synth(config);
        assert!(
            blocks.is_ok(),
            "expected fixture valid-{name} to succeed, got {blocks:?}"
        );
        let image = assert_image(blocks.unwrap());
        assert_eq!(
            image.source, IMAGE_SOURCE,
            "expected image source to be {}",
            IMAGE_SOURCE
        );

        if config.contains(&S::Decorative) {
            assert!(
                matches!(image.alt, ImageAlt::Decorative),
                "expected image to be decorative"
            );
        }
        if config.contains(&S::Alt) {
            match &image.alt {
                ImageAlt::Decorative => panic!("expected image to have alt text, got decorative"),
                ImageAlt::Description(desc) => {
                    assert_eq!(
                        desc, IMAGE_ALT,
                        "expected image alt to be {}, got {desc}",
                        IMAGE_ALT
                    );
                }
            }
        }
        if config.contains(&S::Caption) {
            assert_caption(&image, Some(IMAGE_CAPTION.to_string()));
        } else {
            assert_caption(&image, None);
        }
    }
}

#[test]
fn invalid() {
    let cases = [
        (
            "decorative-alt",
            &vec![S::Decorative, S::Alt],
            "Image already marked decorative and cannot have an alt description",
        ),
        (
            "alt-decorative",
            &vec![S::Alt, S::Decorative],
            "Image already has an alt description and cannot be marked decorative",
        ),
        (
            "decorative-caption",
            &vec![S::Decorative, S::Caption],
            "Image already marked decorative and cannot have a caption",
        ),
        (
            "caption-decorative",
            &vec![S::Caption, S::Decorative],
            "Image already has a caption and cannot be marked decorative",
        ),
        (
            "decorative-decorative",
            &vec![S::Decorative, S::Decorative],
            "Image should not be marked decorative multiple times",
        ),
        (
            "alt-alt",
            &vec![S::Alt, S::Alt],
            "Image cannot have multiple alt descriptions",
        ),
        (
            "caption-caption",
            &vec![S::Caption, S::Caption],
            "Image cannot have multiple captions",
        ),
        (
            "caption",
            &vec![S::Caption],
            "Image requires an alt property or be marked decorative",
        ),
        (
            "none",
            &vec![],
            "Image requires an alt property or be marked decorative",
        ),
    ];
    for (name, config, error) in cases {
        let blocks = synth(config);
        let parse_error = blocks.expect_err("expected parse to fail").to_string();
        assert!(
            parse_error.ends_with(error),
            "expected fixture invalid-{name} to fail with error {error}, got {parse_error}"
        );
    }
}
