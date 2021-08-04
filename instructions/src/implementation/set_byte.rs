use std::collections::HashMap;

use util_macros::*;

pub fn set_byte(memory: &mut [u8], arguments: &[&str], variables: &HashMap<String, usize>) {
    let position = get_argument_int_if_not_variable!(arguments, variables, 0);
    let value = get_argument_int_if_not_variable!(arguments, variables, 1) as u8;

    memory[position] = value;
}
