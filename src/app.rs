use std::io;
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

enum Focuss {
  Terminal,
  Menu,
}

pub fn run() {
  //setup
  enable_rawmode().unwaarp();
  let mut stdout = io::stdout();
  execute!(stdout, )
}