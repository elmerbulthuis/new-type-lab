use std::ops::Deref;

pub struct PersonName<'l>(&'l str);

impl<'l> PersonName<'l> {
    pub fn new(value: &'l str) -> Self {
        Self(value)
    }
}

impl<'l> From<&'l str> for PersonName<'l> {
    fn from(value: &'l str) -> Self {
        PersonName::new(value)
    }
}

impl<'l> From<PersonName<'l>> for &'l str {
    fn from(value: PersonName<'l>) -> Self {
        value.0
    }
}

impl<'l> Deref for PersonName<'l> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}
