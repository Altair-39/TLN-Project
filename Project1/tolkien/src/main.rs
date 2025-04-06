mod cky;
mod grammar;

use cky::cky_parse;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use grammar::Cfg;
use grammarconversion::converter;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};
use std::{fs, io, path::Path, time::Duration};

#[derive(Debug, Clone)]
struct AppState {
    grammar_options: Vec<String>,
    list_state: ListState,
    mode: Mode,
    input: String,
    message: String,
    message_timer: u8,
    current_grammar_rules: String,
    scroll_offset: u16,
}

#[derive(Debug, Clone, PartialEq)]
enum Mode {
    MainMenu,
    ParseSentence(String),
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let grammar_dir = "rsrc/grammar";
    let grammar_options = get_grammar_options(grammar_dir)?;
    let mut app_state = AppState {
        grammar_options,
        list_state: ListState::default().with_selected(Some(0)),
        mode: Mode::MainMenu,
        input: String::new(),
        message: String::new(),
        message_timer: 0,
        current_grammar_rules: String::new(),
        scroll_offset: 0,
    };

    let mut running = true;
    while running {
        terminal.draw(|f| ui(f, &mut app_state))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                let grammar_name = match &app_state.mode {
                    Mode::ParseSentence(name) => Some(name.clone()),
                    _ => None,
                };

                match app_state.mode {
                    Mode::MainMenu => {
                        handle_main_menu(&mut app_state, key_event.code, &mut running, grammar_dir)?
                    }
                    Mode::ParseSentence(_) => {
                        if let Some(name) = grammar_name {
                            handle_parse_sentence(
                                &mut app_state,
                                key_event.code,
                                &name,
                                grammar_dir,
                            )?
                        }
                    }
                }
            }
        }

        if app_state.message_timer > 0 {
            app_state.message_timer -= 1;
            if app_state.message_timer == 0 {
                app_state.message.clear();
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn handle_main_menu(
    app_state: &mut AppState,
    key: KeyCode,
    running: &mut bool,
    grammar_dir: &str,
) -> io::Result<()> {
    match key {
        KeyCode::Char('q') => *running = false,
        KeyCode::Up => {
            if let Some(selected) = app_state.list_state.selected() {
                if selected > 0 {
                    app_state.list_state.select(Some(selected - 1));
                }
            }
        }
        KeyCode::Down => {
            if let Some(selected) = app_state.list_state.selected() {
                if selected < app_state.grammar_options.len() + 1 {
                    app_state.list_state.select(Some(selected + 1));
                }
            }
        }
        KeyCode::Enter => {
            if let Some(selected) = app_state.list_state.selected() {
                let options_count = app_state.grammar_options.len();
                match selected {
                    x if x == options_count + 2 => *running = false,
                    _ => {
                        let grammar_name = app_state.grammar_options[selected].clone();
                        app_state.current_grammar_rules =
                            get_grammar_rules(&grammar_name, grammar_dir)
                                .unwrap_or_else(|_| "Could not load grammar rules".to_string());
                        app_state.mode = Mode::ParseSentence(grammar_name);
                        app_state.input.clear();
                        app_state.scroll_offset = 0;
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}

fn handle_parse_sentence(
    app_state: &mut AppState,
    key: KeyCode,
    grammar_name: &str,
    grammar_dir: &str,
) -> io::Result<()> {
    match key {
        KeyCode::Up => {
            app_state.scroll_offset = app_state.scroll_offset.saturating_sub(1);
        }
        KeyCode::Down => {
            app_state.scroll_offset = app_state.scroll_offset.saturating_add(1);
        }
        KeyCode::Enter => {
            let sentence = app_state.input.trim();
            if sentence == "Quit" {
                app_state.mode = Mode::MainMenu;
                app_state.input.clear();
            } else {
                let txt_path = Path::new(grammar_dir).join(format!("{}.txt", grammar_name));
                let json_path = txt_path.with_extension("json");

                if !json_path.exists() {
                    converter(txt_path.to_str().unwrap())?;
                }

                let grammar = Cfg::new(grammar_name);
                let result = cky_parse(sentence, &grammar, "output.json");
                app_state.message = if result.is_some() {
                    "✅ Valid grammar!".to_string()
                } else {
                    "❌ Invalid grammar!".to_string()
                };
                app_state.message_timer = 20;
                app_state.input.clear();
            }
        }
        KeyCode::Char(c) => {
            app_state.input.push(c);
        }
        KeyCode::Backspace => {
            app_state.input.pop();
        }
        KeyCode::Esc => {
            app_state.mode = Mode::MainMenu;
            app_state.input.clear();
        }
        _ => {}
    }
    Ok(())
}

fn get_grammar_options(grammar_dir: &str) -> io::Result<Vec<String>> {
    let mut options: Vec<String> = fs::read_dir(grammar_dir)?
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.extension()?.to_str()? == "txt" {
                path.file_stem()?.to_str().map(|s| s.to_string())
            } else {
                None
            }
        })
        .collect();

    options.sort();
    Ok(options)
}

fn get_grammar_rules(grammar_name: &str, grammar_dir: &str) -> io::Result<String> {
    fs::read_to_string(Path::new(grammar_dir).join(format!("{}.txt", grammar_name)))
}

fn ui(frame: &mut Frame, app_state: &mut AppState) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(4),
        ])
        .split(frame.area());

    let title = Paragraph::new("Grammar Parser").block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::LightBlue)),
    );
    frame.render_widget(title, main_chunks[0]);

    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(8)])
        .split(main_chunks[1]);

    match app_state.mode.clone() {
        Mode::MainMenu => render_main_menu(frame, content_chunks[1], app_state),
        Mode::ParseSentence(grammar_name) => {
            let help_text = Paragraph::new(format!(
                "Enter sentence to parse with '{}' (type 'Quit' to return)\n\
                Use PageUp/PageDown to scroll grammar rules",
                grammar_name
            ));
            frame.render_widget(help_text, content_chunks[0]);

            let rules_block = Block::default()
                .title("Grammar Rules")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::LightGreen));

            let rules_text = Text::from(app_state.current_grammar_rules.as_str());
            let rules = Paragraph::new(rules_text.clone())
                .block(rules_block)
                .scroll((app_state.scroll_offset, 0))
                .wrap(Wrap { trim: true });

            frame.render_widget(rules, content_chunks[1]);

            if rules_text.height() > content_chunks[1].height as usize {
                let scroll_info = Paragraph::new(format!(
                    "Scroll: {}/{}",
                    app_state.scroll_offset + 1,
                    rules_text.height()
                ))
                .alignment(Alignment::Right);
                frame.render_widget(scroll_info, content_chunks[1]);
            }
        }
    }

    let bottom_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(3)])
        .split(main_chunks[2]);

    if !app_state.message.is_empty() {
        let message = Paragraph::new(app_state.message.as_str()).style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );
        frame.render_widget(message, bottom_chunks[0]);
    }

    if matches!(app_state.mode, Mode::ParseSentence(_)) {
        let input = Paragraph::new(app_state.input.as_str())
            .block(Block::default().borders(Borders::ALL).title("Input"));
        frame.render_widget(input, bottom_chunks[1]);
    }
}

fn render_main_menu(frame: &mut Frame, area: Rect, app_state: &mut AppState) {
    let items: Vec<ListItem> = app_state
        .grammar_options
        .iter()
        .map(|opt| ListItem::new(Line::from(vec![Span::raw(opt.as_str())])))
        .collect();

    let list = List::new(items)
        .block(Block::default().title("Grammars").borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, area, &mut app_state.list_state);
}
