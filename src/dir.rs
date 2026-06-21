use std::{env, fmt::format, io::SeekFrom::Current};

// use tokio::fs::read_dir;

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

pub fn cmd_ls () -> Vec<String> {
    let mut result = Vec::new();

    let current = match env::current_dir() {
        Ok(p) => p,
        Err(e) => {
            result.push(format!("eror {}", e ));
            return result
        } 
    };

    let entries = match std::fs::read_dir(&current) {
        Ok(e) => e,
        Err(e) => {
            result.push(format!("eror {}", e));
            return result
        }
    };
     for entry in entries {
        if let Ok(entry) = entry {
            let name = entry.file_name().to_string_lossy().to_string();
            let is_dir = entry.path().is_dir();
            if is_dir {
                result.push(format!("DIR {}", name))
            } else {
                result.push(format!(" FILE {}", name))
            }
        }
     }
     if result.is_empty() {
        result.push(String::from(" (empty)"));
     }

    result
}

//handler buat enter keycode

pub fn hendler(cmd: &str, termout: &mut Vec<String>) {
    termout.push(format!("> {}", cmd));

    if cmd.starts_with("cd ") {
        let path = &cmd[3..];
        termout.push(cmd_cd(path));
    } else if cmd == "ls" {
        for line in cmd_ls(){
            termout.push(line);
        }
    } //else if 
    else {
        termout.push(String::from("wrong syntax"));
    }
    termout.push(String::from(""));
}