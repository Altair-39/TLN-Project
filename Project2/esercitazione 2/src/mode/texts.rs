use crate::utils::prompting::select_book;
use std::ffi::OsStr;
use std::fs;

const TEXT_DIR: &str = "rsrc/texts";

fn get_text_files() -> Vec<std::fs::DirEntry> {
    let paths = fs::read_dir(TEXT_DIR).expect("Unable to read directory");

    paths
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension() == Some(OsStr::new("txt")))
        .collect()
}

fn get_book_choices(text_files: &[std::fs::DirEntry]) -> Vec<String> {
    text_files
        .iter()
        .map(|entry| {
            entry
                .path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        })
        .collect()
}

fn find_selected_book<'a>(
    text_files: &'a [std::fs::DirEntry],
    selected_book: &'a str,
) -> Option<&'a std::fs::DirEntry> {
    text_files
        .iter()
        .find(|entry| entry.path().file_stem().unwrap().to_str().unwrap() == selected_book)
}

fn read_text_content(file_path: &std::path::Path) -> String {
    fs::read_to_string(file_path).expect("Unable to read text file")
}

pub fn read_texts() -> String {
    let text_files = get_text_files();
    let book_choices = get_book_choices(&text_files);
    let selected_book = select_book(book_choices);

    if let Some(selected_path) = find_selected_book(&text_files, &selected_book) {
        let file_path = selected_path.path();
        read_text_content(&file_path)
    } else {
        panic!("Book not found")
    }
}
