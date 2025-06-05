use std::str::FromStr;
use std::sync::LazyLock;

use regex::Regex;

const LIST: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"list(?:\s+(\w+))?").expect("Invalid regex. This is a bug."));
const ADD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"add\s+(\w+)\s+to\s+(\w+)").expect("Invalid regex. This is a bug.")
});

#[derive(Debug)]
pub enum Command {
    List(Listing),
    Add {
        employee: String,
        department: String,
    },
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(captures) = LIST.captures(s) {
            return if let Some(department) = captures.iter().flatten().nth(1) {
                Ok(Command::List(Listing::Department(
                    department.as_str().into(),
                )))
            } else {
                Ok(Command::List(Listing::Company))
            };
        }

        if let Some(captures) = ADD.captures(s) {
            let (_, [employee, department]) = captures.extract();
            return Ok(Command::Add {
                employee: employee.into(),
                department: department.into(),
            });
        }

        Err(())
    }
}

#[derive(Debug)]
pub enum Listing {
    Company,
    Department(String),
}
