use std::collections::HashMap;

use utils::*;

pub fn copy_byte(memory: &mut [u8], arguments: &[&str], variables: &HashMap<String, usize>) {
    let from = get_argument_int_if_not_variable!(arguments, variables, 0);
    let to = get_argument_int_if_not_variable!(arguments, variables, 1);

    memory[to] = memory[from];
}
