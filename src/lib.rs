

use std::{fs::File, io::Write};

use proc_macro::TokenStream;
use quote::quote;
use rand::{distributions::Alphanumeric, Rng};
use syn::{parse_macro_input, ItemFn, parse_str, Block};

#[proc_macro_attribute]
pub fn obfuscate(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);

    // Insert the generated dead code
    let dead_code = generate_dead_code().stmts;
    let new_stmts = dead_code.into_iter().chain(input.block.stmts.into_iter()).collect();
    input.block.stmts = new_stmts;

    // Return the new function
    TokenStream::from(quote! { #input })
}

// Your existing dead code generation code would go here, modified to return Block instead of syn::Block
// Your existing dead code generation code would go here, modified to return Block instead of syn::Block
fn generate_dead_code() -> Block {
    let mut rng = rand::thread_rng();

    let num_functions = rng.gen_range(10000..10010);
    let mut code = String::from("{\n");
    let mut generated_functions = Vec::new();

    // Generate random functions
    for i in 0..num_functions {
        let function_name: String = std::iter::once(
            rng.gen_range(b'a'..=b'z').into()
        )
        .chain((&mut rng).sample_iter(&Alphanumeric).take(9).map(char::from))
        .collect();

        if is_keyword(&function_name) {
            continue;
        }

        generated_functions.push(function_name.clone());

        let operation = match rng.gen_range(0..1) {
            0 => "+",
            1 => "-",
            2 => "*",
            _ => "/", 
        };

        let function_declaration = if rng.gen_bool(0.5) && i != 0 {
            let previous_function_index = rng.gen_range(0..i);
            let previous_function_name = &generated_functions[previous_function_index];
            format!("
                fn {}(x: usize, y: usize) -> usize {{
                    let result = {}(x, y);
                    result {} 1
                }}\n", function_name, previous_function_name, operation)
        } else {
            format!("
                fn {}(x: usize, y: usize) -> usize {{
                    x {} y
                }}\n", function_name, operation)
        };

        code.push_str(&function_declaration);
    }

    // Generate complex control structures
    for _ in 0..rng.gen_range(0..50) {
        let index = rng.gen_range(0..generated_functions.len());
        let function_name = &generated_functions[index];

        let arg1 = (&mut rng).gen_range(0..100);
        let arg2 = (&mut rng).gen_range(0..100);

        let control_structure = format!("
            let mut a = 0;
            for _ in 0..100 {{
                a = {}(a, {} + {});
                if a < 50 {{
                    for _ in 0..50 {{
                        a = a.checked_add(1).unwrap_or(a);
                    }}
                }} else {{
                    for _ in 0..50 {{
                        if a % 2 == 0 {{
                            a = a.checked_sub(1).unwrap_or(a);
                        }} else {{
                            a = a.checked_add(1).unwrap_or(a);
                        }}
                    }}
                }}
            }}\n", function_name, arg1, arg2);

        code.push_str(&control_structure);
    }

    code.push_str("}\n");

    let mut file = File::create("new.rs").unwrap();
    file.write_all(code.as_bytes()).unwrap();
    let dead_code: Block = parse_str(&code).expect("Unable to parse block");

    dead_code
}


// Helper function to check if a string is a Rust keyword
fn is_keyword(s: &str) -> bool {
    [
        "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
        "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
        "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
        "use", "where", "while", "async", "await", "dyn", "abstract", "become", "box", "do",
        "final", "macro", "override", "priv", "typeof", "unsized", "virtual", "yield",
    ].contains(&s)
}
