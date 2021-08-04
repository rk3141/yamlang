use runner::*;
use std::collections::HashMap;

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

    variables.insert("std.stdout".to_string(), MEMSIZE);

    for instruction in asm.split("\n") {
        if instruction == "" {
            continue;
        }

        let chunks = instruction.split(" ").collect::<Vec<&str>>();

        let (&keyword, arguments) = chunks.split_first().unwrap();

        match keyword {
            "seb" => set_byte(&mut memory, arguments, &variables),

            "cpy" => copy_byte(&mut memory, arguments, &variables),

            "inc" => increment(&mut memory, arguments, &mut variables),

            "dec" => decrement(&mut memory, arguments, &mut variables),

            "var" => set_variable(arguments, &mut variables),

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
