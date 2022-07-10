use crate::token;
use crate::value_type;

use std::collections::{HashMap, VecDeque};
use std::iter::Peekable;
use std::slice::Iter;

pub fn parse(
    token_iter: &mut Peekable<Iter<'_, (u32, token::Token)>>,
) -> Result<VecDeque<token::Token>, String> {
    let mut output_queue: VecDeque<token::Token> = VecDeque::new();
    let mut operator_stack: Vec<token::Token> = Vec::new();

    loop {
        match token_iter.peek() {
            Some(&&(_, token::Token::Then)) | None => break,
            _ => {}
        }

        match token_iter.next() {
            Some(&(_, ref value_token)) if value_token.is_value() => {
                output_queue.push_back(value_token.clone())
            }
            Some(&(_, ref operator_token)) if operator_token.is_operator() => {
                if !operator_stack.is_empty() {
                    let top_operator = operator_stack.last().unwrap().clone();
                    if top_operator.is_operator() {
                        let associativity = operator_token.operator_associativity().unwrap();

                        if (associativity == token::Associativity::Left
                            && operator_token.operator_precedence()
                                <= top_operator.operator_precedence())
                            || (associativity == token::Associativity::Right
                                && operator_token.operator_precedence()
                                    < top_operator.operator_precedence())
                        {
                            let top_operator = operator_stack.pop().unwrap();
                            output_queue.push_back(top_operator.clone());
                        }
                    }
                }

                operator_stack.push(operator_token.clone());
            }
            Some(&(_, token::Token::Comma)) => operator_stack.push(token::Token::Comma),
            Some(&(_, token::Token::Lparen)) => operator_stack.push(token::Token::Lparen),
            Some(&(_, token::Token::Rparen)) => loop {
                match operator_stack.pop() {
                    Some(token::Token::Lparen) => break,
                    Some(ref next_token) => output_queue.push_back(next_token.clone()),
                    None => return Err(String::from("ERR: Mismatched parenthesis in expression.")),
                }
            },
            _ => unreachable!(),
        }
    }

    while !operator_stack.is_empty() {
        match operator_stack.pop().unwrap() {
            token::Token::Lparen | token::Token::Rparen => {
                return Err(String::from("ERR: Mismatched parenthesis in expression."))
            }
            operator_token => output_queue.push_back(operator_token.clone()),
        }
    }

    Ok(output_queue)
}

pub fn parse_and_eval<'a>(
    token_iter: &mut Peekable<Iter<'a, (u32, token::Token)>>,
    variables: &HashMap<String, value_type::ValueType>,
) -> Result<value_type::ValueType, String> {
    match parse(token_iter) {
        Ok(mut output_queue) => {
            let mut stack: Vec<value_type::ValueType> = Vec::new();

            while !output_queue.is_empty() {
                match output_queue.pop_front() {
                    Some(token::Token::Number(ref number)) => {
                        stack.push(value_type::ValueType::Number(*number))
                    }
                    Some(token::Token::Text(ref text)) => {
                        stack.push(value_type::ValueType::Text(text.clone()))
                    }
                    Some(token::Token::Variable(ref name)) => match variables.get(name) {
                        Some(value) => stack.push(value.clone()),
                        None => {
                            return Err(format!(
                                "ERR: Invalid variable reference {} in expression.",
                                name
                            ))
                        }
                    },

                    Some(ref unary_token) if unary_token.is_unary_operator() => {
                        if !stack.is_empty() {
                            let value = stack.pop().unwrap();
                            let result = match *unary_token {
                                token::Token::UnaryMinus => -value,
                                token::Token::Bang => !value,

                                _ => unreachable!(),
                            };
                            match result {
                                Ok(value) => stack.push(value),
                                Err(error) => return Err(error),
                            }
                        } else {
                            return Err(format!("ERR: {:?} requires an operand.", unary_token));
                        }
                    }

                    Some(ref comparison_token) if comparison_token.is_comparison_operator() => {
                        if stack.len() >= 2 {
                            let operand_2 = &stack.pop().unwrap();
                            let operand_1 = &stack.pop().unwrap();

                            let result = match *comparison_token {
                                token::Token::Equals => operand_1 == operand_2,
                                token::Token::NotEqual => operand_1 != operand_2,
                                token::Token::LessThan => operand_1 < operand_2,
                                token::Token::GreaterThan => operand_1 > operand_2,
                                token::Token::LessThanEqual => operand_1 <= operand_2,
                                token::Token::GreaterThanEqual => operand_1 >= operand_2,

                                _ => unreachable!(),
                            };

                            stack.push(value_type::ValueType::Bool(result));
                        } else {
                            return Err(format!(
                                "ERR: {:?} Comparison operator requires two operands",
                                comparison_token
                            ));
                        }
                    }

                    Some(ref binary_operator_token)
                        if binary_operator_token.is_binary_operator() =>
                    {
                        if stack.len() >= 2 {
                            let operand_2 = stack.pop().unwrap();
                            let operand_1 = stack.pop().unwrap();

                            let result = match *binary_operator_token {
                                token::Token::Plus => operand_1 + operand_2,
                                token::Token::Minus => operand_1 - operand_2,
                                token::Token::Multiply => operand_1 * operand_2,
                                token::Token::Divide => operand_1 / operand_2,

                                _ => unreachable!(),
                            };

                            match result {
                                Ok(value) => stack.push(value),
                                Err(err) => return Err(err),
                            }
                        }
                    }

                    None => unreachable!(),
                    _ => unreachable!(),
                }
            }

            assert!(stack.len() == 1);
            Ok(stack[0].clone())
        }
        _ => Err(String::from("ERR: Invalid expression.")),
    }
}
