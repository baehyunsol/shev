use ragit_fs::{
    basename,
    is_dir,
    read_bytes,
    read_dir,
};
use shev::{
    Color,
    Config,
    Entries,
    Entry,
    EntryFlag,
    Graphic,
    TextBox,
    Transition,
};
use std::collections::HashMap;

fn main() {
    let mut entries_map = HashMap::new();
    std::env::set_current_dir("../..").unwrap();
    build_entries(&mut entries_map, "./", None);
    shev::run(Config::default(), entries_map, String::from("./"));
}

fn build_entries(entries_map: &mut HashMap<String, Entries>, path: &str, parent: Option<&str>) {
    let mut entries = vec![];

    for file in read_dir(path, true).unwrap() {
        if is_dir(&file) {
            build_entries(entries_map, &file, Some(path));
            entries.push(Entry {
                side_bar_title: basename(&file).unwrap(),
                top_bar_title: Some(file.to_string()),
                content: None,
                extra_content: None,
                category1: None,
                category2: None,
                transition1: Some(Transition {
                    id: file.to_string(),
                    description: Some(String::from("change directory")),
                }),
                transition2: None,
                flag: EntryFlag::Green,
            });
        }

        else {
            entries.push(Entry {
                side_bar_title: basename(&file).unwrap(),
                top_bar_title: Some(file.to_string()),
                content: None,
                extra_content: None,
                category1: None,
                category2: None,
                transition1: None,
                transition2: None,
                flag: EntryFlag::Blue,
            });
        }
    }

    entries_map.insert(
        path.to_string(),
        Entries {
            id: path.to_string(),
            title: Some(path.to_string()),
            entries,
            transition: parent.map(|p| Transition { id: p.to_string(), description: Some(String::from("move to parent directory")) }),
            render_canvas: |entry, _| match (entry.top_bar_title.as_ref().unwrap(), entry.flag) {
                (f, EntryFlag::Green) => {
                    let s = match read_dir(f, true) {
                        Ok(files) => format!(
                            "{} file{}\n{}",
                            files.len(),
                            if files.len() == 1 { "" } else { "s" },
                            files.join("\n"),
                        ),
                        Err(e) => format!("error: {e:?}"),
                    };
                    Ok(TextBox::new(
                        &s,
                        16.0,
                        Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
                        [20.0, 20.0, 2000.0, 2000.0],
                    ).render())
                },
                (f, _) if f.ends_with(".png") => Ok(vec![Graphic::ImageFile {
                    path: f.to_string(),
                    x: 0.0,
                    y: 0.0,
                    w: 900.0,
                    h: 600.0,
                }]),
                (f, _) => {
                    let s = match read_bytes(f) {
                        Ok(b) => match String::from_utf8(b) {
                            Ok(s) => s,
                            Err(_) => String::from("<BINARY FILE>"),
                        },
                        Err(e) => format!("error: {e:?}"),
                    };
                    Ok(TextBox::new(
                        &s,
                        16.0,
                        Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
                        [20.0, 20.0, 2000.0, 2000.0],
                    ).render())
                },
            },
        },
    );
}
