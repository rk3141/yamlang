use std::collections::HashMap;

use super::{core_get_variable, core_set_variable};

pub fn core_stdout_write_byte(memory: &mut [u8], variables: &mut HashMap<String, usize>, byte: u8) {
    let current_stdout = core_get_variable(variables, "std.stdout".to_string()).unwrap();

    memory[current_stdout] = byte;
    core_set_variable(variables, "std.stdout".to_string(), current_stdout + 1);
}
