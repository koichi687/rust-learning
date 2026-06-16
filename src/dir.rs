use std::{env, fmt::format};

pub fn cmd_cd(path: &str) -> String {
    if path.is_empty() {
        return String::from("usage: cd <path>");
    }

    match env::set_current_dir(path) {
        Ok(_) => {
            let current = env::current_dir()
            .unwrap();
        format!("moved to: {}", current.display())
        }
        Err(e) => {
            format!("eror {}", e)
        }
    }
}

//handler buat enter keycode

pub fn hendler(cmd: &str, termout: &mut Vec<String>) {
    termout.push(format!("> {}", cmd));

    if cmd.starts_with("cd ") {
        let path = &cmd[3..];
        termout.push(cmd_cd(path));
    } else {
        termout.push(String::from("wrong syntax"));
    }
    termout.push(String::from(""));
}