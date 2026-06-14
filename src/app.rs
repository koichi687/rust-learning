// use std::io;
// use console::style;
// use crossterm::{
//   event::{self, Event, KeyCode, KeyEventKind},
//   execute,
//   terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
// };

// use ratatui::{
//     backend::CrosstermBackend,
//     layout::{Constraint, Direction, Layout},
//     style::{Color, Modifier, Style},
//     text::{Line, Span},
//     widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
//     Terminal,
// };

// // use crate::app::Focus::Terminal;

// // use crate::menu::menu

// enum Focus {
//   Terminal,
//   Menu,
// }

// pub fn run() {
//   //setup
//   enable_raw_mode().unwrap();
//   let mut stdout = io::stdout();
//   execute!(stdout,EnterAlternateScreen).unwrap();
//   let backend = CrosstermBackend::new(stdout);
//   let mut terminal = Terminal::new(backend).unwrap();

//   //menu 
//   let menu_items = vec!["Exit"];
//   let mut menu_state = ListState::default();
//   menu_state.select(Some(0));

//   //terminal
//   let mut input = String::new();
//   let mut output_lines: Vec<String> = vec![
//     String::from("perjrec v0.1.0 - by koichi"),
//     String::from("type 'kch.test' to test"),
//     String::from(""),
//   ];
//   let mut focus = Focus::Terminal;

//   loop {
//     // terminal.draw(|f| {
//     //   //interface atas dan bawah
//       // let vertical = Layout::default()
//       // .direction(Direction::Vertical)
//       // .constraints([
//       //   Constraint::Min(0), 
//       //   Constraint::Length(3)])
//       // .split(f.area());
//     // });
//     // //atass kiri >> menu, kanan >> main
//     // terminal.draw(|f| {
//     //   //interface atas dan bawah
//     //   let horizontal = Layout::default()
//     //   .direction(Direction::Horizontal)
//     //   .constraints([
//     //     Constraint::Min(0), 
//     //     Constraint::Length(20)])
//     //   .split(f.area());
//     // });

//     // //kiri >> menu
//     // terminal.draw(|f| {
//     //   //interface atas dan bawah
//     //   let menu_list: Vec<ListItem> = menu_items
//     //   .iter()
//     //   .map(|i| ListItem::new(*i))
//     //   .collect();
//     // let menu_style = match focus {
//     //   Focus::Menu => Style::default().fg(Color::Cyan)
//     //   .add_modifier(Modifier::BOLD),
//     //   Focus::Terminal => Style::default().fg(Color::DarkGray),
//     // };
//     terminal.draw(|f|{
//       let vertical = Layout::default()
//         .direction(Direction::Vertical)
//         .constraints([Constraint::Min(0), Constraint::Length(3)])
//         .split(f.area());
 
//         // layout kiri & kanan
//       let horizontal = Layout::default()
//         .direction(Direction::Horizontal)
//         .constraints([Constraint::Length(20), Constraint::Min(0)])
//         .split(vertical[0]);
 
//         // kiri: menu
//       let menu_list: Vec<ListItem> = menu_items
//         .iter()
//         .map(|i| ListItem::new(*i))
//         .collect();

//       let menu_style = match focus {
//         Focus::Menu => Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
//         Focus::Terminal => Style::default().fg(Color::DarkGray),
//       };
//       let menu_block = List::new(menu_list) // FIX: Menu_list -> menu_list
//         .block(Block::default().borders(Borders::ALL).title("Menu"))
//         .highlight_style(menu_style)
//         .highlight_symbol("> ");
//       f.render_stateful_widget(menu_block, horizontal[0], &mut menu_state);
 
//         // kanan: main
//       let main_text: Vec<Line> = output_lines
//         .iter()
//         .map(|l| Line::from(Span::raw(l.clone())))
//         .collect();
//       let main_block = Paragraph::new(main_text)
//         .block(Block::default().borders(Borders::ALL).title("main"));
//       f.render_widget(main_block, horizontal[1]);
 
//         // bawah: terminal
//       let term_style = match focus {
//         Focus::Terminal => Style::default().fg(Color::Green), // FIX: Terrminal -> Terminal
//         Focus::Menu => Style::default().fg(Color::DarkGray),
//       };
//       let term_title = match focus {
//         Focus::Terminal => "terminal",
//         Focus::Menu => "terminal (exit() to focus)",
//       };
//       let term_block = Paragraph::new(format!("> {}", input))
//         .block(Block::default().borders(Borders::ALL).title(term_title)) // FIX: block:: -> Block::, borders:: -> Borders::
//         .style(term_style);
//       f.render_widget(term_block, vertical[1]);
//     }).unwrap();

//     // let menu_block = List::new(Menu_list).block(Block::default().borders(Borders::ALL).title("Menu"))
//     // .highlight_style(menu_style)
//     // .highlight_symbol(">");
//     // f.render_stateful_widget(menu_block, horizontal[0], &mut menu_state);
  
//     // //kanan menu
//     // let term_style = match focus {
//     //   Focus::Terrminal => Style::default().fg(Color::Green),
//     //   Focus::Menu => Style::default().fg(Color::DarkGray),
//     // };
//     // let term_title = match focus {
//     //   Focus::Terminal => "terminal",
//     //   Focus::Menu => "terminal (exit() to focus)",
//     // };
//     // let term_block = Paragraph::new(format!("> {}",input))
//     //     .block(block::default().borders(borders::ALL).title(term_title))
//     //     .style(term_style);
//     //   f.render_widget(term_block, vertical[1]);
//     //   }).unwarp();

//       //handle
//      if let Event::Key(key) = event::read().unwrap() {
//             if key.kind != KeyEventKind::Press {
//                 continue;
//     } 

//       match focus{
//         Focus::Terminal => {
//           match key.code {
//             KeyCode::Char(c) => {
//               input.push(c);
//             }
//             KeyCode::Backspace => {
//               input.pop();
//             }
//             KeyCode::Enter => {
//               let cmd =  input.trim().to_string();
//               input.clear();

//               match cmd.as_str() {
//                 "kch.test" => {
//                     output_lines.push(String::from("$ kch.test"));
//                     output_lines.push(String::from("  connection ok!"));
//                     output_lines.push(String::from(""));
//                 }
//                 "kch.test" => {
//                     output_lines.push(String::from("$ add"));
//                     output_lines.push(String::from("  add not ready now"));
//                     output_lines.push(String::from(""));
//                 }
//                 "kch.test" => {
//                     output_lines.push(String::from("$ edit"));
//                     output_lines.push(String::from("  edit not ready now"));
//                     output_lines.push(String::from(""));
//                 }
//                 "kch.test" => {
//                     output_lines.push(String::from("$ exit"));
//                     output_lines.push(String::from("  switch to menu"));
//                     output_lines.push(String::from(""));

//                     output_lines.push(String::from(""));
//                     focus = Focus::Menu;
//                 }
//                 _ => {
//                       output_lines.push(format!("${}", cmd));
//                       output_lines.push(String::from("unknown command"));
//                       output_lines.push(String::from(" "));

//                 }
//               }
//             }
//             _ => {}
//           }
//         }
//         Focus::Menu => {
//           match key.code {
//             KeyCode::Up => {
//               let i => if i = 0 {menu_items.len() - 1} else {i - 1},
//               none => 0,
//             };
//             menu_state.select(some(i));
//           }
//           KeyCode::Down => {
//             let i = match menu_state.selected() {
//               some(i) => (i + 1) % menu_items.len(),
//               None => 0,
//             };
//             menu_state.select(Some(i));
//           }
//           KeyCode::Enter => {
//             match menu_state.selected() {
//               some(0) =>  break, //exit
//               _ => {}
//             }
//           }
//           _ => {}
//         }
//       }
//     }
//   }  

//   disable_raw_mode().unwrap();
//   execute!(terminal.backend_mut(), LeaveAlternateScreen).unwrap();
// }
