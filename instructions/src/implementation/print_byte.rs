use std::collections::HashMap;

use util_macros::*;

use crate::core;

pub fn print_byte(memory: &mut [u8], arguments: &[&str], variables: &mut HashMap<String, usize>) {
    let value = get_argument!(arguments, 0);

    if value.starts_with("'") && value.ends_with("'") && value.len() > 2 {
        let mut byte = value.bytes().nth(1).unwrap();

        if value.len() == 4 {
            if byte == b'\\' {
                let modifier = value.bytes().nth(2).unwrap();

                byte = match modifier {
                    b'a' => 7,
                    b'b' => 8,
                    b't' => 9,
                    b'n' => 10,
                    b'r' => 13,

                    _ => throw_runtime_error!(
                        "I dont know what is this modifier `\\{}`",
                        modifier as char
                    ),
                };
            }
        }
        if value.len() > 4 {
            throw_runtime_error!("What is this mess of a charecter `{}` ):(", value);
        }

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
