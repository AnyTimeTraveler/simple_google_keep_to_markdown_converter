use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct Note {
    color: String,
    #[serde(rename = "isTrashed")]
    is_trashed: bool,
    #[serde(rename = "isPinned")]
    is_pinned: bool,
    #[serde(rename = "isArchived")]
    is_archived: bool,
    #[serde(rename = "listContent")]
    list_content: Option<Vec<ListItem>>,
    #[serde(rename = "textContentHtml")]
    text_content_html: Option<String>,
    title: String,
    #[serde(rename = "userEditedTimestampUsec")]
    user_edited_timestamp_usec: u64,
    #[serde(rename = "createdTimestampUsec")]
    created_timestamp_usec: u64,
    labels: Option<Vec<Label>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Label {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ListItem {
    #[serde(rename = "isChecked")]
    is_checked: bool,
    #[serde(rename = "textHtml")]
    text_html: String,
    text: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    for dir_entry in Path::new("Takeout/Keep").read_dir()? {
        let dir_entry = dir_entry?;
        if dir_entry.file_name().to_string_lossy().ends_with(".json") {
            let file = OpenOptions::new()
                .read(true)
                .open(&dir_entry.path().as_path())?;
            let parsed: Note = serde_json::from_reader(file).map_err(|error| {
                eprintln!("{:?} {:?}", dir_entry.file_name(), error);
                error
            })?;

            let note = format_note(dir_entry.file_name().to_string_lossy(), parsed);
            let mut md_file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&dir_entry.path().with_extension("md").as_path())?;

            md_file.write_all(note.as_bytes())?;
        }
    }

    Ok(())
}

fn format_note(filename: Cow<str>, x: Note) -> String {
    let mut out = String::new();
    if x.title.is_empty() {
        out.push_str(&format!("# {}\n", filename));
    } else {
        out.push_str(&format!("# {}\n", x.title));
    }
    let mut has_tag = false;
    if x.is_pinned {
        out.push_str("#keep/pinned ");
        has_tag = true;
    }
    if x.is_trashed {
        out.push_str("#keep/trashed ");
        has_tag = true;
    }
    if x.is_archived {
        out.push_str("#keep/archived ");
        has_tag = true;
    }
    if !has_tag {
        out.push_str("#keep/active");
    }
    out.push_str("\n");

    if let Some(list) = x.list_content {
        out.push_str("\n");
        for item in list {
            let checked = if item.is_checked { "X" } else { " " };
            out.push_str(&format!(" - [{}] {}\n", checked, item.text));
        }
        out.push_str("\n");
    }
    if let Some(html) = x.text_content_html {
        let string = html2md::parse_html(&html);
        out.push_str(&format!("{}\n", string));
    }
    out
}
