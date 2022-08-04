use colored::Colorize;
use crate::tui_gen::cmove;

pub struct Frame<'a> {
    pub title: &'a str,
    pub title_color: &'a str,
    pub frame_color: &'a str,
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

impl Frame<'_> {
    #[allow(dead_code)]
    pub fn clear(&self) {
        // draw middle
        for i in 0..(self.h-1) {
            cmove(self.x+1, self.y+i+1);
            for _j in 0..(self.w-2) {
                print!(" ");
            }
        }
    }
    pub fn display(&self) {
        let ul = "╭".color(self.frame_color);
        let ur = "╮".color(self.frame_color);
        let ll = "╰".color(self.frame_color);
        let lr = "╯".color(self.frame_color);
        let hor = "─".color(self.frame_color);
        let ver = "│".color(self.frame_color);

        // draw top horizontal
        cmove(self.x, self.y);
        print!("{}", ul);
        for _i in 0..(self.w-2) {
            print!("{}", hor);
        }
        print!("{}", ur);

        // draw middle
        for i in 0..(self.h-1) {
            cmove(self.x, self.y+i+1);
            print!("{}", ver);
            for _j in 0..(self.w-2) {
                print!(" ");
            }
            print!("{}", ver);
        }

        // draw bottom horizontal
        cmove(self.x, self.y+self.h);
        print!("{}", ll);
        for _i in 0..(self.w-2) {
            print!("{}", hor);
        }
        println!("{}", lr);

        if self.title.len() > 0 {
            // print title 
            cmove(self.x+2, self.y);
            print!(" {} ", self.title.color(self.title_color));
        }
    }
}

pub struct MsgFrame<'a> {
    pub frame: Frame<'a>,
    pub msg: Vec<&'a str>,
}

impl MsgFrame<'_> {
    pub fn display_msg(&self) {
        for i in 0..self.msg.len() {
            if self.msg.len() > (self.frame.h - 1) {
                if i > (self.msg.len() - self.frame.h) {
                    cmove(self.frame.x + 2, self.frame.y + (i - (self.msg.len() - self.frame.h)));
                    print!("{}", self.msg[i]);
                }
            } else {
                cmove(self.frame.x + 2, self.frame.y + (i + 1));
                print!("{}", self.msg[i]);
            }
        }
    }
}

