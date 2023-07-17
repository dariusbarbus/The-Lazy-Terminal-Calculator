//use std::io::{stdout, Write};
extern crate meval;

// Switches functionality between different modes
pub fn calculator(conv: i32, eq: String) {
    if conv == 1 {
        println!("CONVERSION NOT WORKING");
    } else if conv == 2 {
        println!("NO HELP MENU");
    } else if conv == 0 {
        let r = meval::eval_str(eq).unwrap();
        println!("{}", r);
    }
}
