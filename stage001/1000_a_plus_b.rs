use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
enum InputError {
    ParseError(ParseIntError),
    InvalidInput,
}

impl std::fmt::Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseError(err) => write!(f, "ParseError({})", err),
            Self::InvalidInput => write!(f, "Invalid input: Please enter two numbers"),
        }
    }
}

impl std::error::Error for InputError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ParseError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<ParseIntError> for InputError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseError(value)
    }
}

impl From<std::io::Error> for InputError {
    fn from(_: std::io::Error) -> Self {
        Self::InvalidInput
    }
}

fn main() -> Result<(), InputError> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let nums: Vec<u8> = input.split_whitespace()
        .map(|s| s.parse::<u8>().map_err(InputError::from))
        .collect::<Result<Vec<_>, _>>()?;

    if nums.len() != 2 {
        return Err(InputError::InvalidInput);
    }

    println!("{}", nums[0] + nums[1]);
    Ok(())
}
