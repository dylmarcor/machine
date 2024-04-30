pub mod help {
    use std::io;

    // Help Functions
    pub fn display_help() {
        println!("Which function would you like help with?");

        loop {
            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Could not read user input");

            let input = input.trim(); 

            if input == "list" {
                print!("{}[2J", 27 as char);
                println!("Here are the current functions available:");
                println!("rpm");
                println!("sfpm");
            }

            if input == "rpm" {
                print!("{}[2J", 27 as char);
                println!("You must enter the SFPM and the Diameter of the material in inches, in that order.\n
                        Usage: rpm <SFPM> <Diameter>\n
                        Example: rpm 90.0 1.5\n");
            }


            if input == "q" || input == "quit" {
                break;
            }
        }
    }
}
