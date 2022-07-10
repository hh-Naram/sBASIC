use crate::token;

use itertools::Itertools;
use std::str::FromStr;

pub struct Instruction {
    pub line_number: u32,
    pub tokens: Vec<(u32, token::Token)>,
}

fn is_valid(token_string: &str) -> bool {
    let mut characters = token_string.chars();
    let character = characters.next();

    match character {
        Some(character) => match character {
            'a'..='z' | 'A'..='Z' => (),
            _ => return false,
        },
        None => return false,
    }

    for character in characters {
        match character {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => (),
            _ => return false,
        }
    }

    true
}

pub fn tokenize_line(line: &str) -> Result<Instruction, String> {
    let mut char_iterator = line.chars().enumerate().peekable();
    let mut line_number = 0u32;
    let mut tokens: Vec<(u32, token::Token)> = Vec::new();

    while char_iterator.peek() != None {
        let (position, character) = char_iterator.next().unwrap();

        if position == 0 {
            if character.is_numeric() {
                let mut number_chars: Vec<char> = char_iterator
                    .by_ref()
                    .take_while(|&(_, x)| !x.is_whitespace())
                    .map(|(_, x)| x)
                    .collect();
                number_chars.insert(0, character);
                let number_string: String = number_chars.into_iter().collect();

                match u32::from_str(number_string.as_str()) {
                    Ok(number) => line_number = number,
                    Err(_) => {
                        return Err(format!(
                            "Lines Must start with a number followed by a whitespace:\n\t{}",
                            line
                        ))
                    }
                }
            } else {
                return Err(format!("Lines must start with a number:\n\t{}", line));
            }
        } else {
            match character {
                character if character.is_whitespace() => {
                    continue;
                }

                '"' => {
                    let string_chars: Vec<char> = char_iterator
                        .by_ref()
                        .take_while(|&(_, x)| x != '"')
                        .map(|(_, x)| x)
                        .collect();
                    let text: String = string_chars.into_iter().collect();
                    tokens.push((position.try_into().unwrap(), token::Token::Text(text)));
                }
                '-' => {
                    if !tokens.is_empty() && tokens.last().unwrap().1.is_value() {
                        tokens.push((position.try_into().unwrap(), token::Token::Minus));
                    } else {
                        tokens.push((position.try_into().unwrap(), token::Token::UnaryMinus));
                    }
                }
                '!' => tokens.push((position.try_into().unwrap(), token::Token::Bang)),
                '(' => tokens.push((position.try_into().unwrap(), token::Token::Lparen)),
                ')' => tokens.push((position.try_into().unwrap(), token::Token::Rparen)),
                ',' => tokens.push((position.try_into().unwrap(), token::Token::Comma)),
                _ => {
                    let mut token_chars: Vec<char> = char_iterator
                        .by_ref()
                        .peeking_take_while(|&(_, x)| !(x.is_whitespace() || x == ')'))
                        .map(|(_, x)| x)
                        .collect();
                    token_chars.insert(0, character);
                    let token_string: String = token_chars.into_iter().collect();

                    if i32::from_str(token_string.as_str()).is_ok() {
                        tokens.push((
                            position.try_into().unwrap(),
                            token::Token::Number(i32::from_str(token_string.as_str()).unwrap()),
                        ));
                    } else {
                        let token = token::Token::to_token(token_string.as_str());

                        match token {
                            None => {
                                if is_valid(&token_string) {
                                    tokens.push((
                                        position.try_into().unwrap(),
                                        token::Token::Variable(token_string.to_string()),
                                    ))
                                } else {
                                    return Err(format!(
                                        "Unimplimented token at {}: \t{}",
                                        position, token_string
                                    ));
                                }
                            }

                            Some(token::Token::Rem) => {
                                tokens.push((position.try_into().unwrap(), token::Token::Rem));
                                char_iterator.next();

                                let comment_string: String =
                                    char_iterator.by_ref().map(|(_, x)| x).collect();
                                tokens.push((
                                    (position + 4) as u32,
                                    token::Token::Comment(comment_string),
                                ));
                            }

                            Some(token) => {
                                tokens.push((position.try_into().unwrap(), token));
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(Instruction {
        line_number,
        tokens,
    })
}
