use bin::*;
use std::{collections::HashMap, iter::Peekable};

fn print_stdout(memory: &[u8]) {
    for byte in &memory[MEMSIZE..] {
        if *byte == 0 {
            break;
        }
        print!("{}", *byte as char);
    }
}

fn main() {
    let asm = std::fs::read_to_string("main.yam").unwrap();

    let mut memory = [0u8; MEMSIZE + STDOUT_SIZE];

    let mut variables: HashMap<String, usize> = HashMap::new();
    let mut variable_types: HashMap<String, YamType> = HashMap::new();
    let mut variable_offset_map: HashMap<String, usize> = HashMap::new();

    let mut empty_spot = 0;

    variables.insert("std.stdout".to_string(), MEMSIZE);

    for (_line, instruction) in asm.split("\n").enumerate() {
        if instruction == "" {
            continue;
        }

        let chunks = instruction.split(" ").collect::<Vec<&str>>();

        let (&keyword, arguments) = chunks.split_first().unwrap();

        let mut new_argument = vec![];
        let mut current = String::new();

        let mut found_quote = false;

        for piece in arguments {
            current += piece;
            if piece == &"'" {
                current += " '";
                found_quote = true;
            }

            new_argument.push(current);
            current = String::new();

            if new_argument.len() == 1 && found_quote {
                break;
            }
        }

        let arguments = new_argument
            .iter()
            .map(|v| v.as_str())
            .collect::<Vec<&str>>();
        let arguments = arguments.as_slice();

        match keyword {
            "seb" => set_byte(&mut memory, arguments, &variables),

            "cpy" => copy_byte(&mut memory, arguments, &variables),

            "inc" => increment(&mut memory, arguments, &mut variables),

            "dec" => decrement(&mut memory, arguments, &mut variables),

            "var" => set_variable(
                &mut memory,
                arguments,
                &mut variables,
                &mut empty_spot,
                &mut variable_offset_map,
            ),

            "print_byte" => {
                print_byte(&mut memory, arguments, &mut variables);
            }

            "!" => {
                // Ignore comment
            }

            _ => {
                eprintln!("Debug: {:?}", memory);
                panic!("whats that keyword: {:?}", keyword);
            }
        }
    }

    print_stdout(&memory);
}
