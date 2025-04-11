use std::fs;

use crate::utils::prompting::select_book;

pub fn read_texts() -> String {
    let text_dir = "rsrc/texts";
    let paths = fs::read_dir(text_dir).expect("Unable to read directory");

    let text_files: Vec<_> = paths
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension() == Some(std::ffi::OsStr::new("txt")))
        .collect();

    let book_choices: Vec<_> = text_files
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
        .collect();

    let selected_book = select_book(book_choices);

    if let Some(selected_path) = text_files
        .iter()
        .find(|entry| entry.path().file_stem().unwrap().to_str().unwrap() == selected_book)
    {
        let file_path = selected_path.path();
        fs::read_to_string(file_path).expect("Unable to read text file")
    } else {
        panic!("Book not found")
    }
}
