pub fn format_checker(string: &str) -> i32 {
    let mut _option = 0;
    //converts to hex
    if string.contains('h') {
        _option = 1;
    //converts to binary
    } else if string.contains('b') {
        _option = 2;
    //opens help menu
    } else if string.contains('e') {
        _option = 3;
    //closes the program
    } else if string.contains('x') {
        _option = 4;
    //pass to mavel
    } else if string.chars().next().unwrap().is_numeric() {
        _option = 5;
    }
    _option
}
