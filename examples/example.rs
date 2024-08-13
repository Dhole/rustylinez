extern crate rustylinez;

use std::time::Duration;
use std::thread;
use rustylinez::error::ReadlineError;
use rustylinez::Editor;

fn main() {
    let mut rl = Editor::new();
    if let Err(_) = rl.load_history("history.txt") {
        println!("No previous history.");
    }
    let rl_println = rl.get_println();
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(3));
            rl_println(format!("OLA K ASE"));
        }
    });
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                println!("Line: {}", line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}
