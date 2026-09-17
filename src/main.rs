use std::io;
use std::process::Command;

fn main() {
  let mut number = 0;
  const buttons[3]: [&str, &str, &str] = {
    "File convert",
    "Text convert",
    "Exit"
  };

  loop {
    Command::new("clear").status().unwrap();
    println!("markdown-to-html\n");

    for idx in 0..buttons.len() {
      println!("{} {buttons[idx]}", if idx == number ">>" else ">");
    }
  }
}
