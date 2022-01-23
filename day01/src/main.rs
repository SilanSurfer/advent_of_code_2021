use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Result<T> = std::result::Result<T, anyhow::Error>;

fn count_increase(data: impl IntoIterator<Item = u64>) -> u64 {
    data.into_iter()
        .tuple_windows()
        .filter(|(prev, next)| prev < next)
        .count() as _
}

fn count_increase_window(data: impl IntoIterator<Item = u64>) -> u64 {
    let trans_data: Vec<u64> = data
        .into_iter()
        .tuple_windows::<(_, _, _)>()
        .map(|(a, b, c)| a + b + c)
        .collect();
    count_increase(trans_data)
}

fn read_input(data: impl BufRead) -> Result<Vec<u64>> {
    data.lines()
        .map(|line| line?.parse().map_err(Into::into))
        .collect()
}

fn main() -> Result<()> {
    let file = File::open("day01/input.txt")?;
    let reader = BufReader::new(file);
    let data = read_input(reader)?;

    println!("First case: {}", count_increase(data.clone()));
    println!("Second case: {}", count_increase_window(data));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{count_increase, count_increase_window};

    #[test]
    fn first_example_case() {
        let data: Vec<u64> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(7, count_increase(data));
    }

    #[test]
    fn second_example_case() {
        let data: Vec<u64> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(5, count_increase_window(data));
    }
}
