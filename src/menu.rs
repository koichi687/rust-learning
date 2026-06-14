use std::io;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

//ratattui -> backend to acses terminal (ya gitu lah kira kira) kode backend di line 25-42
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

pub enum MenuResult {
    Exit,
    Term,
}

pub fn menu() -> MenuResult {
    //terminal
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap(); //buat jalanin terminal
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    //fisrt
    let main_items = vec!["0. Exit", "1. Terminal"]; //5. about add  
    let mut main_state = ListState::default();
    main_state.select(Some(1));

    //sub
    let sub_items = vec!["Exit"];
    let mut sub_state = ListState::default();
    sub_state.select(Some(0));

    let mut depth: usize = 0;

    //set terminal
    let mut termin = String::new();
    let mut termout: Vec<String> = vec![
        String::from("terminal ready. type a command. . ."),
        String::from(""),
    ];

    let result = loop {
        terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Length(25), Constraint::Min(0)])
                    .split(f.area());

                // kiri: menu utama
                let main_list: Vec<ListItem> =
                    main_items.iter().map(|i| ListItem::new(*i)).collect();
                let main_block = List::new(main_list)
                    .block(Block::default().borders(Borders::ALL).title("Menu"))
                    .highlight_style(
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )
                    .highlight_symbol("> ");
                f.render_stateful_widget(main_block, chunks[0], &mut main_state);

                //layout
                if depth == 1 {
                    let term_chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Min(0), Constraint::Length(3)])
                        .split(chunks[1]);
                    let out_text: Vec<Line> = termout
                        .iter()
                        .map(|l| Line::from(Span::raw(l.clone())))
                        .collect();
                    let out_block = Paragraph::new(out_text)
                        .block(Block::default().borders(Borders::ALL).title("Terminal"));
                    f.render_widget(out_block, term_chunks[0]);
                    let in_block = Paragraph::new(format!("> {}", termin))
                    .block(Block::default().borders(Borders::ALL));
                    f.render_widget(in_block, term_chunks[1]);
                }
            }).unwrap();

        //handle input keyboard
        if let Event::Key(key) = event::read().unwrap() {
            //loop keyboard fix
            if key.kind != KeyEventKind::Press {
                continue;
            }

            if depth == 0 {
                match key.code {
                    KeyCode::Up => {
                        let i = match main_state.selected() {
                            Some(i) => {
                                if i == 0 {
                                    main_items.len() - 1
                                } else {
                                    i - 1
                                }
                            }
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
                            // Some(2) => depth = 2,
                            _ => {}
                        }
                    }
                    _ => {}
                }
            } else {
                match key.code {
                    KeyCode::Char(c) => {
                        termin.push(c);
                    }
                    KeyCode::Backspace => {
                        termin.pop();
                    }
                    KeyCode::Enter => {
                        let cmd = termin.trim().to_string();
                        termin.clear();
                        termout.push(format!("> {}", cmd));
                        termout.push(String::from("wrong syntax"));
                        termout.push(String::from(" "));
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
