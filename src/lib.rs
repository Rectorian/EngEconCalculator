pub fn test_ansi() {
    //This is fucking sick.
    //This code is the Null code. It isn't needed for anything (That I know of right now)
    let ansi_code_null: char = '\x00';
    //Hex code for the bell
    let ansi_code_bell: char = '\x07';
    let octal_esc = 0o033;
    let ansi_code_esc: char = '\x1B';

    //Sequence that will cleear the screen and reset the cursor to the front position.
    let ansi_seq_clear_screen: &str = "\x1B[2J\x1B[H";

    println!("{}Ringing a Bell (hopefully)", ansi_seq_clear_screen);
    println!("{}This is some test text", ansi_code_bell);
    //println!("{}{}This is some test text", ansi_code_esc, ansi_code_bell);

    //Sequence to set background color to red.
    let ansi_color_sequence: &str = "\x1B[48:2:255:0:0m";

    //This sequence will print the the text red until reset or changed.
    //NOTE: While iTerm2 supports 8-bit color, the default macOS terminal does not. Use the other
    //color representation (5;ColorNumberHere)
    let ansi_color_picked: &str =
        "\x1B[38;2;255;0;0mThis text will print red in terminals that support 8-bit color";
    let ansi_text_color_sequence: &str = "\x1B[38:2:0:128:128m";
    //Don't use spaces within the sequence like the following [38 2;0;255;128m"; it causes an issue for some reason

    //A test to see if the 8 bit colors work correctly in the macOS terminal
    let mut combined_fore_and_back = String::new();
    format!("{}{}", ansi_color_picked, ansi_text_color_sequence);

    let ansi_non_256_background_color: &str = "\x1B[48;5;11m";

    //A statement that will result in text with a red background
    //Due to not resetting the color formatting before the newline is created, all the space up
    //until the first character of the next print statement will also have a red background
    //println!("{}{}This text's background should be red", ansi_color_sequence, ansi_text_color_sequence);

    //Me messing around with different ways to make the string to try and solve the issue that
    //ended up being me not understanding that the default macOS terminal does not have 8-bit color
    //for ANSI
    combined_fore_and_back += "This is some text I want to print";

    println!("{}", combined_fore_and_back);

    println!("\x1B[0mThe following text should be red");
    println!("{}", ansi_color_picked);
}

pub mod ansi_commands {
    pub const ANSI_CLEAR: &str = "\x1B[2J\x1B[H";
    pub const ANSI_RESET_COL: &str = "\x1B[38m";

    //Function to get colored text and reset it.
    pub fn get_text_colored(text: String, color_val: u8) -> String {
        let set_color_sequence: String = format!("\x1B[38;5;{}m", color_val);
        format!("{}{}{}", set_color_sequence, text, ANSI_RESET_COL)
    }
}
