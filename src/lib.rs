pub mod invest_items;

pub mod ansi_commands {
    const ANSI_CLEAR: &str = "\x1B[2J\x1B[H";
    const ANSI_RESET_COL: &str = "\x1B[39m";

    //Function to get colored text and reset it.
    pub fn get_text_colored(text: &str, color_val: u8) -> String {
        let set_color_sequence: String = format!("\x1B[38;5;{}m", color_val);
        format!("{}{}{}", set_color_sequence, text, ANSI_RESET_COL)
    }

    pub fn clear_screen() {
        print!("{ANSI_CLEAR}");
    }

    pub fn disable_cursor() {
        print!("\x1B[?;25;l");
    }

    pub fn enable_cursor() {
        print!("\x1B[?;25;h");
    }
}
