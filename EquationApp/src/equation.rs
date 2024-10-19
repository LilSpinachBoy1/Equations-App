// Structure to hold an Equation
pub struct Equation {
    variables: Vec<String>,
    equation: String,
    hint_description: String,
}

// Implementation of Equation structure
impl Equation {
    // __init__, creates Equation struct
    pub fn new(variables: Vec<String>, equation: String, hint_description: String) -> Self {
        Equation {
            variables,
            equation,
            hint_description,
        }
    }

    // Function to output attributes
    pub fn get_equation(&self) -> String {self.equation.clone()}
    pub fn get_hint_description(&self) -> String {self.hint_description.clone()}
    pub fn get_variables(&self) -> String {self.variables.clone().join(", ")}
}
