// Structure to hold an Equation
pub struct Equation {
    id: u32,
    variables: Vec<String>,
    equation: String,
    hint_description: String,
}

// Implementation of Equation structure
impl Equation {
    // __init__, creates Equation struct
    pub fn new(id: u32, variables: Vec<String>, equation: String, hint_description: String) -> Self {
        Equation {
            id,
            variables,
            equation,
            hint_description,
        }
    }

    // Improved get attribute func
    pub fn get_attribute(&self, attr: String) -> String {
        if attr.to_string() == String::from("variables") {
            self.variables.clone().join(", ")
        } else if attr.to_string() == String::from("equation") {
            self.equation.clone()
        } else if attr.to_string() == String::from("hint_description") {
            self.hint_description.clone()
        } else {
            String::from("FAIL")
        }
    }
}
