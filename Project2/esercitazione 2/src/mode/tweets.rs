use crate::utils::prompting::select_tweet_file;
use std::ffi::OsStr;
use std::fs;

const TWEET_DIR: &str = "rsrc/tweets";

fn get_tweet_files() -> Vec<std::fs::DirEntry> {
    let paths = fs::read_dir(TWEET_DIR).expect("Unable to read directory");

    paths
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension() == Some(OsStr::new("csv")))
        .collect()
}

fn get_tweet_choices(tweet_files: &[std::fs::DirEntry]) -> Vec<String> {
    tweet_files
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

fn find_selected_file<'a>(
    tweet_files: &'a [std::fs::DirEntry],
    selected_person: &'a str,
) -> Option<&'a std::fs::DirEntry> {
    tweet_files
        .iter()
        .find(|entry| entry.path().file_stem().unwrap().to_str().unwrap() == selected_person)
}

fn read_tweet_content(file_path: &std::path::Path) -> String {
    let mut rdr = csv::Reader::from_path(file_path).expect("Unable to read CSV file");
    let mut all_tweets = String::new();

    let headers = rdr.headers().expect("Unable to read headers").clone();
    let header_vec: Vec<&str> = headers.iter().collect();

    let content_column_index = if header_vec.contains(&"text") {
        header_vec.iter().position(|&h| h == "text").unwrap()
    } else if header_vec.contains(&"content") {
        header_vec.iter().position(|&h| h == "content").unwrap()
    } else {
        panic!("CSV file does not contain 'text' or 'content' column");
    };

    for result in rdr.records() {
        let record = result.expect("Invalid record");

        if let Some(content) = record.get(content_column_index) {
            all_tweets.push_str(content);
            all_tweets.push(' ');
        }
    }

    all_tweets
}

pub fn read_tweets() -> String {
    let tweet_files = get_tweet_files();
    let tweet_choices = get_tweet_choices(&tweet_files);
    let selected_person = select_tweet_file(tweet_choices);

    if let Some(selected_file) = find_selected_file(&tweet_files, &selected_person) {
        let file_path = selected_file.path();
        read_tweet_content(&file_path)
    } else {
        panic!("No tweet CSV files found")
    }
}
