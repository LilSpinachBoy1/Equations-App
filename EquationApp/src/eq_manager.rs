/*
EQUATION MANAGER - HOW IT WORKS:
1 - An instance is initialised in "main.rs" as the program is run
2 - The "load_from_file" function is run on instantiation to check if there are any equations stored in the file
3 - Other functions use the vec to manipulate and access the stored equations
4 - The "save_to_file" function should be run on close/save command or button
*/

use std::fs::File;
use std::io::Read;
use crate::equation::Equation;

pub struct EquationManager {
    next_free_id: u32,
    equations: Vec<Equation>,
    file_location: String,
}

impl EquationManager {
    pub fn new(file_location: String) -> Self {
        EquationManager {
            next_free_id: 0,
            equations: vec![],  // Create empty vec TODO: Change this to make use of the load function?
            file_location,
        }
    }

    // Func to read file contents to a string, and then add them to the "equations" attribute of the EquationManager struct
    pub fn load_from_file(&mut self) -> std::io::Result<()> {
        let mut file = File::open(&self.file_location)?;
        //TODO: adjust this code to access the details right, for now is just kind of testing it
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        println!("{}", contents);

        // Return "ok" if everything works alright!
        Ok(())
    }

    // Function to create a new equation and add it to the equation manager.
    pub fn add_equation(&mut self, variables: Vec<String>, equation: String, hint_description: String) -> None {
        let eq = Equation::new(self.next_free_id, variables, equation, hint_description);  // Create new equation
        self.next_free_id += 1;   // Increment next free ID, as that one has been taken
        self.equations.push(eq);  // Add new equation to "equations" vec
    }
}