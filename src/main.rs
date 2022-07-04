use sbasic::lexer;

use std::env;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(path)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn main() {
    let mut argv = env::args();
    if env::args().len() != 2 {
        println!("Usage: sBASIC [FILE]");
    } else {
        let file_name: String = argv.nth(1).unwrap();
        let program = read_file(file_name.as_str());
        match program {
            Ok(source) => {
                let mut instructions: Vec<lexer::Instruction> = Vec::new();
                for (line_number, line) in source.lines().enumerate() {
                    let tokenized_line = lexer::tokenize_line(line);

                    match tokenized_line {
                        Ok(x) => instructions.push(x),
                        Err(err) => println!("Error at line {}: {}", line_number, err),
                    }
                }
            }
            Err(error) => println!("ERR: {}", error),
        }
    }
}
