#[macro_use]
extern crate clap;

use clap::Arg;

enum Position {
    Left(usize),
    Zero,
    Right(usize),
}

struct Tape {
    left: Vec<u8>,
    origin: u8,
    right: Vec<u8>,
    position: Position,
}

impl Tape {
    fn new() -> Self {
        Self {
            left: vec![],
            origin: 0,
            right: vec![],
            position: Position::Zero,
        }
    }
    fn step_right(&mut self) {
        self.position = match self.position {
            Position::Left(0) => Position::Zero,
            Position::Zero => {
                if self.right.len() == 0 {
                    self.right.push(0);
                }

                Position::Right(0)
            }
            Position::Right(n) => {
                if n + 1 == self.right.len() {
                    self.right.push(0);
                }

                Position::Right(n + 1)
            }
            Position::Left(n) => Position::Left(n - 1),
        }
    }
    fn step_left(&mut self) {
        self.position = match self.position {
            Position::Right(0) => Position::Zero,
            Position::Zero => {
                if self.left.len() == 0 {
                    self.left.push(0);
                }

                Position::Left(0)
            }
            Position::Right(n) => Position::Right(n - 1),
            Position::Left(n) => {
                if n + 1 == self.left.len() {
                    self.left.push(0);
                }

                Position::Left(n + 1)
            }
        }
    }
    fn increment(&mut self) {
        match self.position {
            Position::Left(n) => self.left[n] += 1,
            Position::Zero => self.origin += 1,
            Position::Right(n) => self.right[n] += 1,
        }
    }
    fn decrement(&mut self) {
        match self.position {
            Position::Left(n) => self.left[n] -= 1,
            Position::Zero => self.origin -= 1,
            Position::Right(n) => self.right[n] -= 1,
        }
    }
    fn get(&self) -> u8 {
        match self.position {
            Position::Left(n) => self.left[n],
            Position::Zero => self.origin,
            Position::Right(n) => self.right[n],
        }
    }
    fn set(&mut self, byte: u8) {
        match self.position {
            Position::Left(n) => self.left[n] = byte,
            Position::Zero => self.origin = byte,
            Position::Right(n) => self.right[n] = byte,
        }
    }
}

fn main() {
    let matches = app_from_crate!()
        .arg(
            Arg::with_name("code")
                .help("This is your brainfuck code")
                .value_name("CODE")
                .required(true)
                .index(1),
        )
        .get_matches();

    let mut tape = Tape::new();
    interpret(&mut tape, matches.value_of("code").unwrap());
}

fn interpret(tape: &mut Tape, code: &str) {
    use std::io::Read;

    let mut code: Vec<_> = code.chars().rev().enumerate().collect();

    while let Some((i, c)) = code.pop() {
        match c {
            '>' => tape.step_right(),
            '<' => tape.step_left(),
            '+' => tape.increment(),
            '-' => tape.decrement(),
            '.' => print!("{}", tape.get() as char),
            ',' => tape.set(
                std::io::stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok())
                    .expect("failed to read byte"),
            ),
            '[' => {
                let mut buf = String::new();
                let mut bracket_count = 1;

                while let Some((_, next_char)) = code.pop() {
                    if next_char == '[' {
                        bracket_count += 1;
                    } else if next_char == ']' {
                        bracket_count -= 1;
                    }

                    if next_char == ']' && bracket_count == 0 {
                        break;
                    }

                    buf.push(next_char);
                }

                while tape.get() != 0 {
                    interpret(tape, buf.as_str());
                }
            }
            '\n' | '\t' | '\r' | ' ' => {}
            _ => panic!("unexpected character `{}` at index {}", c, code.len() - i),
        }
    }
}
