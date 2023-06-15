use std::io::{stdout, Write};
extern crate meval;

fn main (){
    //displays inline and gets input
    let mut input = String::new();
    print!(">> ");
    stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();

    let mut conv = false;

    if input.contains('c') {
        conv = true;
    }

    match conv{
        true =>{
           let s = input.replace('c', ""); 
           let s2 = s.trim();
           let num: i32 = s2.parse().unwrap();
           let b = format!("{:b}", num);
           let h = format!("{:X}", num);
           println!("Binary: {}", b);
           println!("Hexadecimal: {}", h);
           println!("Decimal: {}", num);
        },
        false => {
            let r = meval::eval_str(input).unwrap();
            println!("{}", r);
        },
    }

}
