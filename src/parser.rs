use crate::token;
use crate::value_type;

use std::collections::{HashMap, VecDeque};
use std::iter::Peekable;
use std::slice::Iter;

/*
 * TODO 1: Implement parse(token_iter)
 * TODO 2: Implement parse_and_eval(token_iter, variables)
 */

pub fn parse(
    _token_iter: &mut Peekable<Iter<'_, (u32, token::Token)>>,
) -> Result<VecDeque<token::Token>, String> {
    unimplemented!();
}

pub fn parse_and_eval<'a>(
    _token_iter: &mut Peekable<Iter<'a, (u32, token::Token)>>,
    _variables: &HashMap<String, value_type::ValueType>,
) -> Result<value_type::ValueType, String> {
    unimplemented!();
}
