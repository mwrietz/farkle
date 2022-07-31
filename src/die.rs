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
        if self.active == true {
            if self.selected == true {
                label_color = "blue".to_string();
            }
            else {
                label_color = "green".to_string();
            }
        }
        else {
            label_color = "red".to_string();
        }
        let frm = i_o::Frame {
            title: format!("{}", self.label),
            title_color: label_color,
            x: 5 + self.position * 12,
            y: 5,
            w: 9,
            h: 4,
        };
        frm.display();
    }

    pub fn display_face(&self) {
        let x = 5 + self.position * 12;
        let y: u16 = 5;
        let mut rows = Vec::new();

        rows.push(vec!["       ", "   *   ", "       "]);
        rows.push(vec!["     * ", "       ", " *     "]);
        rows.push(vec!["     * ", "   *   ", " *     "]);
        rows.push(vec![" *   * ", "       ", " *   * "]);
        rows.push(vec![" *   * ", "   *   ", " *   * "]);
        rows.push(vec![" *   * ", " *   * ", " *   * "]);

        let mut line_count = 1;
        for i in 0..3 {
            i_o::cmove(x + 1, y + line_count);
            print!("{}", rows[self.value as usize - 1][i]);
            line_count += 1;
        }
    }

    pub fn display_die(&self) {
        self.display_boundary();
        self.display_face();
    }

    pub fn select(&mut self) {
        if self.active == true {
            self.selected = !self.selected;
            self.display_die();
        }
    }
}
