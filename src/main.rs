use std::io::{self, Write};
use std::fs::{self, File};
use std::process::Command;

fn clear() {
  if cfg!(target_os = "windows") {
    Command::new("cmd").args(["/C", "cls"]).status().unwrap();
  } else {
    Command::new("clear").status().unwrap();
  }
}

fn main() {
  let mut data = String::new();
  loop {
    clear();
    println!("markdown-to-html\n");
    println!("1. File convert\n2. Text convert\n3. Exit");

    print!("[1, 2 or 3] > ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut data).expect("Failed to read line");

    match data.trim() {
      "1" => file_convert(),
      "2" => text_convert(),
      "3" => break,
      _ => {}
    }
    data.clear();
  }
}

fn file_convert() {
  let mut data = String::new();

  clear();
  println!("File convert");

  print!("Enter the .md file path: ");
  io::stdout().flush().unwrap();
  io::stdin().read_line(&mut data).expect("Failed to read line");

  let path = data.trim();
  let contents = fs::read_to_string(path).expect("Failed to read file");
  let mut file = File::create(format!("{path}.html")).expect("Failed to create file");

  let mut body = String::new();
  let mut chars_iter = contents.chars().peekable();

  let mut is_bold = false;
  let mut is_italics = false;

  while let Some(c) = chars_iter.next() {
      if c == '*' && chars_iter.peek() == Some(&'*') {
          chars_iter.next();
          
          if is_bold {
              body.push_str("</b>");
              is_bold = false;
          } else {
              body.push_str("<b>");
              is_bold = true;
          }
      } else if c == '*' {
          if is_italics {
              body.push_str("</i>");
              is_italics = false;
          } else {
              body.push_str("<i>");
              is_italics = true;
          }
      } else {
          body.push(c);
      }
  }

  let html = format!(
r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{path}</title>
</head>
<body>
  {body}
</body>
</html>
"#
  );

  file.write_all(html.as_bytes()).expect("Failed to write to file");
  println!("Successfully converted to {path}.html!");

  io::stdin().read_line(&mut data).expect("Failed to read line");
}

fn text_convert() {}
