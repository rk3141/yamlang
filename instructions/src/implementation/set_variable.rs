use std::collections::HashMap;

use runtime_constants::YamType;
use utils::*;

use crate::core;

enum Either<T, U> {
    Is(T),
    Or(U),
}

pub fn set_variable(
    memory: &mut [u8],
    arguments: &[&str],
    variables: &mut HashMap<String, usize>,
    empty_spot: &mut usize,
    variable_offset_map: &mut HashMap<String, usize>,
) {
    let variable_name = get_argument!(arguments, 0);

    let value = get_argument!(arguments, 1);

    let key_for_variables = value;

    let mut value = Either::Is(value);

    if variables.get(&key_for_variables.to_string()).is_some() {
        value = Either::Or(get_variable!(variables, key_for_variables));
    }

    match value {
        Either::Is(other) => {
            let (type_of, data) = parse_misc_data_type(other);

            match type_of {
                YamType::String => {
                    let size = data.len();

                    for (k, &byte) in data.iter().enumerate() {
                        memory[*empty_spot] = byte;
                        *empty_spot += 1;
                    }

                    variable_offset_map.insert(key_for_variables.to_string(), size);
                }
                _ => return,
            };
        }

        Either::Or(&value) => {
            memory[*empty_spot] = value as u8;
            *empty_spot += 1;

            variable_offset_map.insert(key_for_variables.to_string(), 1);
        }
    };

    core::core_set_variable(variables, variable_name.to_string(), *empty_spot - 1);
}

fn parse_misc_data_type(data: &str) -> (YamType, Vec<u8>) {
    if data.starts_with("\"") && data.ends_with("\"") {
        let chars = data.chars().map(|ch| ch as u8).collect::<Vec<u8>>();

        (YamType::String, chars)
    } else {
        (YamType::Null, vec![])
    }
}
