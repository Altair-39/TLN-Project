mod cky;
mod grammar;
use cky::cky_parse_jurafsky;
use cky::cky_parse_quenya;
use crossterm::{
    cursor,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use grammar::{CfgJurafsky, CfgQuenya};
use inquire::{Confirm, Select, Text};
use std::io::{self};

fn main() {
    loop {
        let name = Select::new(
            "What grammar do you want to use?",
            vec!["Jurafsky", "Eldar", "Quit"],
        )
        .with_help_message("chose 'Quit' to quit the program.")
        .prompt();

        io::stdout()
            .execute(terminal::Clear(ClearType::All))
            .unwrap();
        io::stdout().execute(cursor::MoveTo(0, 0)).unwrap();

        match name {
            Ok(ref name) if *name == "Quit" => {
                let ans = Confirm::new("Do you really want to quit?")
                    .with_default(false)
                    .prompt();
                match ans {
                    Ok(true) => break,
                    Ok(false) => {}
                    Err(_) => println!("An error happened.\n"),
                }
            }
            Ok(ref name) if *name == "Jurafsky" => {
                let grammar = CfgJurafsky::new();
                loop {
                    let sentence = Text::new("Please, insert a phrase that you want to check.\n")
                        .with_help_message("Type 'Quit' to return to the choice of the grammar.")
                        .prompt();
                    match sentence {
                        Ok(ref sentence) if sentence == "Quit" => break,
                        Ok(sentence) => {
                            if cky_parse_jurafsky(&sentence, &grammar) {
                                println!("'{}' is grammatically valid!\n", sentence);
                            } else {
                                println!("'{}' is NOT grammatically valid.\n", sentence);
                            }
                        }
                        Err(_) => println!("An error happened.\n"),
                    }
                }
            }
            Ok(ref name) if *name == "Eldar" => {
                let grammar = CfgQuenya::new();
                loop {
                    let sentence = Text::new("Please, insert a phrase that you want to check.\n")
                        .with_help_message("Type 'Quit' to return to the choice of the grammar.")
                        .prompt();
                    match sentence {
                        Ok(ref sentence) if sentence == "Quit" => break,
                        Ok(sentence) => {
                            if cky_parse_quenya(&sentence, &grammar) {
                                println!("'{}' is grammatically valid!\n", sentence);
                            } else {
                                println!("'{}' is NOT grammatically valid.\n", sentence);
                            }
                        }
                        Err(_) => println!("An error happened.\n"),
                    }
                }
            }
            Ok(_) => {
                println!("Invalid input, please select either 'Jurafsky', 'Eldar' or 'Quit.");
            }
            Err(_) => {
                println!(
                    "An error occurred when asking for your grammar selection, try again later."
                );
            }
        }
    }
}
