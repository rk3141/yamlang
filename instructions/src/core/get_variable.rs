use std::collections::HashMap;

use util_macros::throw_runtime_error;

pub fn core_get_variable(
    variables: &HashMap<String, usize>,
    variable_name: String,
) -> Option<usize> {
    if let Some(&value) = variables.get(&variable_name) {
        Some(value)
    } else {
        None
    }
}
