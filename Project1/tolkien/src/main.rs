mod cky;
mod grammar;
use cky::cky_parse;
use grammar::Cfg;
use inquire::{Confirm, Select, Text};
use std::fs;
use std::io::{self};
use std::path::Path;

fn clean() {
    println!("\x1b[2J\x1b[H");
}

fn pause() {
    println!("Press any key to continue...");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {}
        Err(error) => println!("error: {error}"),
    }
    clean();
}

fn main() {
    loop {
        let grammar_dir = "rsrc/grammar";
        clean();
        println!("╔════════════════════════════╗");
        println!("║     Choose a Grammar       ║");
        println!("╚════════════════════════════╝");

        let grammar_options: Vec<String> = fs::read_dir(grammar_dir)
            .expect("Failed to read grammar directory")
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    let path = e.path();
                    if path.extension()?.to_str()? == "json" {
                        path.file_stem()?.to_str().map(|s| s.to_string())
                    } else {
                        None
                    }
                })
            })
            .collect();

        let mut options = grammar_options.clone();
        options.sort();
        options.push("Delete".to_string());
        options.push("New".to_string());
        options.push("Quit".to_string());

        let name = Select::new("What grammar do you want to use?", options)
            .with_help_message("Choose 'New' to add a new grammar, 'Delete' to delete a grammar or 'Quit' to quit the program.")
            .prompt();
        clean();

        match name {
            Ok(ref name) if *name == "New" => {
                let sentence = Text::new("Insert the path to the JSON file you want to add.\n")
                    .with_help_message("Type 'Quit' to return to the choice of the grammar.")
                    .prompt();
                match sentence {
                    Ok(ref sentence) if sentence == "Quit" => {
                        clean();
                        break;
                    }
                    Ok(sentence) => {
                        let source_path = Path::new(&sentence);
                        let grammar_dir = Path::new("rsrc/grammar");

                        if source_path.exists()
                            && source_path.extension().and_then(|ext| ext.to_str()) == Some("json")
                        {
                            let file_name = source_path.file_name().unwrap();
                            let destination_path = grammar_dir.join(file_name);
                            match fs::copy(source_path, &destination_path) {
                                Ok(_) => {
                                    println!("Successfully added the grammar file.");
                                }
                                Err(e) => {
                                    println!("Failed to copy file: {}", e);
                                }
                            }
                            pause();
                        } else {
                            println!("Invalid file. Ensure the file exists and is a JSON file.");
                            pause();
                        }
                    }
                    Err(_) => {
                        println!("An error happened.\n");
                        pause();
                    }
                }
            }
            Ok(ref name) if *name == "Delete" => {
                let grammar_files: Vec<String> = fs::read_dir(grammar_dir)
                    .expect("Failed to read grammar directory")
                    .filter_map(|entry| {
                        entry.ok().and_then(|e| {
                            let path = e.path();
                            if path.extension()?.to_str()? == "json" {
                                path.file_name()?.to_str().map(String::from)
                            } else {
                                None
                            }
                        })
                    })
                    .collect();

                if grammar_files.is_empty() {
                    println!("No grammar files available to delete.");
                    pause();
                }

                let file_to_delete =
                    Select::new("Select a grammar file to delete:", grammar_files.clone()).prompt();

                if let Ok(filename) = file_to_delete {
                    let file_path = Path::new(grammar_dir).join(&filename);
                    let confirm =
                        Confirm::new(&format!("Are you sure you want to delete {}?", filename))
                            .prompt();

                    if let Ok(true) = confirm {
                        if fs::remove_file(&file_path).is_ok() {
                            println!("Deleted {}", filename);
                        } else {
                            println!("Failed to delete {}", filename);
                        }
                    }
                }
                pause();
            }
            Ok(ref name) if *name == "Quit" => {
                let ans = Confirm::new("Do you really want to quit?")
                    .with_default(false)
                    .prompt();
                match ans {
                    Ok(true) => {
                        clean();
                        break;
                    }
                    Ok(false) => {}
                    Err(_) => {
                        println!("An error happened.\n");
                        pause();
                    }
                }
            }
            Ok(name) => {
                let grammar = Cfg::new(&name);
                loop {
                    let sentence = Text::new("Please, insert a phrase that you want to check.\n")
                        .with_help_message("Type 'Quit' to return to the choice of the grammar.")
                        .prompt();
                    match sentence {
                        Ok(ref sentence) if sentence == "Quit" => {
                            clean();
                            break;
                        }
                        Ok(sentence) => {
                            if cky_parse(&sentence, &grammar, "output.json") {
                                println!("'{}' is grammatically valid!\n", sentence);
                            } else {
                                println!("'{}' is NOT grammatically valid.\n", sentence);
                            }
                            pause();
                        }
                        Err(_) => {
                            println!("An error happened.\n");
                            pause();
                        }
                    }
                }
            }
            Err(_) => {
                println!(
                    "An error occurred when asking for your grammar selection, try again later."
                );
            }
        }
    }
}
