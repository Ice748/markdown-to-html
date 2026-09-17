use std::io::{self, Write};
use std::process::Command;

fn main() {
  let mut data = String::new();
  loop {
    Command::new("clear").status().unwrap();
    println!("markdown-to-html\n");
    println!("1. File convert\n2. Text convert\n3. Exit");

    print!("[1, 2 or 3] >");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut data).expect("Failed to read line");

    match data.trim() {
      "1" => file_convert(),
      "2" => text_convert(),
      "3" => break,
      _ => {}
    }
  }
}

fn file_convert() {}
fn text_convert() {}
