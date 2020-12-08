use std::path::Path;
use std::collections::HashSet;
use AoC2020::utils::read_lines;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Instruction
{
  ACC(i32),
  JMP(i32),
  NOP(i32),
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct ExecutionState {
  last_pc: i32,
  pc: i32,
  acc: i32,
}

impl ExecutionState {
  fn new() -> Self {
    ExecutionState { last_pc: -1, pc: 0, acc: 0 }
  }

  fn execute_instruction(&self, instruction: Instruction) -> ExecutionState
  {
    match instruction {
      Instruction::ACC(num) => ExecutionState { last_pc: self.pc, pc: self.pc + 1, acc: self.acc + num },
      Instruction::JMP(num) => ExecutionState { last_pc: self.pc, pc: self.pc + num, acc: self.acc },
      Instruction::NOP(_) => ExecutionState { last_pc: self.pc, pc: self.pc + 1, acc: self.acc }
    }
  }
}

fn parse_line(line: &str) -> Instruction
{
  let num = if &line[4..5] == "-" { -1 } else { 1 } * line[5..].parse::<i32>().unwrap();
  match &line[0..3] {
    "jmp" => Instruction::JMP(num),
    "acc" => Instruction::ACC(num),
    "nop" => Instruction::NOP(num),
    _ => panic!("invalid operation")
  }
}

#[derive(Debug, Eq, PartialEq)]
enum Error
{
  LOOP(ExecutionState),
  OVERSHOT(ExecutionState),
}

fn run_instructions(instructions: &[Instruction]) -> Result<ExecutionState, Error>
{
  let mut state = ExecutionState::new();
  let mut visited = HashSet::<i32>::new();

  while (state.pc as usize) < instructions.len() {
    visited.insert(state.pc);
    let instruction = instructions[state.pc as usize];
    let new_state = state.execute_instruction(instruction);
    if visited.contains(&new_state.pc)
    {
      return Err(Error::LOOP(state));
    }
    state = new_state
  }

  if state.pc == instructions.len() as i32 {
    Ok(state)
  } else {
    Err(Error::OVERSHOT(state))
  }
}

fn problem2(path: &Path) -> i32
{
  let mut instructions = read_lines(path).unwrap()
    .map(|l| parse_line(&l.unwrap()))
    .collect::<Vec<_>>();

  for i in 0..instructions.len() {
    let instruction = instructions[i];

    instructions[i] = match instruction {
      Instruction::JMP(num) => Instruction::NOP(num),
      Instruction::NOP(num) => Instruction::JMP(num),
      _ => continue
    };

    if let Ok(state) = run_instructions(&instructions) {
      return state.acc;
    }

    instructions[i] = instruction
  }

  panic!("Did not find a solution")
}

fn problem1(path: &Path) -> i32
{
  let result = run_instructions(&read_lines(path).unwrap()
    .map(|l| parse_line(&l.unwrap()))
    .collect::<Vec<_>>());
  if let Err(Error::LOOP(state)) = result {
    state.acc
  } else {
    0
  }
}

fn main() {
  let path = Path::new(r"data/8-1.txt");
  println!("Result of problem 1: {}", problem1(path));
  println!("Result of problem 2: {}", problem2(path));
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn test_parse_line()
  {
    assert_eq!(parse_line("jmp +2"), Instruction::JMP(2));
    assert_eq!(parse_line("jmp -30"), Instruction::JMP(-30));

    assert_eq!(parse_line("acc +5"), Instruction::ACC(5));
    assert_eq!(parse_line("acc -3"), Instruction::ACC(-3));

    assert_eq!(parse_line("nop +0"), Instruction::NOP(0));
    assert_eq!(parse_line("nop -50"), Instruction::NOP(-50));
  }

  #[test]
  fn test_execute_instruction()
  {
    let mut state = ExecutionState::new();
    let instructions = vec![
      Instruction::ACC(2),
      Instruction::JMP(20),
      Instruction::ACC(-30),
      Instruction::NOP(0),
      Instruction::JMP(-4),
    ];
    let mut states: Vec<ExecutionState> = vec![];

    for i in instructions {
      state = state.execute_instruction(i);
      states.push(state)
    }

    assert_eq!(states[0], ExecutionState { last_pc: 0, pc: 1, acc: 2 });
    assert_eq!(states[1], ExecutionState { last_pc: 1, pc: 21, acc: 2 });
    assert_eq!(states[2], ExecutionState { last_pc: 21, pc: 22, acc: -28 });
    assert_eq!(states[3], ExecutionState { last_pc: 22, pc: 23, acc: -28 });
    assert_eq!(states[4], ExecutionState { last_pc: 23, pc: 19, acc: -28 });
  }

  #[test]
  fn test_run_instructions()
  {
    let instructions = vec![
      Instruction::ACC(2),
      Instruction::JMP(2),
      Instruction::ACC(-30),
      Instruction::NOP(0),
    ];
    assert_eq!(run_instructions(&instructions),
               Ok(ExecutionState { last_pc: 3, pc: 4, acc: 2 }));
    let instructions = vec![
      Instruction::ACC(2),
      Instruction::JMP(30),
      Instruction::ACC(-30),
      Instruction::NOP(0),
    ];
    assert_eq!(run_instructions(&instructions),
               Err(Error::OVERSHOT(ExecutionState { last_pc: 1, pc: 31, acc: 2 })));

    let instructions = vec![
      Instruction::NOP(0),
      Instruction::ACC(1),
      Instruction::JMP(4),
      Instruction::ACC(3),
      Instruction::JMP(-3),
      Instruction::ACC(-99),
      Instruction::ACC(1),
      Instruction::JMP(-4),
      Instruction::ACC(6)
    ];

    assert_eq!(run_instructions(&instructions),
               Err(Error::LOOP(ExecutionState { last_pc: 3, pc: 4, acc: 5 })));
  }
}
