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

struct Submarine {
    moves: Vec<Command>,
    position: (i64, i64),
}

impl Submarine {
    pub fn new(moves: Vec<Command>) -> Self {
        Submarine {
            moves,
            position: (0, 0),
        }
    }

    pub fn calculate_final_position(&mut self) -> Result<(i64, i64)> {
        // TODO: Change that, it's shitty solution
        let moves = self.moves.clone();
        moves.iter().for_each(|command| self.apply_command(command));
        Ok(self.position)
    }

    fn apply_command(&mut self, command: &Command) {
        match command {
            Command::Forward(x) => {
                self.position.0 = self.position.0 + *x as i64;
            }
            Command::Up(x) => {
                self.position.1 = self.position.1 - *x as i64;
            }
            Command::Down(x) => {
                self.position.1 = self.position.1 + *x as i64;
            }
        };
    }
}

fn read_input(reader: impl BufRead) -> Result<Vec<Command>> {
    let mut data: Vec<Command> = vec![];
    for line in reader.lines() {
        data.push(line?.parse::<Command>()?);
    }
    Ok(data)
}

fn main() -> Result<()> {
    let file = File::open("day02/input.txt")?;
    let reader = BufReader::new(file);
    let data = read_input(reader)?;

    let result_1 = Submarine::new(data).calculate_final_position()?;
    println!("Part 1 solution: {}", result_1.0 * result_1.1);
    Ok(())
}
