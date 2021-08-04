use std::collections::HashMap;

use util_macros::*;

use crate::core;

pub fn print_byte(memory: &mut [u8], arguments: &[&str], variables: &mut HashMap<String, usize>) {
    let value = get_argument!(arguments, 0);

    if value.starts_with("'") && value.ends_with("'") && value.len() == 3 {
        let byte = value.bytes().nth(1).unwrap();

        core::core_stdout_write_byte(memory, variables, byte);
    } else {
        let pos = core::core_get_variable(variables, value.to_string());
        if pos.is_some() {
            let pos = pos.unwrap();
            core::core_stdout_write_byte(memory, variables, memory[pos]);
        } else {
            throw_runtime_error!("Expected a byte to print in the form of 'y', 'a', 'm', etc.");
        }
    }
}
