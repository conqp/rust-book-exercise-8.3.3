use std::collections::hash_map::Entry;

#[derive(Debug)]
pub struct DepartmentMut<'entry> {
    entry: Entry<'entry, String, Vec<String>>,
}

impl<'entry> DepartmentMut<'entry> {
    pub fn add(self, employee: &str) {
        self.entry.or_default().push(employee.into())
    }
}

impl<'entry> From<Entry<'entry, String, Vec<String>>> for DepartmentMut<'entry> {
    fn from(inner: Entry<'entry, String, Vec<String>>) -> Self {
        Self { entry: inner }
    }
}
