mod menu;
mod dir;

fn main (){
    let chose = menu::menuterm();
    match chose {
        menu::MenuResult::Exit => {
            println!("bye");
            return
        }
        menu::MenuResult::Term => {}
    }
}
