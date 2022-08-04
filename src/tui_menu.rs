use colored::Colorize;
use crossterm::{cursor, execute};
use getch::Getch;
use std::io;
use std::io::{stdout, Write};

use crate::tui_gen::horiz_line;
use crate::tui_gen::cmove;
use crate::tui_gen::tsize;

#[allow(dead_code)]
pub fn menu(menu_title: &str, items: &Vec<&str>) -> u8 {
    println!("{}", menu_title);
    for (i, item) in items.iter().enumerate() {
        println!("    {}) {}", i + 1, item);
    }

    println!("");
    print!("Selection: ");
    io::stdout().flush().unwrap();

    let mut _a: u8 = 0;
    let menu_len = items.len();
    loop {
        let g = Getch::new();
        _a = g.getch().unwrap();
        if _a <= 48 || _a > (48 + menu_len as u8) {
            continue;
        }
        break;
    }

    println!("");

    _a - 48
}

// menu_horiz - example use
//
// let keys = vec!["a", "r", "e", "d", "s", "m", "q"];
// let menu_items = vec!["Add", "Remove", "Edit", "Details", "Summary", "Menu", "Quit"];
// let val = menu_horiz(keys, menu_items);

//
// add color argument
//

#[allow(dead_code)]
pub fn menu_horiz(keys: &Vec<&str>, items: &Vec<&str>) -> char {
    let (_width, height) = tsize();
    cmove(0, height - 2);

    horiz_line("blue");
    for (i, item) in items.iter().enumerate() {
        print!("{:>4}:{}", keys[i].green(), item);
    }
    execute!(stdout(), cursor::Hide).unwrap();
    io::stdout().flush().unwrap();

    let mut _a: u8 = 0;
    let keys_len = keys.len();
    loop {
        let mut flag = false;
        let g = Getch::new();
        _a = g.getch().unwrap();

        for i in 0..keys_len {
            let ch = keys[i].chars().nth(0).unwrap();
            if (_a as char) == ch {
                flag = true;
                break;
            }
        }
        if flag == true {
            break;
        }
    }

    _a as char
}
