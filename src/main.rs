extern crate core;

use std::io::stdin;

use log::{error, info};

use command::{Command, Listing};
use company::Company;

mod command;
mod company;

fn main() {
    env_logger::init();
    let mut company = Company::default();

    stdin().lines().map_while(Result::ok).for_each(|line| {
        if let Ok(command) = line.parse() {
            match command {
                Command::List(listing) => match listing {
                    Listing::Company => println!("{company}"),
                    Listing::Department(department) => {
                        if let Some(department) = company.department(&department) {
                            println!("{department}");
                        } else {
                            error!("No such department: {department}");
                        }
                    }
                },
                Command::Add {
                    employee,
                    department,
                } => {
                    company.department_mut(&department).add(&employee);
                    info!("Added {employee} to {department}");
                }
            }
        } else {
            error!("Invalid command!");
        }
    })
}
