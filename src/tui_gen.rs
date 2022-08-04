use colored::Colorize;
use crossterm::{cursor, execute};
use getch::Getch;
use std::io::{stdout, Write};

#[allow(dead_code)]
pub fn cls() {
    std::process::Command::new("clear").status().unwrap();
}

#[allow(dead_code)]
pub fn cmove(x: usize, y: usize) {
    execute!(stdout(), cursor::MoveTo(x as u16, y as u16)).unwrap();
}

#[allow(dead_code)]
pub fn horiz_line(color: &str) {
    for _i in 0..80 {
        print!("{}", "─".color(color).bold());
    }
    println!("");
}

#[allow(dead_code)]
pub fn pause() {
    let (w, h) = tsize();
    let clear_message = "                            ";
    let message = "Press any key to continue...".blue();
    let message_len: usize = message.len();
    cmove((w - message_len)/2, h - 2);
    print!("{}", message);
    std::io::stdout().flush().unwrap();
    let g = Getch::new();
    let _keypress = g.getch().unwrap();
    cmove((w - message_len)/2, h - 2);
    print!("{}", clear_message);
}

#[allow(dead_code)]
pub fn print_title(title_string: &str, color: &str) {
    println!("");
    for c in title_string.chars() {
        print!("{}", " ");
        print!("{}", c.to_string().color(color).bold());
    }
    println!("");
    horiz_line(color);
    println!("");
}

#[allow(dead_code)]
pub fn splash_screen(line1: &str, line2: &str) {
    //const VERSION: &str = env!("CARGO_PKG_VERSION");

    cls();
    let (width, height) = tsize();

    let line1_length: usize = line1.len();
    cmove(width / 2 - line1_length / 2, height / 2 - 1);
    println!("{}", line1.bold());

    let line2_length: usize = line2.len();
    cmove(width / 2 - line2_length / 2, height / 2 + 1);
    println!("{}", line2);

    execute!(stdout(), cursor::Hide).unwrap();

    // pause for splash screen
    //let one_sec = std::time::Duration::from_millis(1000);
    let dur = std::time::Duration::new(2, 0);
    std::thread::sleep(dur);
    cls();

    execute!(stdout(), cursor::Show).unwrap();
}

#[allow(dead_code)]
pub fn timestamp() -> String {
    let now = chrono::Local::now();
    return now.to_string();
}

#[allow(dead_code)]
pub fn tsize() -> (usize, usize) {
    let size = crossterm::terminal::size();
    let (w, h) = match size {
        Ok((w, h)) => (w, h),
        Err(error) => panic!("tsize error: {:?}", error),
    };
    (w as usize, h as usize)
}

