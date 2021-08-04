use std::collections::HashMap;

pub fn core_set_variable(
    variables: &mut HashMap<String, usize>,
    variable_name: String,
    value: usize,
) {
    variables.insert(variable_name, value);
}
