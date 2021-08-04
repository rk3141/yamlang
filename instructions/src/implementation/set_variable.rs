use std::collections::HashMap;

use util_macros::*;

use crate::core;

pub fn set_variable(
    memory: &mut [u8],
    arguments: &[&str],
    variables: &mut HashMap<String, usize>,
    empty_spot: &mut usize,
) {
    let variable_name = get_argument!(arguments, 0);
    let value = get_argument_int_if_not_variable!(arguments, variables, 1);

    memory[*empty_spot] = value as u8;
    *empty_spot += 1;

    core::core_set_variable(variables, variable_name.to_string(), *empty_spot - 1);
}
