extern crate meval;

// Switches functionality between different modes
pub fn calculator(conv: i32, eq: String) {
    //converts number (any format) to hex
    if conv == 1 {
        let s = eq.replace('h', "");
        let s2 = s.trim();
        let num: i32 = s2.parse().unwrap();
        println!("= {:X}", num);
    //converts number (any format) to binary
    } else if conv == 2 {
        let s = eq.replace('b', "");
        let s2 = s.trim();
        let num: i32 = s2.parse().unwrap();
        println!("= {:b}", num);
    //Shows the help menu
    } else if conv == 3 {
        help();
    //Calculator
    } else if conv == 5 {
        let r = meval::eval_str(eq).unwrap();
        println!("= {}", r);
    }
}

fn help() {
    println!(
        "               
    ####################################
    #The LTC - Lazy Terminal Calculator#
    ####################################
    
    h   :   Converts number to Hexadecimal
        -   h 10 or h10
    b   :   Converts number to Binary
        -   b 10 or b10
    x   :   Closes calculator
    e   :   Opens this menu

    You can find more about the project at 
    "
    )
}
