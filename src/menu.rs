use std::io;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal
};

pub enum MenuResult{
    Exit,
    Add,
    Edit
}

pub fn menu() -> MenuResult {
    //terminal
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    //fisrt
    let main_items = vec!["0. Exit", "1. Activity"];
    let mut main_state = ListState::default();
    main_state.select(Some(1));

    //sub
    let sub_items = vec!["Add", "Edit / Access", "Exit"];
    let mut sub_state = ListState::default();
    sub_state.select(Some(0));

     let mut depth: usize = 0;
 
let result = loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Length(25), Constraint::Min(0)])
                .split(f.area());
 
            // Panel kiri: menu utama
            let main_list: Vec<ListItem> = main_items
                .iter()
                .map(|i| ListItem::new(*i))
                .collect();
            let main_block = List::new(main_list)
                .block(Block::default().borders(Borders::ALL).title("Menu"))
                .highlight_style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("> ");
            f.render_stateful_widget(main_block, chunks[0], &mut main_state);
 
            // Panel kanan: submenu muncul kalau activity
            if depth == 1 {
                let sub_list: Vec<ListItem> = sub_items
                    .iter()
                    .map(|i| ListItem::new(*i))
                    .collect();
                let sub_block = List::new(sub_list)
                    .block(Block::default().borders(Borders::ALL).title("Activity"))
                    .highlight_style(
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    )
                    .highlight_symbol("> ");
                f.render_stateful_widget(sub_block, chunks[1], &mut sub_state);
            }
        }).unwrap();
 
        // Handle input keyboard
        if let Event::Key(key) = event::read().unwrap() {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            if depth == 0 {
                match key.code {
                    KeyCode::Up => {
                        let i = match main_state.selected() {
                            Some(i) => if i == 0 { main_items.len() - 1 } else { i - 1 },
                            None => 0,
                        };
                        main_state.select(Some(i));
                    }
                    KeyCode::Down => {
                        let i = match main_state.selected() {
                            Some(i) => (i + 1) % main_items.len(),
                            None => 0,
                        };
                        main_state.select(Some(i));
                    }
                    KeyCode::Enter => {
                        match main_state.selected() {
                            Some(0) => break MenuResult::Exit,
                            Some(1) => depth = 1,
                            _ => {}
                        }
                    }
                    _ => {}
                }
            } else {
                match key.code {
                    KeyCode::Up => {
                        let i = match sub_state.selected() {
                            Some(i) => if i == 0 { sub_items.len() - 1 } else { i - 1 },
                            None => 0,
                        };
                        sub_state.select(Some(i));
                    }
                    KeyCode::Down => {
                        let i = match sub_state.selected() {
                            Some(i) => (i + 1) % sub_items.len(),
                            None => 0,
                        };
                        sub_state.select(Some(i));
                    }
                    KeyCode::Enter => {
                        match sub_state.selected() {
                            Some(0) => break MenuResult::Add,
                            Some(1) => break MenuResult::Edit,
                            Some(2) => depth = 0,
                            _ => {}
                        }
                    }
                    KeyCode::Esc => {
                        depth = 0;
                    }
                    _ => {}
                }
            }
        }
    };

    //restore
    disable_raw_mode().unwrap();
    execute!(terminal.backend_mut(), LeaveAlternateScreen).unwrap();
 
    result
}   