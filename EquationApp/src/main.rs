// Add structures
mod equation;
mod eq_manager;

fn main() {
    // Create P=IV equation
    let vars = vec![String::from("P"), String::from("I"), String::from("V")];
    let equation = String::from("P=IV");
    let hint = String::from("The equation linking PD, current and power");
    let eq_piv = equation::Equation::new(0, vars, equation, hint);

    // Output PIV
    println!("{}", eq_piv.get_attribute(String::from("hint_description")));
}
