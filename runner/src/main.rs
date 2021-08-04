use runner::*;
use std::collections::HashMap;

const MEMSIZE: usize = 1024;
const STDOUT_LOC: usize = 896;

fn print_stdout(memory: &[u8]) {
    for byte in &memory[STDOUT_LOC..] {
        if *byte == 0 {
            break;
        }
        print!("{}", *byte as char);
    }
}

fn main() {
    let asm = std::fs::read_to_string("main.yam").unwrap();

    let mut memory = [0u8; MEMSIZE];
    let mut variables: HashMap<String, usize> = HashMap::new();

    for instruction in asm.split("\n") {
        if instruction == "" {
            continue;
        }

        let chunks = instruction.split(" ").collect::<Vec<&str>>();

        let (&keyword, arguments) = chunks.split_first().unwrap();

        match keyword {
            "seb" => {
                set_byte(&mut memory, arguments, &variables);
            }

            "cpy" => {
                copy_byte(&mut memory, arguments, &variables);
            }

            "inc" => increment(&mut memory, arguments, &mut variables),

            "dec" => decrement(&mut memory, arguments, &mut variables),

            "var" => set_variable(&mut memory, arguments, &mut variables),

            "!" => {
                // Ignore comment
            }

            _ => {
                eprintln!("Debug: {:?}", memory);
                panic!("whats that keyword: {:?}", keyword);
            }
        }
    }

    // eprintln!("Debug: {:?}", memory);

    print_stdout(&memory);
}
