use crate::format::format_checker;

mod calc;
mod format;
mod input;
extern crate meval;

fn main() {
    let mut conv = 0;

    while conv != 4 {
        let string = input::input();
        conv = format_checker(&string);
        calc::calculator(conv, string);
    }
}
