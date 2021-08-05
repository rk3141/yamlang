use std::collections::HashMap;

use runtime_constants::YamType;
use utils::get_argument;

use crate::core;

pub fn print_variable(
    memory: &mut [u8],
    arguments: &[&str],
    variables: &mut HashMap<String, usize>,
    variable_size_map: &HashMap<String, usize>,
    variable_types: &HashMap<String, YamType>,
) {
    let variable_name = get_argument!(arguments, 0).to_string();
    let variable = core::core_get_variable(&variables, variable_name.clone());

    if let Some(variable_position) = variable {
        let &size = variable_size_map.get(&variable_name).unwrap();
        let &type_of = variable_types.get(&variable_name).unwrap();

        match type_of {
            YamType::String => {
                let bytes = memory[(variable_position)..(variable_position + size)].to_vec();

                for byte in bytes {
                    core::core_stdout_write_byte(memory, variables, byte);
                }
            }

            _ => {}
        };
    }
}
