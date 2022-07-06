use crate::lexer;
use crate::parser;
use crate::token;
use crate::value_type;

use std::collections::BTreeMap;
use std::collections::HashMap;

pub fn interpret(instructions: Vec<lexer::Instruction>) -> Result<String, String> {
    let mut variables: HashMap<String, value_type::ValueType> = HashMap::new();
    let mut instruction_ids = BTreeMap::new();
    let mut instruction_map = BTreeMap::new();

    for (index, instruction) in instructions.iter().enumerate() {
        instruction_map.insert(&instruction.line_number, index);
        instruction_ids.insert(&instruction.line_number, &instruction.tokens);
    }

    let instruction_numbers: Vec<_> = instruction_map.keys().clone().collect();
    let instruction_amount = instruction_numbers.len();

    let mut instruction_index = 0;
    let mut instruction_goto = false;

    loop {
        let instruction_number = instruction_numbers[instruction_index];
        let tokens = &instruction_ids[instruction_number];
        let mut token_iter = tokens.iter().peekable();

        if !tokens.is_empty() {
            let (position, ref token) = *token_iter.next().unwrap();
            instruction_goto = false;

            match *token {
                token::Token::Print => match parser::parse_and_eval(&mut token_iter, &variables) {
                    Ok(value_type::ValueType::Text(x)) => println!("{}", x),
                    Ok(value_type::ValueType::Number(x)) => println!("{}", x),
                    Ok(value_type::ValueType::Bool(x)) => println!("{}", x),
                    Err(_) => {
                        return Err(format!(
                            "ERR [{:?} | {}]: PRINT must be followed by a valid expression.",
                            instruction_number, position
                        ))
                    }
                },
                token::Token::Input => match token_iter.next() {
                    Some(&(_, token::Token::Variable(ref variable))) => {
                        let mut buffer = String::new();

                        std::io::stdin()
                            .read_line(&mut buffer)
                            .expect("Failed to read from STDIN.");
                        buffer = buffer.trim().to_string();
                        let value = value_type::ValueType::Text(buffer);

                        variables
                            .entry(variable.clone().to_string())
                            .or_insert(value);
                    }

                    _ => {
                        return Err(format!(
                            "ERR [{:?} | {}]: INPUT must be followed by an identifier",
                            instruction_number,
                            position + 5
                        ))
                    }
                },
                token::Token::Let => {
                    match (
                        token_iter.next(),
                        token_iter.next(),
                        parser::parse_and_eval(&mut token_iter, &variables),
                    ) {
                        (
                            Some(&(_, token::Token::Variable(ref variable))),
                            Some(&(_, token::Token::Equals)),
                            Ok(ref value),
                        ) => {
                            variables.insert(variable.clone().to_string(), value.clone());
                        }

                        (_, _, Err(e)) => {
                            return Err(format!(
                                "ERR [{:?} | {}]: LET {}.",
                                instruction_number, position, e
                            ))
                        }

                        _ => {
                            return Err(format!(
                                "ERR [{:?} | {}]: Invalid syntax for LET.",
                                instruction_number, position
                            ))
                        }
                    }
                }
                token::Token::If => {
                    match (
                        parser::parse_and_eval(&mut token_iter, &variables),
                        token_iter.next(),
                        token_iter.next(),
                    ) {
                        (
                            Ok(value_type::ValueType::Bool(ref value)),
                            Some(&(_, token::Token::Then)),
                            Some(&(_, token::Token::Number(ref number))),
                        ) => {
                            if *value {
                                instruction_goto = true;
                                let line_number = *number as u32;
                                match instruction_map.get(&line_number) {
                                    Some(index) => instruction_index = *index,
                                    _ => {
                                        return Err(format!(
                                            "ERR [{:?} | {}]: Invalid target for IF.",
                                            instruction_number, position
                                        ))
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(format!(
                                "ERR [{:?} | {}]: Invalid syntax for IF.",
                                instruction_number, position
                            ))
                        }
                    }
                }
                token::Token::Goto => {
                    instruction_goto = true;
                    match token_iter.next() {
                        Some(&(position, token::Token::Number(number))) => {
                            let line_number = number as u32;
                            match instruction_map.get(&line_number) {
                                Some(index) => instruction_index = *index,
                                _ => {
                                    return Err(format!(
                                        "ERR [{:?} | {}]: Invalid target for GOTO",
                                        instruction_number, position
                                    ))
                                }
                            }
                        }
                        Some(&(position, _)) => {
                            return Err(format!(
                                "ERR [{:?} | {}]: GOTO must be followed by a valid line number.",
                                instruction_number, position
                            ))
                        }
                        None => {
                            return Err(format!(
                                "ERR [{:?} | {}]: GOTO must be followed by a line number.",
                                instruction_number,
                                position + 4
                            ))
                        }
                    }
                }
                token::Token::Rem => {}
                _ => {
                    return Err(format!(
                        "ERR [{:?} | {}]: Invalid syntax.",
                        instruction_number,
                        position + 4
                    ))
                }
            }
        }

        if !instruction_goto {
            instruction_index += 1;
            if instruction_index == instruction_amount {
                break;
            }
        }
    }

    Ok(String::from("PROGRAM ran successfully."))
}
