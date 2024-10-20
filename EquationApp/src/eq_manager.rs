use crate::equation;
use std::fs;

pub struct EquationManager {
    next_free_id: u32,
    equations: Vec<equation::Equation>,
    file_location: String,
}