use std::fs;

use crate::utils::prompting::select_tweet_file;

pub fn read_tweets() -> String {
    let tweet_dir = "rsrc/tweets";
    let paths = fs::read_dir(tweet_dir).expect("Unable to read directory");

    let tweet_files: Vec<_> = paths
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension() == Some(std::ffi::OsStr::new("csv")))
        .collect();

    let tweet_choices: Vec<_> = tweet_files
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

    let selected_person = select_tweet_file(tweet_choices);

    if let Some(selected_file) = tweet_files
        .iter()
        .find(|entry| entry.path().file_stem().unwrap().to_str().unwrap() == selected_person)
    {
        let file_path = selected_file.path();
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
    } else {
        panic!("No tweet CSV files found")
    }
}
