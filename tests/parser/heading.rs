use std::assert_eq;

use qf::{
    compiler::model::site::{Block, Inline},
    error::QFError,
    parser::block::parse,
};

struct HeadingValues {
    level: u8,
    content: Vec<Inline>,
}

fn assert_heading(blocks: Vec<Block>, level: u8) -> HeadingValues {
    assert!(
        blocks.len() == level as usize,
        "expected 1 block parsed, got {}",
        blocks.len()
    );
    let block = blocks[level as usize - 1].to_owned();
    match block {
        Block::Heading { level, content } => HeadingValues { level, content },
        _ => panic!("expected block to be a Heading"),
    }
}

const HEADING_TEXT: &str = "heading text";

fn synth(level: u8) -> Result<Vec<Block>, QFError> {
    let mut lines: Vec<String> = vec![];
    for level in 1..level + 1 {
        lines.push(format!(
            "{} {}\n\n",
            "#".repeat(level as usize),
            HEADING_TEXT
        ));
    }
    let lines: Vec<&str> = lines.iter().map(String::as_str).collect();
    parse(&lines)
}

fn synth_skip(start: u8, end: u8) -> Result<Vec<Block>, QFError> {
    let mut lines: Vec<String> = vec![];
    for level in 1..start + 1 {
        lines.push(format!(
            "{} {}\n\n",
            "#".repeat(level as usize),
            HEADING_TEXT
        ));
    }
    lines.push(format!("{} {}\n", "#".repeat(end as usize), HEADING_TEXT));
    let lines: Vec<&str> = lines.iter().map(String::as_str).collect();
    parse(&lines)
}

#[test]
fn valid_header_levels() {
    let cases = [1, 2, 3, 4, 5];
    for expected_level in cases {
        let blocks = synth(expected_level);
        assert!(
            blocks.is_ok(),
            "expected fixture-{expected_level} to succeed, got {blocks:?}"
        );
        let heading = assert_heading(blocks.unwrap(), expected_level);
        assert_eq!(
            heading.level, expected_level,
            "expected fixture-{expected_level} level to be {}, got {}",
            expected_level, heading.level
        );
        assert!(
            heading.content.len() == 1,
            "expected fixture-{expected_level} content to include a single block, got {}",
            heading.content.len()
        );
        let inline = &heading.content[0];
        match inline {
            Inline::Text(text) => assert_eq!(text, HEADING_TEXT),
            _ => panic!("expected fixture-{expected_level} inline text block, got {inline:?}"),
        }
    }
}

#[test]
fn invalid_header_level() {
    let blocks = synth(6);
    assert!(
        blocks.is_err(),
        "expected fixture-6 to fail, got {blocks:?}"
    );
    let error = blocks.unwrap_err();
    let expected_error = "Maximum of 5 hashes and a space required for headers";
    assert!(
        error.to_string().ends_with(expected_error),
        "expected fixture-6 error to end with {expected_error}, got {error:?}"
    );
}

#[test]
fn valid_header_skips() {
    let cases = [(5, 4), (4, 3), (3, 2), (2, 1)];
    for (start, end) in cases {
        let blocks = synth_skip(start, end);
        assert!(
            blocks.is_ok(),
            "expected fixture-{}-{} to succeed, got {blocks:?}",
            start,
            end
        );
    }
}

#[test]
fn invalid_header_skips() {
    let cases = [
        (
            1,
            3,
            "Headers cannot skip levels, expected level 2 but found level 3",
        ),
        (
            2,
            4,
            "Headers cannot skip levels, expected level 3 but found level 4",
        ),
        (
            3,
            5,
            "Headers cannot skip levels, expected level 4 but found level 5",
        ),
    ];
    for (start, end, expected_error) in cases {
        let blocks = synth_skip(start, end);
        assert!(
            blocks.is_err(),
            "expected fixture-{}-{} to fail, got {blocks:?}",
            start,
            end
        );
        let error = blocks.unwrap_err();
        assert!(
            error.to_string().ends_with(expected_error),
            "expected fixture-{}-{} error to end with {expected_error}, got {error:?}",
            start,
            end
        );
    }
}
