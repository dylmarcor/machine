pub mod help {
    use std::io;

    // Help Functions
    pub fn display_help() {
        println!("Which function would you like help with?");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Could not read user input");

        loop {
            if input == "rpm" {
                println!("You must enter the SFPM and the Diameter of the material in inches, in that order.\n\n
                        Usage: rpm <SFPM> <Diameter>\n
                        Example: rpm 90.0 1.5\n");
            }
        }
    }
}