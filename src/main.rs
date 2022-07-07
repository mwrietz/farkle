// farkle
// 20220704
// m.w.rietz

use colored::Colorize;
use rand::Rng;
use std::process;

const ALL: u8 = 0;
const ACTIVE: u8 = 1;
const INACTIVE: u8 = 2;
const SELECTED: u8 = 3;
const MAINSTATUS: u8 = 4;
const DICE: u8 = 5;

const X_ALL: u16 = 5;
const Y_ALL: u16 = 16;
const X_ACTIVE: u16 = 5;
const Y_ACTIVE: u16 = 22;
const X_SELECTED: u16 = 41;
const Y_SELECTED: u16 = 16;
const X_INACTIVE: u16 = 41;
const Y_INACTIVE: u16 = 22;
const STATUS_WIDTH: u16 = 33;
const STATUS_HEIGHT: u16 = 5;

const X_DICE: u16 = 2;
const Y_DICE: u16 = 4;
const DICE_WIDTH: u16 = 75;
const DICE_HEIGHT: u16 = 8;
const X_MAINSTATUS: u16 = 2;
const Y_MAINSTATUS: u16 = 14;
const MAINSTATUS_WIDTH: u16 = 75;
const MAINSTATUS_HEIGHT: u16 = 15;

const X_FARKLE: u16 = 3;
const Y_FARKLE: u16 = 18;
const FARKLE_WIDTH: u16 = 73;
const FARKLE_HEIGHT: u16 = 3;

struct Die {
    value: u16,
    label: String,
    position: u16,
    active: bool,
    selected: bool,
}

struct Data {
    score: u16,
    roll_count: u16,
}

fn main() {
    i_o::cls();
    i_o::print_title_blue("Farkle");

    let mut dice = Vec::new();
    let mut data = Data {
        score: 0,
        roll_count: 0,
    };

    draw_status_window(DICE);
    draw_status_window(MAINSTATUS);
    draw_status_window(ALL);
    draw_status_window(ACTIVE);
    draw_status_window(INACTIVE);
    draw_status_window(SELECTED);

    initial_roll(&mut dice, &mut data);
    draw_all(&mut dice);

    update_roll_count(data.roll_count);

    update_status_window(&mut dice, ACTIVE);
    update_status_window(&mut dice, INACTIVE);
    update_status_window(&mut dice, SELECTED);

    menu(&mut dice, &mut data);
}

fn menu(dice: &mut Vec<Die>, data: &mut Data) {
    data.score = 0;
    data.roll_count = 1;
    loop {
        let menu_items = vec![
            "Sel", "Sel", "Sel", "Sel", "Sel", "Sel", "Keep Sel", "Roll", "Quit",
        ];
        let keys = vec!["a", "b", "c", "d", "e", "f", "k", "r", "q"];

        let selection = i_o::menu_horiz_blue(&keys, &menu_items);

        match selection {
            'a' => select(&mut dice[0]),
            'b' => select(&mut dice[1]),
            'c' => select(&mut dice[2]),
            'd' => select(&mut dice[3]),
            'e' => select(&mut dice[4]),
            'f' => select(&mut dice[5]),
            'k' => {
                data.score += score(dice, SELECTED);
                keep_selected(dice, data);
                draw_all(dice);
                update_status_window(dice, SELECTED);
                update_status_window(dice, ACTIVE);
                update_status_window(dice, INACTIVE);
                // display score_tot in INACTIVE status window
                i_o::cmove(X_INACTIVE + 2, Y_INACTIVE + 2);
                print!("score: {}    ", data.score);
            }
            'r' => {
                roll_unselected(dice, data);
                update_status_window(dice, ACTIVE);
            }
            'q' => {
                process::exit(1);
            }
            _ => usage(),
        }
        update_status_window(dice, SELECTED);
    }
}

fn count_dice(dice: &mut Vec<Die>, set: u8) -> u16 {
    let mut count: u16 = 0;
    if set == INACTIVE {
        for i in 0..dice.len() {
            if dice[i].active == false {
                count += 1;
            }
        }
    }
    count
}

fn count_values(dice: &mut Vec<Die>, set: u8) -> Vec<usize> {
    let mut counts = vec![0, 0, 0, 0, 0, 0, 0];
    for j in 1..7 {
        for i in 0..dice.len() {
            if set == ACTIVE {
                if dice[i].active == true {
                    if dice[i].value == j {
                        counts[j as usize] += 1;
                    }
                }
            }
            if set == INACTIVE {
                if dice[i].active == false {
                    if dice[i].value == j {
                        counts[j as usize] += 1;
                    }
                }
            }
            if set == SELECTED {
                if dice[i].selected == true {
                    if dice[i].value == j {
                        counts[j as usize] += 1;
                    }
                }
            }
            if set == ALL {
                if dice[i].value == j {
                    counts[j as usize] += 1;
                }
            }
        }
    }

    counts
}

fn display_boundary(die: &Die, label_color: String) {
    i_o::window(&i_o::Window {
        x: 5 + die.position * 12,
        y: 6,
        w: 9,
        h: 4,
        title: format!("{}", die.label),
        title_color: label_color,
    });
}

fn display_face(die: &Die) {
    let x = 5 + die.position * 12;
    let y: u16 = 6;
    let mut row1 = String::from("");
    let mut row2 = String::from("");
    let mut row3 = String::from("");

    if die.value == 1 {
        row1 = String::from("       ");
        row2 = String::from("   *   ");
        row3 = String::from("       ");
    }
    if die.value == 2 {
        row1 = String::from("     * ");
        row2 = String::from("       ");
        row3 = String::from(" *     ");
    }
    if die.value == 3 {
        row1 = String::from("     * ");
        row2 = String::from("   *   ");
        row3 = String::from(" *     ");
    }
    if die.value == 4 {
        row1 = String::from(" *   * ");
        row2 = String::from("       ");
        row3 = String::from(" *   * ");
    }
    if die.value == 5 {
        row1 = String::from(" *   * ");
        row2 = String::from("   *   ");
        row3 = String::from(" *   * ");
    }
    if die.value == 6 {
        row1 = String::from(" *   * ");
        row2 = String::from(" *   * ");
        row3 = String::from(" *   * ");
    }

    i_o::cmove(x + 1, y + 1);
    print!("{}", row1);
    i_o::cmove(x + 1, y + 2);
    print!("{}", row2);
    i_o::cmove(x + 1, y + 3);
    print!("{}", row3);
}

fn draw_all(dice: &mut Vec<Die>) {
    let l = dice.len();
    for i in 0..l {
        if dice[i].active == false {
            display_boundary(&dice[i], "red".to_string());
        } else {
            display_boundary(&dice[i], "green".to_string());
        }
        display_face(&dice[i]);
    }
}

fn draw_single(die: &Die) {
    if die.selected == true {
        display_boundary(&die, "blue".to_string());
    }
    else {
        display_boundary(&die, "green".to_string());
    }
    display_face(&die);

}

fn draw_status_window(set: u8) {
    if set == DICE {
        i_o::window(&i_o::Window {
            x: X_DICE,
            y: Y_DICE,
            w: DICE_WIDTH,
            h: DICE_HEIGHT,
            title: format!("{}", "Dice"),
            title_color: "white".to_string(),
        });
    }
    if set == MAINSTATUS {
        i_o::window(&i_o::Window {
            x: X_MAINSTATUS,
            y: Y_MAINSTATUS,
            w: MAINSTATUS_WIDTH,
            h: MAINSTATUS_HEIGHT,
            title: format!("{}", "Status"),
            title_color: "white".to_string(),
        });
    }
    if set == ALL {
        i_o::window(&i_o::Window {
            x: X_ALL,
            y: Y_ALL,
            w: STATUS_WIDTH,
            h: STATUS_HEIGHT,
            title: format!("{}", "Turn Status"),
            title_color: "white".to_string(),
        });
    }
    if set == INACTIVE {
        i_o::window(&i_o::Window {
            x: X_INACTIVE,
            y: Y_INACTIVE,
            w: STATUS_WIDTH,
            h: STATUS_HEIGHT,
            title: format!("{}", "Inactive (Scored) Dice"),
            title_color: "red".to_string(),
        });
    }
    if set == ACTIVE {
        i_o::window(&i_o::Window {
            x: X_ACTIVE,
            y: Y_ACTIVE,
            w: STATUS_WIDTH,
            h: STATUS_HEIGHT,
            title: format!("{}", "Active Dice"),
            title_color: "green".to_string(),
        });
    }
    if set == SELECTED {
        i_o::window(&i_o::Window {
            x: X_SELECTED,
            y: Y_SELECTED,
            w: STATUS_WIDTH,
            h: STATUS_HEIGHT,
            title: format!("{}", "Selected Dice"),
            title_color: "blue".to_string(),
        });
    }
}

fn farkle() {
    let (_width, height) = i_o::tsize();

    i_o::window(&i_o::Window {
        x: X_FARKLE,
        y: Y_FARKLE,
        w: FARKLE_WIDTH,
        h: FARKLE_HEIGHT,
        title: format!("{}", ""),
        title_color: "red".to_string(),
    });

    i_o::cmove(X_FARKLE + 1, Y_FARKLE + 1);
    print!(
        "{}",
        "                    * * *  F A R K L E !  * * *"
            .bold()
            .red()
    );
    i_o::cmove(X_FARKLE + 1, Y_FARKLE + 2);
    print!(
        "{}",
        "                    * * *  S c o r e = 0  * * *"
            .bold()
            .red()
    );

    // move to bottom of screen and clear menu
    i_o::cmove(0, height - 1);
    print!("                                                                                ");
    i_o::cmove(0, height - 2);
}

fn initial_roll(dice: &mut Vec<Die>, data: &mut Data) {
    // setup dice
    let mut rng = rand::thread_rng();
    let n_dice = 6;
    for d in 0..n_dice {
        let lbl = format!("{:?}", ((d as u8) + 97) as char);
        let instance = Die {
            value: rng.gen_range(1, 7),
            label: lbl,
            position: d,
            active: true,
            selected: false,
        };
        dice.push(instance);
    }

    // sort dice by value
    dice.sort_by_key(|x| x.value);

    // update position
    for i in 0..dice.len() {
        dice[i].position = i as u16;
        dice[i].label = format!("{:?}", ((i as u8) + 97) as char);
    }

    roll_unselected(dice, data);
}

fn keep_selected(dice: &mut Vec<Die>, data: &mut Data) {
    // make selected inactive
    for i in 0..dice.len() {
        if dice[i].selected == true {
            dice[i].active = false;
            dice[i].selected = false;
        }
    }

    let mut next_open = 0;

    // move inactives
    for i in 0..dice.len() {
        if dice[i].active == false {
            dice[i].position = next_open;
            next_open += 1;
        }
    }
    // move actives
    for i in 0..dice.len() {
        if dice[i].active == true {
            dice[i].position = next_open;
            next_open += 1;
        }
    }

    // if all dice inactive make all dice active and reroll
    if count_dice(dice, INACTIVE) == 6 {
        for i in 0..dice.len() {
            dice[i].active = true;
        }
        roll_unselected(dice, data);
    }
}

fn print_count(counts: &Vec<usize>) {
    print!("count: [");
    for i in 1..(counts.len() - 1) {
        print!("{}, ", counts[i]);
    }
    print!("{}]", counts[counts.len() - 1]);
}

fn roll_unselected(dice: &mut Vec<Die>, data: &mut Data) {
    let mut rng = rand::thread_rng();
    for i in 0..dice.len() {
        if dice[i].active == true && dice[i].selected == false {
            dice[i].value = rng.gen_range(1, 7);
            draw_single(&dice[i]);
        }
    }
    data.roll_count += 1;
    update_roll_count(data.roll_count);

    if score(dice, ACTIVE) == 0 {
        farkle();
        process::exit(1);
    }
}

fn select(die: &mut Die) {
    if die.active == true {
        if die.selected == false {
            die.selected = true;
        } else {
            die.selected = false;
        }
        //draw_single_select(&die);
        draw_single(&die);
    }
    draw_status_window(SELECTED);
}

fn score(dice: &mut Vec<Die>, set: u8) -> u16 {
    let counts = count_values(dice, set);
    let mut score = 0;
    let _sextet = 0;
    let _quintet = 0;
    let _quartet = 0;
    let _triplet = 0;
    let _pair = 0;

    let _sextet = counts.iter().filter(|&n| *n == 6).count();
    let _quintet = counts.iter().filter(|&n| *n == 5).count();
    let _quartet = counts.iter().filter(|&n| *n == 4).count();
    let _triplet = counts.iter().filter(|&n| *n == 3).count();
    let _pair = counts.iter().filter(|&n| *n == 2).count();
    let _straight = counts.iter().filter(|&n| *n == 1).count();

    if _sextet == 1 {
        score = 3000;
    }
    if _triplet == 2 {
        score = 2500;
    }
    if _quintet == 1 {
        score = 2000;
    }
    if _pair == 3 {
        score = 1500;
    }
    if _straight == 6 {
        score = 1500;
    }
    if _quartet == 1 && _pair == 1 {
        score = 1500;
    }
    if _quartet == 1 && _pair == 0 {
        score = 1000;
    }

    if _triplet == 1 {
        if counts[1] == 3 {
            score = 300 + counts[5] * 50;
        }
        if counts[2] == 3 {
            score = 200 + counts[1] * 100 + counts[5] * 50;
        }
        if counts[3] == 3 {
            score = 300 + counts[1] * 100 + counts[5] * 50;
        }
        if counts[4] == 3 {
            score = 400 + counts[1] * 100 + counts[5] * 50;
        }
        if counts[5] == 3 {
            score = 500 + counts[1] * 100;
        }
        if counts[6] == 3 {
            score = 600 + counts[1] * 100 + counts[5] * 50;
        }
    }

    if score == 0 {
        score = counts[1] * 100 + counts[5] * 50;
    }

    score as u16
}

fn update_status_window(dice: &mut Vec<Die>, set: u8) {
    let mut x = 0;
    let mut y = 0;
    let mut counts: Vec<usize> = Vec::new();

    if set == ALL {
        x = X_ALL + 2;
        y = Y_ALL + 1;
    }
    if set == ACTIVE {
        x = X_ACTIVE + 2;
        y = Y_ACTIVE + 1;
        counts = count_values(dice, ACTIVE);
    }
    if set == INACTIVE {
        x = X_INACTIVE + 2;
        y = Y_INACTIVE + 1;
        counts = count_values(dice, INACTIVE);
    }
    if set == SELECTED {
        x = X_SELECTED + 2;
        y = Y_SELECTED + 1;
        counts = count_values(dice, SELECTED);
    }

    // clear status window
    i_o::cmove(x, y);
    print!("                      ");
    y += 1;
    i_o::cmove(x, y);
    print!("                      ");
    y += 1;
    i_o::cmove(x, y);
    print!("                      ");
    y += 1;
    i_o::cmove(x, y);
    print!("                      ");
    y -= 3;

    if set != ALL {
        i_o::cmove(x, y);
        print_count(&counts);
        y += 1;

        if set == SELECTED {
            i_o::cmove(x, y);
            print!("selected score: {}    ", score(dice, set));
            y += 1;
        }
        if set == ACTIVE {
            i_o::cmove(x, y);
            print!("tentative score: {}    ", score(dice, set));
            y += 1;
        }

        // six of a kind
        let mut freq = counts.iter().filter(|&n| *n == 6).count();
        if freq == 1 {
            i_o::cmove(x, y);
            print!("six of a kind");
            y += 1;
        }
        // five of a kind
        freq = counts.iter().filter(|&n| *n == 5).count();
        if freq == 1 {
            i_o::cmove(x, y);
            print!("five of a kind");
            y += 1;
        }
        // four of a kind
        freq = counts.iter().filter(|&n| *n == 4).count();
        if freq == 1 {
            i_o::cmove(x, y);
            print!("four of a kind");
            y += 1;
        }
        // triplets
        freq = counts.iter().filter(|&n| *n == 3).count();
        if freq > 0 {
            i_o::cmove(x, y);
            print!("triplets: {}", freq);
            y += 1;
        }
        // pairs
        freq = counts.iter().filter(|&n| *n == 2).count();
        if freq > 0 {
            i_o::cmove(x, y);
            print!("pairs: {}", freq);
        }
    }
}

fn update_roll_count(count: u16) {
    i_o::cmove(X_ALL + 2, Y_ALL + 1);
    print!("rolls: {}    ", count);
}

fn usage() {
    i_o::cls();
    process::exit(1);
}
