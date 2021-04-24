use std::path::Path;
use std::ops::RangeInclusive;
use std::collections::{HashSet, HashMap};
use std::cmp::{min, max};
use AoC2020::utils::read_lines;
use std::hash::Hash;

use nom::{
  IResult,
  branch::alt,
  bytes::complete::take,
  character::complete::{char, one_of},
  combinator::map,
  multi::{fold_many1, many0},
  number::complete::double,
  sequence::tuple,
};
use std::fmt;
use std::fmt::Formatter;
use nom::error::ErrorKind;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Operator {
  Minus,
  Plus,
  Divide,
  Multiply,
  OpenParenthesis,
  CloseParenthesis,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
  Number(f64),
  Operator(Operator),
}

// #[derive(Debug, Clone, PartialEq)]
// pub enum ParseUserError {
//   InvalidOperator { operator: char },
// }

fn parse_operator(s: &str) -> IResult<&str, Operator> {
  let (s, c) = take(1 as usize)(s)?;
  assert_eq!(c.len(), 1);
  Ok((
    s,
    match c.chars().next().unwrap() {
      '+' => Ok(Operator::Plus),
      '-' => Ok(Operator::Minus),
      '*' => Ok(Operator::Multiply),
      '/' => Ok(Operator::Divide),
      _ => Err(nom::Err::Error(nom::error::Error::new(s, ErrorKind::Alpha)))
    }?,
  ))
}

fn parse_number(s: &str) -> IResult<&str, f64> {
  double(s)
}

fn skip_whitespace(s: &str) -> IResult<&str, ()> {
  Ok((many0(one_of(" \t\x0c\n"))(s)?.0, ()))
}

pub fn parse(s: &str) -> Vec<Token> {
  let (_, result) = fold_many1(
    map(
      tuple((
        skip_whitespace,
        alt((
          map(parse_operator, Token::Operator),
          map(parse_number, Token::Number),
          map(char('('), |_| Token::Operator(Operator::OpenParenthesis)),
          map(char(')'), |_| Token::Operator(Operator::CloseParenthesis)),
        )),
      )),
      |((), token)| token,
    ),
    Vec::new(),
    |mut acc, token| {
      acc.push(token);
      acc
    },
  )(s).unwrap_or_default();
  result
}

// Implementation of the shunting yard algorithm that executes operators as they're popped off the operator stack
fn execute_calculation(tokens: &Vec<Token>, precedence: &HashMap<Operator, u8>) -> f64
{
  let mut operators: Vec<(Operator, u8)> = vec![];

  let mut stack: Vec<f64> = vec![];
  let execute_operator = |op, stack: &mut Vec<f64>| {
    let num1 = stack.pop().unwrap();
    let num2 = stack.pop().unwrap();
    stack.push(match op {
      Operator::Minus => num1 - num2,
      Operator::Plus => num1 + num2,
      Operator::Multiply => num1 * num2,
      Operator::Divide => num1 / num2,
      _ => panic!["Unsupported operator"]
    })
  };

  for token in tokens {
    match token {
      Token::Number(num) => stack.push(*num),
      Token::Operator(Operator::OpenParenthesis) => operators.push((Operator::OpenParenthesis, 10)),
      Token::Operator(Operator::CloseParenthesis) => {
        while operators.last().unwrap().0 != Operator::OpenParenthesis {
          execute_operator(operators.pop().unwrap().0, &mut stack);
        }
        operators.pop();
      }
      Token::Operator(op) => {
        let p = *precedence.get(op).unwrap();
        while !operators.is_empty() && operators.last().unwrap().1 >= p && operators.last().unwrap().0 != Operator::OpenParenthesis {
          execute_operator(operators.pop().unwrap().0, &mut stack);
        }
        operators.push((*op, p));
      }
    }
  }

  for (op, _) in operators.into_iter().rev() {
    execute_operator(op, &mut stack);
  }

  assert_eq!(stack.len(), 1);

  stack.pop().unwrap()
}

fn problem1(path: &Path) -> usize
{
  let precedence = [(Operator::Plus, 1u8), (Operator::Minus, 1u8), (Operator::Multiply, 1u8), (Operator::Divide, 1u8), (Operator::OpenParenthesis, 10u8), (Operator::CloseParenthesis, 10u8)].iter().cloned().collect();
  read_lines(path)
    .unwrap()
    .map(|l| execute_calculation(&parse(&l.unwrap()), &precedence))
    .fold(0.0, |l, r| l + r)
    as usize
}

fn problem2(path: &Path) -> usize
{
  let precedence = [(Operator::Plus, 2u8), (Operator::Minus, 2u8), (Operator::Multiply, 1u8), (Operator::Divide, 1u8), (Operator::OpenParenthesis, 10u8), (Operator::CloseParenthesis, 10u8)].iter().cloned().collect();
  read_lines(path)
    .unwrap()
    .map(|l| execute_calculation(&parse(&l.unwrap()), &precedence))
    .fold(0.0, |l, r| l + r)
    as usize
}

fn main() {
  let path = Path::new(r"data/18-1.txt");
  println!("Result of problem 1: {}", problem1(path));
  println!("Result of problem 2: {}", problem2(path));
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn test_parse() {
    assert_eq!(parse("1 + 2 * 3 + (4/5)"),
               vec![Token::Number(1.0), Token::Operator(Operator::Plus), Token::Number(2.0), Token::Operator(Operator::Multiply),
                    Token::Number(3.0), Token::Operator(Operator::Plus), Token::Operator(Operator::OpenParenthesis), Token::Number(4.0),
                    Token::Operator(Operator::Divide), Token::Number(5.0), Token::Operator(Operator::CloseParenthesis)]
    )
  }

  #[test]
  fn test_execute_calculation()
  {
    let precedence = [(Operator::Plus, 1u8), (Operator::Minus, 1u8), (Operator::Multiply, 1u8), (Operator::Divide, 1u8), (Operator::OpenParenthesis, 10u8), (Operator::CloseParenthesis, 10u8)].iter().cloned().collect();
    // 1 + 2 * 3 + ((4 * 5)+1) == 30
    let result = execute_calculation(
      &vec![Token::Number(1.0), Token::Operator(Operator::Plus), Token::Number(2.0), Token::Operator(Operator::Multiply),
            Token::Number(3.0), Token::Operator(Operator::Plus), Token::Operator(Operator::OpenParenthesis),
            Token::Operator(Operator::OpenParenthesis), Token::Number(4.0), Token::Operator(Operator::Multiply),
            Token::Number(5.0), Token::Operator(Operator::CloseParenthesis), Token::Operator(Operator::Plus),
            Token::Number(1.0), Token::Operator(Operator::CloseParenthesis)],
      &precedence,
    );
    assert_eq!(result, 30.0);
  }

  #[test]
  fn test_execute_calculation2()
  {
    // 2 * 3 + (4 * 5) == 46
    let precedence = [(Operator::Plus, 2u8), (Operator::Minus, 2u8), (Operator::Multiply, 1u8), (Operator::Divide, 1u8), (Operator::OpenParenthesis, 10u8), (Operator::CloseParenthesis, 10u8)].iter().cloned().collect();
    let result = execute_calculation(
      &vec![Token::Number(2.0), Token::Operator(Operator::Multiply), Token::Number(3.0), Token::Operator(Operator::Plus),
            Token::Operator(Operator::OpenParenthesis), Token::Number(4.0), Token::Operator(Operator::Multiply),
            Token::Number(5.0), Token::Operator(Operator::CloseParenthesis)],
      &precedence,
    );
    assert_eq!(result, 46.0);
  }
}