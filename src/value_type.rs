use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::ops::{Add, Div, Mul, Neg, Not, Sub};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum ValueType {
    Text(String),
    Number(i32),
    Bool(bool),
}

impl Add for ValueType {
    type Output = Result<ValueType, String>;

    fn add(self, other: ValueType) -> Self::Output {
        match (self, other) {
            (ValueType::Number(rh), ValueType::Number(lh)) => Ok(ValueType::Number(rh + lh)),
            (ValueType::Text(rh), ValueType::Text(lh)) => {
                Ok(ValueType::Text(format!("{}{}", rh, lh)))
            }
            (ValueType::Text(rh), ValueType::Number(lh)) => {
                let number_string = i32::from_str(rh.as_str());

                if let Result::Ok(number) = number_string {
                    Ok(ValueType::Number(number + lh))
                } else {
                    Err(format!("Cannot add string: {} to number: {}.", rh, lh))
                }
            }
            (ValueType::Number(rh), ValueType::Text(lh)) => {
                let number_string = i32::from_str(lh.as_str());

                if let Result::Ok(number) = number_string {
                    Ok(ValueType::Number(number + rh))
                } else {
                    Err(format!("Cannot add string: {} to number: {}.", lh, rh))
                }
            }
            _ => Err(String::from("Can only add numbers and strings.")),
        }
    }
}

impl Sub for ValueType {
    type Output = Result<ValueType, String>;

    fn sub(self, other: ValueType) -> Self::Output {
        match (self, other) {
            (ValueType::Number(rh), ValueType::Number(lh)) => Ok(ValueType::Number(rh - lh)),
            (ValueType::Text(rh), ValueType::Number(lh)) => {
                let number_string = i32::from_str(rh.as_str());

                if let Result::Ok(number) = number_string {
                    Ok(ValueType::Number(number - lh))
                } else {
                    Err(format!("Cannot subtract string: {} to number: {}.", rh, lh))
                }
            }
            (ValueType::Number(rh), ValueType::Text(lh)) => {
                let number_string = i32::from_str(lh.as_str());

                if let Result::Ok(number) = number_string {
                    Ok(ValueType::Number(number - rh))
                } else {
                    Err(format!("Cannot subtract string: {} to number: {}.", lh, rh))
                }
            }
            _ => Err(String::from("Can only subtract numbers.")),
        }
    }
}

impl Mul for ValueType {
    type Output = Result<ValueType, String>;

    fn mul(self, other: ValueType) -> Self::Output {
        match (self, other) {
            (ValueType::Number(rh), ValueType::Number(lh)) => Ok(ValueType::Number(rh * lh)),
            (ValueType::Text(rh), ValueType::Number(lh)) => {
                let number_string = i32::from_str(rh.as_str());

                if let Result::Ok(number) = number_string {
                    Ok(ValueType::Number(number * lh))
                } else {
                    Err(format!("Cannot multiply string: {} to number: {}.", rh, lh))
                }
            }
            (ValueType::Number(rh), ValueType::Text(lh)) => {
                let number_string = i32::from_str(lh.as_str());

                if let Result::Ok(number) = number_string {
                    Ok(ValueType::Number(number * rh))
                } else {
                    Err(format!("Cannot multiply string: {} to number: {}.", lh, rh))
                }
            }
            _ => Err(String::from("Can only multiply numbers.")),
        }
    }
}

impl Div for ValueType {
    type Output = Result<ValueType, String>;

    fn div(self, other: ValueType) -> Self::Output {
        match (self, other) {
            (ValueType::Number(rh), ValueType::Number(lh)) => Ok(ValueType::Number(rh / lh)),
            (ValueType::Text(rh), ValueType::Number(lh)) => {
                let number_string = i32::from_str(rh.as_str());

                if let Result::Ok(number) = number_string {
                    Ok(ValueType::Number(number / lh))
                } else {
                    Err(format!("Cannot divide string: {} to number: {}.", rh, lh))
                }
            }
            (ValueType::Number(rh), ValueType::Text(lh)) => {
                let number_string = i32::from_str(lh.as_str());

                if let Result::Ok(number) = number_string {
                    Ok(ValueType::Number(number / rh))
                } else {
                    Err(format!("Cannot divide string: {} to number: {}.", lh, rh))
                }
            }
            _ => Err(String::from("Can only divide numbers.")),
        }
    }
}

impl Not for ValueType {
    type Output = Result<ValueType, String>;

    fn not(self) -> Self::Output {
        match self {
            ValueType::Bool(ref boolean) => Ok(ValueType::Bool(!boolean)),
            _ => Err(String::from("Cannot apply NOT to non-boolean type.")),
        }
    }
}

impl Neg for ValueType {
    type Output = Result<ValueType, String>;

    fn neg(self) -> Self::Output {
        match self {
            ValueType::Number(ref number) => Ok(ValueType::Number(-*number)),
            _ => Err(String::from("Cannot apply NEG to non-boolean type.")),
        }
    }
}

impl PartialEq for ValueType {
    fn eq(&self, other: &ValueType) -> bool {
        match (self, other) {
            (&ValueType::Text(ref rh), &ValueType::Text(ref lh)) => rh == lh,
            (&ValueType::Number(rh), &ValueType::Number(lh)) => rh == lh,
            (&ValueType::Bool(rh), &ValueType::Bool(lh)) => rh == lh,

            (&ValueType::Number(rh), &ValueType::Text(ref lh)) => {
                let number_string = i32::from_str(lh.as_str()).unwrap();
                rh == number_string
            }

            (&ValueType::Text(ref rh), &ValueType::Number(lh)) => {
                let number_string = i32::from_str(rh.as_str()).unwrap();
                lh == number_string
            }
            _ => false,
        }
    }

    fn ne(&self, other: &ValueType) -> bool {
        match (self, other) {
            (&ValueType::Number(rh), &ValueType::Number(lh)) => rh != lh,
            (&ValueType::Text(ref rh), &ValueType::Text(ref lh)) => rh != lh,
            (&ValueType::Bool(rh), &ValueType::Bool(lh)) => rh != lh,

            (&ValueType::Number(rh), &ValueType::Text(ref lh)) => {
                let number_string = i32::from_str(lh.as_str()).unwrap();
                rh != number_string
            }

            (&ValueType::Text(ref rh), &ValueType::Number(lh)) => {
                let number_string = i32::from_str(rh.as_str()).unwrap();
                lh != number_string
            }
            _ => false,
        }
    }
}

impl PartialOrd for ValueType {
    fn partial_cmp(&self, other: &ValueType) -> Option<Ordering> {
        if self > other {
            Some(Ordering::Greater)
        } else if self < other {
            Some(Ordering::Less)
        } else if self == other {
            Some(Ordering::Equal)
        } else {
            None
        }
    }

    fn gt(&self, other: &ValueType) -> bool {
        match (self, other) {
            (&ValueType::Number(rh), &ValueType::Number(lh)) => rh > lh,
            (&ValueType::Number(rh), &ValueType::Text(ref lh)) => {
                let number_string = i32::from_str(lh.as_str()).unwrap();
                rh > number_string
            }

            (&ValueType::Text(ref rh), &ValueType::Number(lh)) => {
                let number_string = i32::from_str(rh.as_str()).unwrap();
                lh > number_string
            }
            _ => false,
        }
    }

    fn ge(&self, other: &ValueType) -> bool {
        match (self, other) {
            (&ValueType::Number(rh), &ValueType::Number(lh)) => rh >= lh,
            (&ValueType::Number(rh), &ValueType::Text(ref lh)) => {
                let number_string = i32::from_str(lh.as_str()).unwrap();
                rh >= number_string
            }

            (&ValueType::Text(ref rh), &ValueType::Number(lh)) => {
                let number_string = i32::from_str(rh.as_str()).unwrap();
                lh >= number_string
            }
            _ => false,
        }
    }

    fn lt(&self, other: &ValueType) -> bool {
        match (self, other) {
            (&ValueType::Number(rh), &ValueType::Number(lh)) => rh < lh,
            (&ValueType::Number(rh), &ValueType::Text(ref lh)) => {
                let number_string = i32::from_str(lh.as_str()).unwrap();
                rh < number_string
            }

            (&ValueType::Text(ref rh), &ValueType::Number(lh)) => {
                let number_string = i32::from_str(rh.as_str()).unwrap();
                lh < number_string
            }
            _ => false,
        }
    }

    fn le(&self, other: &ValueType) -> bool {
        match (self, other) {
            (&ValueType::Number(rh), &ValueType::Number(lh)) => rh <= lh,
            (&ValueType::Number(rh), &ValueType::Text(ref lh)) => {
                let number_string = i32::from_str(lh.as_str()).unwrap();
                rh <= number_string
            }

            (&ValueType::Text(ref rh), &ValueType::Number(lh)) => {
                let number_string = i32::from_str(rh.as_str()).unwrap();
                lh <= number_string
            }
            _ => false,
        }
    }
}
