use std::collections::HashMap;

use runtime_constants::*;
use util_macros::*;

use crate::core;

pub fn set_byte(memory: &mut [u8], arguments: &[&str], variables: &HashMap<String, usize>) {
    let position = get_argument_int_if_not_variable!(arguments, variables, 0);

    let value = get_argument_int_if_not_variable!(arguments, variables, 1) as u8;

    if position > MEMSIZE {
        throw_runtime_error!(
            "You cant access memory beyond {} (allocated memory)",
            MEMSIZE
        );
    }

    core::core_set_byte(memory, position, value);
}
