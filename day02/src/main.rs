use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

type Result<T> = std::result::Result<T, anyhow::Error>;

#[derive(Debug, Clone)]
enum Command {
    Forward(u64),
    Up(u64),
    Down(u64),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        if parts.len() != 2 {
            return Err(anyhow::anyhow!("{} should have only 2 parts!", s));
        }
        let command = match parts[0] {
            "forward" => Command::Forward(parts[1].parse()?),
            "up" => Command::Up(parts[1].parse()?),
            "down" => Command::Down(parts[1].parse()?),
            _ => Err(anyhow::anyhow!("Unknown command: {}", parts[0]))?,
        };
        Ok(command)
    }
}

impl Command {
    pub fn apply(&self, mut position: (i64, i64)) -> (i64, i64) {
        match self {
            Command::Forward(x) => {
                position.0 = position.0 + *x as i64;
            }
            Command::Up(x) => {
                position.1 = position.1 - *x as i64;
            }
            Command::Down(x) => {
                position.1 = position.1 + *x as i64;
            }
        };
        position
    }
}

fn read_input(reader: impl BufRead) -> Result<Vec<Command>> {
    let mut data: Vec<Command> = vec![];
    for line in reader.lines() {
        data.push(line?.parse::<Command>()?);
    }
    Ok(data)
}

fn solution_1(input: Vec<Command>) -> i64 {
    let mut position: (i64, i64) = (0, 0);
    input.iter().for_each(|command| position = command.apply(position));

    position.0 * position.1

}

fn main() -> Result<()> {
    let file = File::open("day02/input.txt")?;
    let reader = BufReader::new(file);
    let data = read_input(reader)?;

    let result_1 = solution_1(data);
    println!("Solution for part 1: {}", result_1);
    assert_eq!(result_1, 2036120);
    Ok(())
}
