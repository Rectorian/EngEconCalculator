pub fn test_ansi() {
    //This is fucking sick.
    //This code is the Null code. It isn't needed for anything (That I know of right now)
    let ansi_code_null: char = '\x00';
    //Hex code for the bell
    let ansi_code_bell: char = '\x07';
    let octal_esc = 0o033;
    let ansi_code_esc: char = '\x1B';

    println!("Ringing a Bell (hopefully)");
    println!("{}This is some test text", ansi_code_bell);
    //println!("{}{}This is some test text", ansi_code_esc, ansi_code_bell);

    let ansi_color_sequence: &str = "\x1B[48:2:255:0:0m";
    //The following will print the text red in both iterm2 and the default macOS terminal
    //My suspicion was that for some reason, only text contained within the string literal will be
    //printed with the correct coloring, and I was right. Will need to figure out how to get around
    //this. The library I looked at seemed to combine everything before sending to the output?
    let ansi_color_picked: &str = "\x1B[31mThis text will print red";
    let ansi_text_color_sequence: &str = "\x1B[38:2:0:128:128m"; //Don't use spaces within the
                                                                 //sequence like the following [38 2;0;255;128m"; it causes an issue for some reason
                                                                 //A test to see if the 8 bit colors work correctly in the macOS terminal
    let mut combined_fore_and_back = String::new();
    format!("{}{}", ansi_color_picked, ansi_text_color_sequence);

    let ansi_non_256_background_color: &str = "\x1B[48;5;11m";

    //From the way this works, it seems that after we set the background to red, it seems that this
    //becomes thedefault?
    //Text printed after this using the println! macro can be adjusted to be different colors and
    //backgrounds, but for some reason, all the white space afterwards has a red background too,
    //but just the first line
    //Must be due to the underlying way that rust implements concatenation using macros? I don't
    //have enough experience to make a specific determination
    //println!("{}{}This text's background should be red", ansi_color_sequence, ansi_text_color_sequence);

    //So I think the way that rust concatenate strings, it does it in a way that seems to get rid
    //of some of the escaped stuff?
    combined_fore_and_back += "This is some text I want to print";

    println!("{}", combined_fore_and_back);

    println!("\x1B[0mThe following text should be red");
    println!("{}", ansi_color_picked);
}
