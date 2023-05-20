use std::{
    fs::File,
    io::{self, Read},
    num,
    path::Path,
};

type FriendlyError = String;

#[derive(Debug)]
enum CliError {
    Io(io::Error, FriendlyError),
    ParseInt(num::ParseIntError, FriendlyError),
    ParseFloat(num::ParseFloatError, FriendlyError),
}

impl std::convert::From<num::ParseIntError> for CliError {
    fn from(err: num::ParseIntError) -> CliError {
        CliError::ParseInt(err, String::from("Invalid number"))
    }
}

impl From<num::ParseFloatError> for CliError {
    fn from(err: num::ParseFloatError) -> CliError {
        CliError::ParseFloat(err, String::from("Invalid number"))
    }
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err, String::from("Check file system"))
    }
}

fn file_double<P: AsRef<Path>>(file_path: P) -> Result<i32, CliError> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let n: i32 = contents.trim().parse()?;
    Ok(2 * n)
}

fn main() {
    let result = file_double("./a-file-that-doesn-not-exits.txt");

    match result {
        Ok(num) => println!("Number is {}", num),
        Err(CliError::Io(_, ferr)) => println!("{}", ferr),
        Err(CliError::ParseFloat(_, ferr)) => println!("{}", ferr),
        Err(CliError::ParseInt(_, ferr)) => println!("{}", ferr),
    };
}
