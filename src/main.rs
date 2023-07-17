mod calc;
mod input;
extern crate meval;

fn main() {
    //shows help command
    println!("use -h for help");

    let mut conv = 0;

    while conv != 3 {
        //displays inline and gets input
        let string = input::input();

        match conv {
            1 => {
                conv = 1;
            }
            2 => {
                conv = 2;
            }
            3 => {
                conv = 3;
            }
            _ => {}
        }

        calc::calculator(conv, string);
    }
}

// fn main() {
//     //displays inline and gets input
//     let mut input = String::new();
//     println!("use -h for help");
//     print!(">> ");
//     stdout().flush().unwrap();
//     std::io::stdin().read_line(&mut input).unwrap();

//     let mut conv = 0;

//     if input.contains('c') {
//         conv = 1;
//     }

//     if input.contains("-h") {
//         conv = 2;
//     }

//     match conv {
//         1 => {
//             let s = input.replace('c', "");
//             let s2 = s.trim();
//             let num: i32 = s2.parse().unwrap();
//             let b = format!("{:b}", num);
//             let h = format!("{:X}", num);
//             println!("Binary: {}", b);
//             println!("Hexadecimal: {}", h);
//             println!("Decimal: {}", num);
//         }
//         0 => {
//             let r = meval::eval_str(input).unwrap();
//             println!("{}", r);
//         }
//         2 => {
//             println!(
//                 "
//                 ####################################
//                 #The LTC - Lazy Terminal Calculator#
//                 ####################################

// To convert a number to another numbering system, write 'c' followed by the number, for example: c 10

// Otherwise, write the equations, this calculator uses proper operator order to process the requiest"
//             );
//         }
//         _ => {}
//     }
// }
