fn main() {
    //EngEconCalculator::ansi_commands::clear_screen(false);
    //use EngEconCalculator::user_interface::*;

    //user_f_from_p();
    //user_p_from_f();
    //user_a_from_p();
    //user_p_from_a();
    //user_a_from_f();
    //user_f_from_a();

    //run();

    EngEconCalculator::ansi_commands::disable_text_wrapping();
    EngEconCalculator::invest_items::cli_disp::test_printing();
}

fn run() {
    EngEconCalculator::ansi_commands::clear_screen(false);
    use EngEconCalculator::user_interface as UI;

    let mut user_input = String::from("Bruh");

    while {
        //Simple do part of the loop to grab the user input. Then checks for exit and returns
        //corresponding conditional
        user_input = UI::grab_user_input("What would you like to calculate? (Type ? for options)")
            .trim()
            .to_lowercase();

        match user_input.as_str() {
            "e" | "end" | "exit" => false,
            _ => true,
        }
    } {
        match user_input.trim().to_lowercase().as_str() {
            "?" => {
                EngEconCalculator::ansi_commands::clear_screen(true);
                println!("Options are:");
                println!("\tExit (E, End, Exit)");
                println!("\tPresent Value (P, Present)");
                println!("\tFuture Value (F, Future)");
                println!("\tUniform Value (A, U, Uniform)");
                println!("\tArithmetic Gradient (AG, Arithmetic)");
            }
            "p" | "present" => {
                present_selected();
            }
            "f" | "future" => {
                future_selected();
            }
            "a" | "u" | "uniform" => {
                uniform_selected();
            }
            "ag" | "arithmetic" => {
                //TODO Put Function Here
                println!("Not Yet Implemented\n");
            }
            _ => {
                EngEconCalculator::ansi_commands::clear_screen(true);
                println!("Invalid Input. Please Try Again");
                continue;
            }
        }
    }

    EngEconCalculator::ansi_commands::clear_screen(false);
}

fn present_selected() {
    EngEconCalculator::ansi_commands::clear_screen(true);
    use EngEconCalculator::user_interface as UI;

    let mut user_input = String::new();

    while {
        user_input = UI::grab_user_input(
            "What would you like to calculate Present value from? (Input ? for options)",
        );
        user_input = user_input.trim().to_lowercase();
        match user_input.to_lowercase().as_str() {
            "e" | "end" | "exit" => false,
            _ => true,
        }
    } {
        match user_input.as_str() {
            "?" => {
                EngEconCalculator::ansi_commands::clear_screen(true);
                println!("Options are:");
                println!("\tFuture Value (F, Final)");
                println!("\tUniform Value (A, U, Uniform)");
                println!("\tArithmetic Gradient (AG, Arithmetic)");
                println!("\tGeometric Gradient (GG, Geometric)");
                continue;
            }
            "f" | "final" => {
                UI::user_p_from_f();
            }
            "a" | "u" | "uniform" => {
                UI::user_p_from_a();
            }
            "ag" | "arithmetic" => {
                UI::user_p_from_g();
            }
            "gg" | "geometric" => {
                UI::user_p_from_g_rate();
            }
            _ => {
                EngEconCalculator::ansi_commands::clear_screen(true);
                println!("Invalid Input. Please Try Again\n");
                continue;
            }
        }

        break;
    }
}

fn future_selected() {
    EngEconCalculator::ansi_commands::clear_screen(true);
    use EngEconCalculator::user_interface as UI;

    let mut user_input = String::new();

    while {
        user_input = UI::grab_user_input(
            "What would you like to calculate Future value from? (Input ? for options)",
        );
        user_input = user_input.trim().to_lowercase();
        match user_input.to_lowercase().as_str() {
            "e" | "end" | "exit" => false,
            _ => true,
        }
    } {
        match user_input.as_str() {
            "?" => {
                EngEconCalculator::ansi_commands::clear_screen(true);
                println!("Options are:");
                println!("\tPresent Value (P, Present)");
                println!("\tUniform Value (A, U, Uniform)");
                println!("\tArithmetic Gradient (AG, Arithmetic)");
                println!("\tGeometric Gradient (GG, Geometric)");
                continue;
            }
            "p" | "present" => {
                UI::user_f_from_p();
            }
            "a" | "u" | "uniform" => {
                UI::user_f_from_a();
            }
            "ag" | "arithmetic" => {
                UI::user_f_from_g();
            }
            "gg" | "geometric" => {
                UI::user_f_from_g_rate();
            }
            _ => {
                EngEconCalculator::ansi_commands::clear_screen(true);
                println!("Invalid Input. Please Try Again\n");
                continue;
            }
        }

        break;
    }
}

fn uniform_selected() {
    EngEconCalculator::ansi_commands::clear_screen(true);
    use EngEconCalculator::user_interface as UI;

    let mut user_input = String::new();

    while {
        user_input = UI::grab_user_input(
            "What would you like to calculate Uniform value from? (Input ? for options)",
        );
        user_input = user_input.trim().to_lowercase();
        match user_input.to_lowercase().as_str() {
            "e" | "end" | "exit" => false,
            _ => true,
        }
    } {
        match user_input.as_str() {
            "?" => {
                EngEconCalculator::ansi_commands::clear_screen(true);
                println!("Options are:");
                println!("\tPresent Value (P, Present)");
                println!("\tFuture Value (F, Future)");
                println!("\tArithmetic Gradient (AG, Arithmetic)");
                println!("\tGeometric Gradient (GG, Geometric)");
                continue;
            }
            "p" | "present" => {
                UI::user_a_from_p();
            }
            "f" | "future" => {
                UI::user_a_from_f();
            }
            "ag" | "arithmetic" => {
                UI::user_a_from_g();
            }
            "gg" | "geometric" => {
                UI::user_a_from_g_rate();
            }
            _ => {
                EngEconCalculator::ansi_commands::clear_screen(true);
                println!("Invalid Input. Please Try Again\n");
                continue;
            }
        }

        break;
    }
}
