use ragit_fs::{basename, read_dir, read_string};
use regex::Regex;
use serde::{Deserialize, Serialize};
use shev::{
    Color,
    Entries,
    Entry,
    EntryFlag,
    EntryState,
    Graphic,
    TextBox,
    Transition,
};
use std::collections::HashMap;

fn main() {
    let test_results = std::env::args().collect::<Vec<_>>()[1].to_string();
    let mut entries_map = HashMap::new();
    let mut tests = vec![];
    let test_result_file_name = Regex::new(r"result\-([0-9a-f]{9})\-(.+)\.json").unwrap();

    for file in read_dir(&test_results, true).unwrap() {
        let file_name = basename(&file).unwrap();

        if let Some(c) = test_result_file_name.captures(&file) {
            let result: TestResult = serde_json::from_str(&read_string(&file).unwrap()).unwrap();
            let mut entries = vec![];

            for single_file_test in result.single_file_test.iter() {
                entries.push(Entry {
                    side_bar_title: single_file_test.name.to_string(),
                    top_bar_title: Some(single_file_test.name.to_string()),
                    content: Some(format!("# stdout\n\n```\n{}\n```\n\n# stderr\n\n```\n{}\n```", single_file_test.stdout, single_file_test.stderr)),
                    flag: if single_file_test.error.is_some() {
                        EntryFlag::Red
                    } else {
                        EntryFlag::Green
                    },
                    ..Entry::default()
                });
            }

            entries_map.insert(
                file_name.to_string(),
                Entries {
                    id: file_name.to_string(),
                    title: Some(file_name.to_string()),
                    entries,
                    transition: Some(Transition {
                        id: String::from("index"),
                        description: Some(String::from("go back to index")),
                    }),
                    render_canvas: render_single_file_test,
                },
            );
            tests.push(Entry {
                side_bar_title: file_name.to_string(),
                top_bar_title: Some(file_name.to_string()),
                transition1: Some(Transition {
                    id: file_name.to_string(),
                    description: Some(String::from("See details")),
                }),
                ..Entry::default()
            });
        }
    }

    entries_map.insert(
        String::from("index"),
        Entries {
            id: String::from("index"),
            title: Some(String::from("Tests")),
            entries: tests,
            transition: None,
            render_canvas: |_, _| Ok(vec![]),
        },
    );
    shev::run(shev::Config::default(), entries_map, String::from("index"))
}

#[derive(Deserialize, Serialize)]
pub struct TestResult {
    meta: HashMap<String, String>,

    #[serde(rename = "crate-test")]
    crate_test: Vec<CrateTest>,

    #[serde(rename = "single-file-test")]
    single_file_test: Vec<SingleFileTest>,
}

#[derive(Deserialize, Serialize)]
pub struct CrateTest {
    name: String,
    debug: CrateTestResult,
    release: CrateTestResult,
    doc: CrateTestResult,
}

#[derive(Deserialize, Serialize)]
pub struct CrateTestResult {
    error: Option<String>,
    elapsed: u32,
}

#[derive(Deserialize, Serialize)]
pub struct SingleFileTest {
    name: String,
    error: Option<String>,
    stdout: String,
    stderr: String,
    hash: String,
}

fn render_single_file_test(e: &Entry, _es: EntryState) -> Result<Vec<Graphic>, String> {
    let (s, colors) = apply_ansi_term_color(e.content.as_ref().unwrap());
    Ok(TextBox::new(
        &s,
        16.0,
        Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
        [20.0, 20.0, 800.0, 2000.0],
    ).with_color_map(colors).render())
}

#[derive(Clone, Copy)]
enum TermColorParseState {
    Text,
    Control,
}

fn apply_ansi_term_color(s: &str) -> (String, Vec<Color>) {
    let mut chars: Vec<char> = vec![];
    let mut colors = vec![];
    let mut curr_color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    let mut state = TermColorParseState::Text;
    let mut digits_buffer = vec![];

    for ch in s.chars() {
        match state {
            TermColorParseState::Text => match ch {
                '\u{1b}' => {
                    digits_buffer = vec![];
                    state = TermColorParseState::Control;
                },
                _ => {
                    chars.push(ch);
                    colors.push(curr_color);
                },
            },
            TermColorParseState::Control => match ch {
                '0'..='9' => {
                    digits_buffer.push(ch);
                },
                'm' => {
                    match digits_buffer.iter().collect::<String>().parse::<u32>() {
                        Ok(0) => {
                            curr_color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
                        },
                        Ok(31) => {
                            curr_color = Color { r: 0.8, g: 0.2, b: 0.2, a: 1.0 };
                        },
                        Ok(32) => {
                            curr_color = Color { r: 0.2, g: 0.8, b: 0.2, a: 1.0 };
                        },
                        Ok(33) => {
                            curr_color = Color { r: 0.8, g: 0.8, b: 0.2, a: 1.0 };
                        },
                        Ok(34) => {
                            curr_color = Color { r: 0.2, g: 0.2, b: 0.8, a: 1.0 };
                        },
                        _ => {},
                    };

                    state = TermColorParseState::Text;
                },
                _ => {},
            },
        }
    }

    (chars.iter().collect(), colors)
}
