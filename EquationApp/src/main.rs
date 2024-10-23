use crate::eq_manager::EquationManager;

// Add structures
mod equation;
mod eq_manager;

fn main() {
    // Create EquationManager
    let file_addr = String::from("src/equations.txt");  // Base directory is same as project, fuck you python
    let mut manager = EquationManager::new(file_addr);

    // Create P=IV equation
    let vars = vec![String::from("P"), String::from("I"), String::from("V")];
    let equation = String::from("P=IV");
    let hint = String::from("The equation linking PD, current and power");
    manager.add_equation(vars, equation, hint);

    // Test writing to file (this aint gunna work is it)
    match manager.write_to_file() {
        Ok(()) => println!("File written!"),
        Err(e) => println!("Error: {}", e),
    }
}
