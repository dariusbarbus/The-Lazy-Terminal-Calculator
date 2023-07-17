use std::io::{stdout, Write};

//displays inline and gets input
pub fn input() -> String {
    let mut input = String::new();
    print!(">> ");
    stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}
