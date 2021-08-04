use std::collections::HashMap;

use util_macros::*;

pub fn set_variable(memory: &mut [u8], arguments: &[&str], variables: &mut HashMap<String, usize>) {
    let variable_name = get_argument!(arguments, 0);
    let value = get_argument_int_if_not_variable!(arguments, variables, 1);

    variables.insert(variable_name.to_string(), value);
}
