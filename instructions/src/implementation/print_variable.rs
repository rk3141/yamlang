use std::collections::HashMap;

use runtime_constants::YamType;

use crate::core;

pub fn print_variable(
    memory: &mut [u8],
    variables: &mut HashMap<String, usize>,
    variable_locations: &HashMap<String, usize>,
    variable_types: &HashMap<String, YamType>,
    variable_name: String,
) {
    let variable = core::core_get_variable(&variables, variable_name.clone());

    if let Some(variable_position) = variable {
        // let &size = variable_sizes.get(&variable_name).unwrap();

        // let data = memory[variable_position..(variable_position + size)].to_owned();

        // let &variable_type = variable_types.get(&variable_name).unwrap();

        // match variable_type {
        //     YamType::String => {
        //         for byte in data {
        //             core::core_stdout_write_byte(memory, variables, byte);
        //         }
        //     }
        //     _ => {}
        // }
    }
}
