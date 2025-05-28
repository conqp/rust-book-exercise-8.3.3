use std::collections::hash_map::Entry;

#[derive(Debug)]
pub struct DepartmentMut<'entry> {
    entry: Entry<'entry, String, Vec<String>>,
}

impl<'entry> DepartmentMut<'entry> {
    pub fn add(self, employee: impl ToString) {
        self.entry.or_default().push(employee.to_string());
    }
}

impl<'entry> From<Entry<'entry, String, Vec<String>>> for DepartmentMut<'entry> {
    fn from(entry: Entry<'entry, String, Vec<String>>) -> Self {
        Self { entry }
    }
}
