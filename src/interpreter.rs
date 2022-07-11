use crate::lexer;
use crate::parser;
use crate::renderer;
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

    let mut program: renderer::Renderer = renderer::Renderer::default();

    let mut graphics = false;
    let mut running = true;

    while running {
        if graphics {
            program.update(&mut running);
        }

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
                        let mut input = String::new();

                        std::io::stdin()
                            .read_line(&mut input)
                            .expect("failed to read line");
                        input = input.trim().to_string();
                        let value = value_type::ValueType::Text(input);
                        variables.insert(variable.clone().to_string(), value);
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

                token::Token::Screen => {
                    match (token_iter.next(), token_iter.next(), token_iter.next()) {
                        (
                            Some(&(_, token::Token::Number(width))),
                            Some(&(_, token::Token::Comma)),
                            Some(&(_, token::Token::Number(height))),
                        ) => {
                            program.set_size(width, height);
                            program.window.show();

                            graphics = true;
                        }
                        _ => {
                            return Err(format!(
                                "ERR [{:?} | {}]: Invalid syntax for SCREEN.",
                                instruction_number, position,
                            ))
                        }
                    }
                }

                token::Token::Clear => {
                    match (
                        token_iter.next(),
                        token_iter.next(),
                        token_iter.next(),
                        token_iter.next(),
                        token_iter.next(),
                    ) {
                        (
                            Some(&(_, token::Token::Number(red))),
                            Some(&(_, token::Token::Comma)),
                            Some(&(_, token::Token::Number(green))),
                            Some(&(_, token::Token::Comma)),
                            Some(&(_, token::Token::Number(blue))),
                        ) => {
                            program.render_clear(red, green, blue);
                        }
                        _ => {
                            return Err(format!(
                                "ERR [{:?} | {}]: Invalid syntax for CLEAR.",
                                instruction_number, position,
                            ))
                        }
                    }
                }

                token::Token::Dot => {
                    match (token_iter.next(), token_iter.next(), token_iter.next()) {
                        (
                            Some(&(_, token::Token::Number(x))),
                            Some(&(_, token::Token::Comma)),
                            Some(&(_, token::Token::Number(y))),
                        ) => {
                            program.render_dot(x, y);
                        }
                        _ => {
                            return Err(format!(
                                "ERR [{:?} | {}]: Invalid syntax for DOT.",
                                instruction_number, position,
                            ))
                        }
                    }
                }

                token::Token::Line => {
                    match (
                        token_iter.next(),
                        token_iter.next(),
                        token_iter.next(),
                        token_iter.next(),
                        token_iter.next(),
                        token_iter.next(),
                        token_iter.next(),
                    ) {
                        (
                            Some(&(_, token::Token::Number(x1))),
                            Some(&(_, token::Token::Comma)),
                            Some(&(_, token::Token::Number(y1))),
                            Some(&(_, token::Token::To)),
                            Some(&(_, token::Token::Number(x2))),
                            Some(&(_, token::Token::Comma)),
                            Some(&(_, token::Token::Number(y2))),
                        ) => {
                            program.render_line(x1, y1, x2, y2);
                        }
                        _ => {
                            return Err(format!(
                                "ERR [{:?} | {}]: Invalid syntax for DOT.",
                                instruction_number, position,
                            ))
                        }
                    }
                }

                token::Token::Circle => {
                    match (
                        token_iter.next(), //x
                        token_iter.next(), //,
                        token_iter.next(), //y
                        token_iter.next(), //,
                        token_iter.next(), //r
                    ) {
                        (
                            Some(&(_, token::Token::Number(x))),
                            Some(&(_, token::Token::Comma)),
                            Some(&(_, token::Token::Number(y))),
                            Some(&(_, token::Token::Comma)),
                            Some(&(_, token::Token::Number(r))),
                        ) => {
                            program.render_circle(x, y, r);
                        }
                        _ => {
                            return Err(format!(
                                "ERR [{:?} | {}]: Invalid syntax for DOT.",
                                instruction_number, position,
                            ))
                        }
                    }
                }

                token::Token::Rem => {}
                token::Token::End => running = false,
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
