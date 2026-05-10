use std::{thread, time::Duration};
use dialoguer::Input;
use indicatif::{ProgressBar, ProgressStyle};
use console::style;
use serde::{Serialize, Deserialize};

mod menu;

#[derive(Serialize, Deserialize, Debug)]
struct Point{
    activity: String,
    progress: Vec<String>,
}

//open read.md to explain how to use this cli, helpful my friend :)
fn lobby (){
    println!("welcome to jurnal listing !");
}
fn inactivy() -> String {
    println!("jurnal start, type another to make progression");
    Input::new()
        .with_prompt("input your activity")
        .interact_text()
        .unwrap()
} 
fn intask() -> Vec<String> {
    let mut list = Vec::new();
    println!("masukan list yang ingin anda isi (ketik 'exit' untuk berhenti):");
    loop {
        let input: String = Input::new()
            .with_prompt("kegiatan")
            .interact_text()
            .unwrap();

        if input == "exit" {
            break;
        }
        list.push(input);
        println!("List saat ini: {:?}", list);
    }
    list
}

fn main(){
    //menu 
    let pilihan = menu::menu();
    match pilihan {
        menu::MenuResult::Exit => {
            println!("bye");
            return;
        }
        menu::MenuResult::Edit => {
            println!("Fitur Edit/Access belum tersedia.");
            return;
        }
        menu::MenuResult::Add=>{

        }
    }

    lobby();

    let activity = inactivy();
    let list = intask();

    let isi = Point {
        activity: activity.clone(),
        progress: list,
    };

    //json compress
    let serialized = serde_json::to_string(&isi).unwrap();
    
    
    //bar
    let bar = ProgressBar::new(100);
    bar.set_style(
        ProgressStyle::default_bar()
            // bar.set_message("  objective complete  ")
            .template("{spinner:.green} [{bar:40.cyan/blue} {msg:^40}] {pos}/{len}")
            .unwrap()
            .progress_chars("#>-")
    );
    bar.set_message("Finish");

    for _ in 0..100 {
        bar.inc(1);
        thread::sleep(Duration::from_millis(30))
    }

    //output
    println!(
        "{} {}",
        style("Task complete : ").green().bold(),
        style(activity).yellow()
    );

    //json extract
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    

    //json save file




}