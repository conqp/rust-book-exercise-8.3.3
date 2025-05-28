use std::collections::HashMap;
use std::fmt::Display;
use std::vec::IntoIter;

use department::Department;
use department_mut::DepartmentMut;

mod department;
mod department_mut;

#[derive(Clone, Debug, Default)]
pub struct Company {
    departments: HashMap<String, Vec<String>>,
}

impl Company {
    #[must_use]
    pub fn department(&self, department: impl AsRef<str>) -> Option<Department> {
        self.departments
            .get(department.as_ref())
            .map(AsRef::as_ref)
            .map(Into::into)
    }

    #[must_use]
    pub fn department_mut(&mut self, department: impl ToString) -> DepartmentMut<'_> {
        self.departments.entry(department.to_string()).into()
    }

    fn iter(&self) -> IntoIter<(&String, &Vec<String>)> {
        let mut departments: Vec<_> = self.departments.iter().collect();
        departments.sort_by(|(department_name_a, _), (department_name_b, _)| {
            department_name_a.cmp(department_name_b)
        });
        departments.into_iter()
    }
}

impl Display for Company {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for (index, (department, employees)) in self.iter().enumerate() {
            write!(
                fmt,
                "Department {department}:\n{}",
                Department::from(employees.as_slice()),
            )?;

            if index + 1 < self.departments.len() {
                writeln!(fmt)?;
            }
        }

        Ok(())
    }
}
