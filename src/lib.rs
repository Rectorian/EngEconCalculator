pub fn test_ansi() {
    //This is fucking sick.
    let ansi_code_esc: char = '\x00';
    let ansi_code_bell: char = '\x07';
    println!("Ringing a Bell (hopefully)");
    println!("{}{}This is some test text", ansi_code_esc, ansi_code_bell);
}
