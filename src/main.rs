use std::io::{self, Write};
use std::fs;
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

fn file_convert() {
  let mut data = String::new();

  println!("File convert");

  print!("Enter the .md file path: ");
  io::stdout().flush().unwrap();
  io::stdin().read_line(&mut data).expect("Failed to reaad line");

  data.trim();
  let contents = fs::read_to_string(data).expect("Failed to read file");
  let mut file = File::create(format!("{data}.html")).expect("Failed to create file");

  let mut body = String::new();
  let mut chars_iter = contents.chars().peekable();

  let mut is_bold = false;
  let mut is_itlalics = false;

  while let Some(c) = chars_iter.next() {
      if c == '*' && chars_iter.peek() == Some(&'*') {
          chars_iter.next();
          
          if is_bold {
              html_body.push_str("</b>");
              is_bold = false;
          } else {
              html_body.push_str("<b>");
              is_bold = true;
          }
      } else if c == '*' {
          if is_italics {
              html_body.push_str("</i>");
              is_italics = false;
          } else {
              html_body.push_str("<i>");
              is_italics = true;
          }
      } else {
          html_body.push(c);
      }
  }

  let html = format!(
    r#"<!DOCTYPE html>
    <html lang="en">
    <head>
    	<meta charset="UTF-8">
    	<meta name="viewport" content="width=device-width, initial-scale=1.0">
    	<title>{data}</title>
    </head>
    <body>
      {contents}
    </body>
    </html>
    "#
  )
  file.write_all(html.as_bytes()).expect("Failed to write to file");
  println!("Succefully converted to {data}.html!");
}

fn text_convert() {}
