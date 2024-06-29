use std::collections::HashMap;

fn main() {

    // Get arguments
    let args = get_args();

    // Original file
    let original_file = File {
        filename: args.args.iter()
            .nth(1)
            .expect("No output file specified")
            .to_string(),
        file_extension: args.patterns.get("--from")
            .unwrap_or(
                &args.args.iter()
                    .nth(1)
                    .expect("No input file specified")
                    .split(".")
                    .last()
                    .expect("Could not infer input file type, please specify using --from <type>")
                    .to_string()
            )
            .to_string(),
    };

    // New file
    let new_file = File {
        filename: args.args.iter()
            .nth(2)
            .expect("No output file specified")
            .to_string(),
        file_extension: args.patterns.get("--to")
            .unwrap_or(
                &args.args.iter()
                    .nth(2)
                    .expect("No input file specified")
                    .split(".")
                    .last()
                    .expect("Could not infer output file type, please specify using --to <type>")
                    .to_string()
            )
            .to_string(),
    };

    println!("from: {:?}, format: {:?}", original_file.filename, original_file.file_extension);
    println!("to: {:?}, format: {:?}", new_file.filename, new_file.file_extension);
}

fn get_args() -> Arguments {
    let mut args = Arguments {
        args: Vec::new(),
        patterns: HashMap::new(),
    };

    let mut skip_next = false;
    for (i, arg) in std::env::args().enumerate() {
        if skip_next {
            skip_next = false;
            continue;
        }

        if arg.starts_with("-") {
            args.patterns.insert(
                arg,
                std::env::args()
                    .nth(i + 1)
                    .expect("Pattern not follow up by value")
            );
            skip_next = true;
            continue;
        }

        args.args.push(arg);
    }

    return args;
}

struct Arguments {
    args: Vec<String>,
    patterns: HashMap<String, String>,
}

struct File {
    filename: String,
    file_extension: String,
}