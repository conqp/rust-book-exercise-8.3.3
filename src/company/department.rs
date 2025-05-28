use std::fmt::Display;

#[derive(Debug)]
pub struct Department<'a> {
    employees: &'a [String],
}

impl<'a> Display for Department<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut employees: Vec<_> = self.employees.to_vec();
        employees.sort();
        write!(f, "{}", employees.join(", "))
    }
}

impl<'a> From<&'a [String]> for Department<'a> {
    fn from(employees: &'a [String]) -> Self {
        Self { employees }
    }
}
