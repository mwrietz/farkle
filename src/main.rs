use colored::Colorize;
use rand::Rng;
use std::process;

mod die;
use crate::die::Die;

// status windows
const TURN_STATUS: u8 = 2;
const ACTIVE: u8 = 3;
const INACTIVE: u8 = 4;
const SELECTED: u8 = 5;

struct Data {
    score: u16,
    roll_count: u16,
}

fn main() {
    i_o::cls();
    i_o::print_title_blue("Farkle");

    let mut ui = Vec::new();
    ui_setup(&mut ui);
    ui_display(&ui);

    let mut dice = Vec::new();
    let mut data = Data {
        score: 0,
        roll_count: 0,
    };

    initial_roll(&mut dice, &mut data);
    display_dice(&mut dice);

    update_status_window(&mut dice, &mut data, &ui, TURN_STATUS);
    update_status_window(&mut dice, &mut data, &ui, ACTIVE);
    update_status_window(&mut dice, &mut data, &ui, INACTIVE);
    update_status_window(&mut dice, &mut data, &ui, SELECTED);

    menu(&mut dice, &mut data, &ui);
}

fn menu(dice: &mut Vec<Die>, data: &mut Data, ui: &Vec<i_o::Frame>) {
    data.score = 0;
    data.roll_count = 1;
    loop {
        let menu_items = vec![
            "Sel", "Sel", "Sel", "Sel", "Sel", "Sel", "Keep Sel", "Roll", "Quit",
        ];
        let keys = vec!["a", "b", "c", "d", "e", "f", "k", "r", "q"];

        let selection = i_o::menu_horiz_blue(&keys, &menu_items);

        match selection {
            'a' => dice[0].select(),
            'b' => dice[1].select(),
            'c' => dice[2].select(),
            'd' => dice[3].select(),
            'e' => dice[4].select(),
            'f' => dice[5].select(),
            'k' => {
                data.score += score(dice, SELECTED);
                keep_selected(dice, data);
                display_dice(dice);
                update_status_window(dice, data, ui, TURN_STATUS);
                update_status_window(dice, data, ui, SELECTED);
                update_status_window(dice, data, ui, ACTIVE);
                update_status_window(dice, data, ui, INACTIVE);
            }
            'r' => {
                roll_unselected(dice, data);
                update_status_window(dice, data, ui, TURN_STATUS);
                update_status_window(dice, data, ui, ACTIVE);
            }
            'q' => {
                process::exit(1);
            }
            _ => usage(),
        }
        update_status_window(dice, data, ui, SELECTED);
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
        }
    }

    counts
}

fn display_dice(dice: &mut Vec<Die>) {
    for i in 0..dice.len() {
        dice[i].display_die();
    }
}

fn farkle() {
    let (_width, height) = i_o::tsize();

    let frm = i_o::Frame {
        title: format!("{}", ""),
        title_color: "red".to_string(),
        x: 3,
        y: 18,
        w: 73,
        h: 3,
    };

    frm.display();

    i_o::cmove(frm.x + 1, frm.y + 1);
    print!(
        "                    {}",
        "* * *  F A R K L E !  * * *".bold().red()
    );
    i_o::cmove(frm.x + 1, frm.y + 2);
    print!(
        "                    {}",
        "* * *  S c o r e = 0  * * *".bold().red()
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
            dice[i].display_die();
        }
    }
    data.roll_count += 1;

    if score(dice, ACTIVE) == 0 {
        farkle();
        process::exit(1);
    }
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

fn update_status_window(dice: &mut Vec<Die>, data: &mut Data, ui: &Vec<i_o::Frame>, set: u8) {
    let x = ui[set as usize].x + 2;
    let mut y = ui[set as usize].y + 1;
    let counts = count_values(dice, set);

    // clear status window
    for _ in 0..4 {
        i_o::cmove(x, y);
        print!("                      ");
        y += 1;
    }
    y -= 4;

    if set == TURN_STATUS {
        i_o::cmove(x, y);
        print!("rolls: {}    ", data.roll_count);
    }

    if set == SELECTED || set == ACTIVE || set == INACTIVE {
        i_o::cmove(x, y);
        print_count(&counts);
        y += 1;
    }

    if set == INACTIVE {
        i_o::cmove(x, y);
        print!("score: {}    ", data.score);
    }

    if set == SELECTED || set == ACTIVE {
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

        let labels: Vec<&str> = vec!["six of a kind", "five of a kind", "four of a kind", "triplets", "pairs"];
        let values: Vec<usize> = vec![6, 5, 4, 3, 2];

        for i in 0..labels.len() {
            let freq = counts.iter().filter(|&n| *n == values[i]).count();
            if freq > 0 {
                i_o::cmove(x, y);
                print!("{}: {}", labels[i], freq);
                y += 1;
            }
        }
    }
}

fn ui_display(ui: &Vec<i_o::Frame>) {
    for i in 0..ui.len() {
        ui[i].display();
    }
}

fn ui_setup(ui: &mut Vec<i_o::Frame>) {
    let title = vec![
        "DICE",
        "STATUS",
        "Turn Status",
        "Active Dice",
        "Inactive (Scored) Dice",
        "Selected Dice",
    ];
    let title_color = vec!["white", "white", "white", "green", "red", "blue"];
    let x = vec![2, 2, 5, 5, 41, 41];
    let y = vec![4, 13, 15, 21, 21, 15];
    let w = vec![75, 75, 33, 33, 33, 33];
    let h = vec![7, 14, 5, 5, 5, 5];
    for i in 0..6 {
        ui.push(i_o::Frame {
            title: title[i].to_string(),
            title_color: title_color[i].to_string(),
            x: x[i],
            y: y[i],
            w: w[i],
            h: h[i],
        });
    }
}

fn usage() {
    i_o::cls();
    process::exit(1);
}
