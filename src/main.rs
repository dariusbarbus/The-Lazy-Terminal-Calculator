use crate::format::format_checker;

mod calc;
mod format;
mod input;
extern crate meval;

fn main() {
    //shows help command
    println!("use e for help");

    let mut conv = 0;

    while conv != 4 {
        //displays inline and gets input
        let string = input::input();
        conv = format_checker(&string); //change parameter to borrow instead of owning, maybe &?
        calc::calculator(conv, string);
    }
}
