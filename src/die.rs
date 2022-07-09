//use colored::Colorize;
//use rand::Rng;
//use std::process;

pub struct Die {
    pub value: u16,
    pub label: String,
    pub position: u16,
    pub active: bool,
    pub selected: bool,
}

impl Die {

    pub fn display_boundary(&self) {
        let mut label_color = String::from("");
        if self.active == true { label_color = "green".to_string(); }
        if self.active == false { label_color = "red".to_string(); }
        if self.selected == true { label_color = "blue".to_string(); }
        let frm = i_o::Frame {
            title: format!("{}", self.label),
            title_color: label_color,
            x: 5 + self.position * 12,
            y: 6,
            w: 9,
            h: 4,
        };
        frm.display();
    }

    pub fn display_face(&self) {
        let x = 5 + self.position * 12;
        let y: u16 = 6;
        let mut rows = Vec::new(); 
        if self.value == 1 { rows = vec!["       ", "   *   ", "       "]; }
        if self.value == 2 { rows = vec!["     * ", "       ", " *     "]; }
        if self.value == 3 { rows = vec!["     * ", "   *   ", " *     "]; }
        if self.value == 4 { rows = vec![" *   * ", "       ", " *   * "]; }
        if self.value == 5 { rows = vec![" *   * ", "   *   ", " *   * "]; }
        if self.value == 6 { rows = vec![" *   * ", " *   * ", " *   * "]; }

        let mut line_count = 1;
        for i in 0..3 {
            i_o::cmove(x + 1, y + line_count);
            print!("{}", rows[i]);
            line_count += 1;
        }
    }

    pub fn display_die(&self) {
        self.display_boundary();
        self.display_face();
    }

    pub fn select(&mut self) {
        if self.active == true {
            if self.selected == false {
                self.selected = true;
            } else {
                self.selected = false;
            }
            self.display_die();
        }
    }

}
