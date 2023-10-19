use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path};
use tree_magic;
use prettytable::{Table, Row, Cell, row, format};
use clap::{Arg, Command};

// TODO: USE https://github.com/garyboone/incr_stats

fn main() {
    let matches = Command::new("ExtCount")
        .version("0.1.0")
        .author("Your Name <your.email@example.com>")
        .about("Counts files by extension and displays MIME types")
        .arg(Arg::new("DIRECTORY")
            .help("Sets the working directory. Defaults to current directory if not set.")
            .default_value(".")
            .index(1))
        .get_matches();

    //let working_dir = matches.value_of("DIRECTORY").unwrap_or(".");
    let working_dir = matches.get_one::<String>("DIRECTORY").unwrap();//.unwrap_or(".");

    // Recursively enumerate files in the specified directory
    let entries = fs::read_dir(&working_dir).expect("Failed to read directory");
    let mut extension_counts: HashMap<String, u32> = HashMap::new();
    let mut extension_to_mime: HashMap<String, HashSet<String>> = HashMap::new();

    for entry in entries {
        if let Ok(entry) = entry {
            process_entry(&entry.path(), &mut extension_counts, &mut extension_to_mime);
        }
    }

    // Create and print the table using prettytable
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(row!["Extension", "Count", "MIME Types"]);

    for (ext, count) in &extension_counts {
        let mime_str = match extension_to_mime.get(ext) {
            Some(mime_set) => format!("{:?}", mime_set),
            None => format!("{:?}", &HashSet::<String>::new()),
        };
        table.add_row(Row::new(vec![
            Cell::new(ext),
            Cell::new(&count.to_string()),
            Cell::new(&mime_str),
        ]));
    }

    table.printstd();
}

fn process_entry(path: &Path, counts: &mut HashMap<String, u32>, mime_map: &mut HashMap<String, HashSet<String>>) {
    // If it's a directory, recursively process its content
    if path.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    process_entry(&entry.path(), counts, mime_map);
                }
            }
        }
    } else if let Some(extension) = path.extension() {
        // If it's a file, count its extension
        let ext_str = format!(".{}", extension.to_string_lossy());
        *counts.entry(ext_str.clone()).or_insert(0) += 1;

        // Determine MIME type using tree_magic
        let mime = tree_magic::from_filepath(path);
        mime_map.entry(ext_str).or_insert_with(HashSet::new).insert(mime);
    }
}

