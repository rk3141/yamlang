use std::collections::HashMap;

pub fn core_set_variable(
    variables: &mut HashMap<String, usize>,
    variable_name: &str,
    value: usize,
) {
    variables.insert(variable_name.to_string(), value);
}
