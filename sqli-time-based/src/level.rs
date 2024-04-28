#![allow(unused)]

use crate::dbg_print;

pub struct Level {
    pub more: bool,
    pub result: u8,
    quant: u8,
    index: u8,
    count: u8,
}

impl Level {
    pub fn new(max: u8) -> Level {
        Level {
            index: 0,
            more: false,
            result: max / 2,
            quant: max / 2 / 2 + 1,
            count: 0,
        }
    }
    /// Divide & conquer, works for 0..=254, fails for 255. Oh well... ¯\_(ツ)_/¯
    pub fn completed(&mut self, result: bool, debug: bool) -> bool {
        self.index += 1;
        dbg_print!(
            debug,
            "run: {}, result: {}, op: {}, than: {}, quant: {}",
            self.index,
            result,
            if self.more { "more" } else { "less" },
            self.result,
            self.quant
        );
        if self.more {
            if result {
                dbg_print!(debug, "{}", true);
                self.result += self.quant;
                self.quant /= 2;
                self.count = 0;
            } else {
                dbg_print!(debug, "{}", false);
                self.more = false;
                self.count += 1;
            }
        } else {
            if result {
                dbg_print!(debug, "{}", true);
                self.result -= self.quant;
                self.quant /= 2;
                self.count = 0;
            } else {
                dbg_print!(debug, "{}", false);
                self.more = true;
                self.count += 1;
            }
        }
        self.count > 1
    }
    pub fn test(needle: u8, max: u8, debug: bool) -> u8 {
        let mut level = Level::new(max);
        loop {
            let result = {
                if level.more {
                    needle > level.result
                } else {
                    needle < level.result
                }
            };
            if level.completed(result, debug) {
                break;
            }
        }
        level.result
    }
}
