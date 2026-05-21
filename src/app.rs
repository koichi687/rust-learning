use std::io;
use console::style;
use crossterm::{
  event::{self, Event, KeyCode, KeyEventKind},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};

use crate::menu::menu;

enum Focuss {
  Terminal,
  Menu,
}

pub fn run() {
  //setup
  enable_raw_mode().unwrap();
  let mut stdout = io::stdout();
  execute!(stdout,EnterAlternateScreen).unwrap();

  //kiri
  let mut input = String::new();
  let mut output_line: Vec<String> = vec![
    String::from("perjrec v0.1.0 - by koichi"),
    String::from("type 'kch.test' to test"),
    String::from(""),
  ];
  //terminal
  let mut focus = Focus::Terminal;

  loop {
    terminal.draw(|f| {
      //interface atas dan bawah
      let vertical = Layout::default()
      .direction(Direction::Vertical)
      .constraints([
        Constraint::Min(0), 
        Constraint::Length(3)])
      .split(f.area());
    })
    //atass kiri >> menu, kanan >> main
    terminal.draw(|f| {
      //interface atas dan bawah
      let horizontal = Layout::default()
      .direction(Direction::Horizontal)
      .constraints([
        Constraint::Min(0), 
        Constraint::Length(20)])
      .split(f.area());
    })

    //kiri >> menu
    terminal.draw(|f| {
      //interface atas dan bawah
      let menu_list: Vec<ListItem> = menu_items
      .iter()
      .map(|i| ListItem::new(*i))
      .collect();
    let menu_style = match focus {
      Focus::menu => Style::default().fg(color::Cyan)
      .add_modifier(Modifier::BOLD),
    } 
    })
  }

  


}