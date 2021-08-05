use std::collections::HashMap;

use runtime_constants::YamType;
use utils::*;

use crate::core;

pub fn set_variable(
    memory: &mut [u8],
    arguments: &[&str],
    variables: &mut HashMap<String, usize>,
    empty_spot: &mut usize,
    variable_size_map: &mut HashMap<String, usize>,
    variable_types: &mut HashMap<String, YamType>,
) {
    let variable_name = get_argument!(arguments, 0);

    let mut value = get_argument!(arguments, 1).to_string();

    let key_for_variables = value.clone();

    if variables.get(&key_for_variables.to_string()).is_some() {
        let target = get_variable!(variables, key_for_variables);
        for (name, location) in variables.iter() {
            if target == location {
                let name = name.clone();
                value = name;
            }
        }
    }

    let (type_of, data) = parse_misc_data_type(value.as_str());

    let offset = match type_of {
        YamType::String => {
            let mut byte_iter = data.iter();

            let mut counter = 0;
            let mut escape = false;

            for &byte in byte_iter.clone() {
                *empty_spot += 1;

                memory[*empty_spot - 1] = if byte == b'\\' {
                    escape = true;
                    let byte = match byte_iter.nth(counter + 1).expect("Missing modifier") {
                        b't' => b'\t',
                        b'n' => b'\n',
                        b'r' => b'\r',
                        b'\\' => b'\\',

                        modifier => {
                            throw_runtime_error!("Unknown modifier {:?}", modifier);
                        }
                    };
                    counter += 1;
                    byte
                } else {
                    if escape {
                        *empty_spot -= 1;
                        counter -= 1;
                        escape = false;
                        continue;
                    };
                    byte
                };

                counter += 1;
            }

            counter
        }

        YamType::Uint8 => {
            memory[*empty_spot] = *data.iter().nth(0).unwrap();
            *empty_spot += 1;

            1
        }

        _ => return,
    };

    variable_size_map.insert(variable_name.to_string(), offset);
    variable_types.insert(variable_name.to_string(), type_of);

    core::core_set_variable(variables, variable_name.to_string(), *empty_spot - offset);
}

fn parse_misc_data_type(data: &str) -> (YamType, Vec<u8>) {
    if data.starts_with("\"") && data.ends_with("\"") {
        let chars = data.chars().map(|ch| ch as u8).collect::<Vec<u8>>();
        let chars = chars[1..(chars.len() - 1)].to_vec();

        (YamType::String, chars)
    } else if let Ok(result) = data.parse::<u8>() {
        (YamType::Uint8, vec![result])
    } else {
        (YamType::Null, vec![])
    }
}
