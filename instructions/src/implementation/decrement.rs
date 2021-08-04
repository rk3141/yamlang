use std::collections::HashMap;

use util_macros::*;

pub fn decrement(memory: &mut [u8], arguments: &[&str], variables: &mut HashMap<String, usize>) {
    let what = get_argument!(arguments, 0);

    if let Ok(address) = what.parse::<usize>() {
        memory[address] -= 1;
    } else {
        let &variable_location = variables
            .get(&what.to_string())
            .expect(format!("variable `{}` doesnt exist", what).as_str());

        memory[variable_location] -= 1;
    }
}
