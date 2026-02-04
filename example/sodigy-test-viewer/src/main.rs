use ragit_fs::{
    basename,
    join,
    read_dir,
    read_string,
};
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
    let test_results_at = std::env::args().collect::<Vec<_>>()[1].to_string();
    let blobs_at = join(&test_results_at, ".index/blobs").unwrap();
    let blobs = load_blobs(&blobs_at);
    let mut entries_map = HashMap::new();
    let mut tests = vec![];
    let test_result_file_name = Regex::new(r"result\-([0-9a-f]{9})\-(.+)\.json").unwrap();

    for file in read_dir(&test_results_at, true).unwrap() {
        let file_name = basename(&file).unwrap();

        if test_result_file_name.is_match(&file) {
            let result: TestResult = serde_json::from_str(&read_string(&file).unwrap()).unwrap();
            let mut entries = vec![];
            tests.push(Entry {
                side_bar_title: file_name.to_string(),
                top_bar_title: Some(file_name.to_string()),
                content: Some(serde_json::to_string(&result).unwrap()),
                transition1: Some(Transition {
                    id: file_name.to_string(),
                    description: Some(String::from("See details")),
                }),
                ..Entry::default()
            });

            for mut single_file_test in result.single_file_test.into_iter() {
                // Some old results have a trailing newline character
                single_file_test.hash = match blobs.get(single_file_test.hash.trim()) {
                    Some(blob) => blob.to_string(),
                    None => format!("Error: failed to load blob `{}`", single_file_test.hash.trim()),
                };

                entries.push(Entry {
                    side_bar_title: single_file_test.name.to_string(),
                    top_bar_title: Some(single_file_test.name.to_string()),
                    content: Some(serde_json::to_string(&single_file_test).unwrap()),
                    extra_content: Some(serde_json::to_string_pretty(&result.meta).unwrap()),
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
        }
    }

    entries_map.insert(
        String::from("index"),
        Entries {
            id: String::from("index"),
            title: Some(String::from("Tests")),
            entries: tests,
            transition: None,
            render_canvas: render_test_result,
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

fn render_test_result(e: &Entry, _: EntryState) -> Result<Vec<Graphic>, String> {
    let test_result: TestResult = serde_json::from_str(&e.content.as_ref().unwrap()).map_err(|e| format!("{e:?}"))?;
    let crate_test_success = test_result.crate_test.iter().filter(|t| t.debug.error.is_none() && t.release.error.is_none() && t.doc.error.is_none()).count();
    let crate_test_fail = test_result.crate_test.len() - crate_test_success;
    let single_file_test_success = test_result.single_file_test.iter().filter(|t| t.error.is_none()).count();
    let single_file_test_fail = test_result.single_file_test.len() - single_file_test_success;
    Ok(TextBox::new(
        &format!("
crate-test: {{ success: {crate_test_success}, fail: {crate_test_fail} }}
single-file-test: {{ success: {single_file_test_success}, fail: {single_file_test_fail} }}
meta: {}",
            serde_json::to_string_pretty(&test_result.meta).unwrap(),
        ),
        16.0,
        Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
        [20.0, 20.0, 800.0, 2000.0],
    ).render())
}

fn render_single_file_test(e: &Entry, es: EntryState) -> Result<Vec<Graphic>, String> {
    let test_result: SingleFileTest = serde_json::from_str(e.content.as_ref().unwrap()).map_err(|e| format!("{e:?}"))?;

    // The main function replaced `test_result.hash` with the file content.
    let file_content = test_result.hash.clone();

    let s = match es {
        EntryState::None | EntryState::Green => file_content,
        EntryState::Red | EntryState::Blue => format!("# stdout\n\n```\n{}\n```\n\n# stderr\n\n```\n{}\n```", test_result.stdout, test_result.stderr),
    };
    let (s, colors) = apply_ansi_term_color(&s);
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
                            curr_color = Color { r: 0.75, g: 0.25, b: 0.25, a: 1.0 };
                        },
                        Ok(32) => {
                            curr_color = Color { r: 0.25, g: 0.75, b: 0.25, a: 1.0 };
                        },
                        Ok(33) => {
                            curr_color = Color { r: 0.75, g: 0.75, b: 0.25, a: 1.0 };
                        },
                        Ok(34) => {
                            curr_color = Color { r: 0.25, g: 0.2, b: 0.75, a: 1.0 };
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

fn load_blobs(blobs_at: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();

    for pre_dir in read_dir(blobs_at, false).unwrap_or(vec![]) {
        let Ok(prefix) = basename(&pre_dir) else { continue; };

        for suffix_full in read_dir(&pre_dir, false).unwrap_or(vec![]) {
            let Ok(suffix) = basename(&suffix_full) else { continue; };
            let Ok(blob) = read_string(&suffix_full) else { continue; };
            result.insert(format!("{prefix}{suffix}"), blob);
        }
    }

    result
}
