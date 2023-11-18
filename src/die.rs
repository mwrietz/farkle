use crate::tui_frm::Frame;
use crate::tui_gen::cmove;

pub struct Die<'a> {
    pub value: usize,
    pub label: &'a str,
    pub position: usize,
    pub active: bool,
    pub selected: bool,
}

impl Die<'_> {
    pub fn display_boundary(&self) {
        let mut _label_color = "";
        if self.active == true {
            if self.selected == true {
                _label_color = "blue";
            } else {
                _label_color = "green";
            }
        } else {
            _label_color = "red";
        }
        let frm = Frame {
            title: self.label,
            title_color: _label_color,
            frame_color: _label_color,
            x: 5 + self.position * 12,
            y: 5,
            w: 9,
            h: 4,
        };
        frm.display();
    }

    pub fn display_face(&self) {
        let x = 5 + self.position * 12;
        let y: usize = 5;
        let mut rows = Vec::new();

        rows.push(vec!["       ", "   *   ", "       "]);
        rows.push(vec!["     * ", "       ", " *     "]);
        rows.push(vec!["     * ", "   *   ", " *     "]);
        rows.push(vec![" *   * ", "       ", " *   * "]);
        rows.push(vec![" *   * ", "   *   ", " *   * "]);
        rows.push(vec![" *   * ", " *   * ", " *   * "]);

        let mut line_count: usize = 1;
        for i in 0..3 {
            cmove(x + 1, y + line_count);
            print!("{}", rows[self.value - 1][i]);
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
