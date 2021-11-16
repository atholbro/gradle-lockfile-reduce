use std::fmt;

#[derive(PartialEq, PartialOrd, Ord, Eq)]
pub struct GradleDependency {
    pub group: String,
    pub name: String,
    pub version: String,
}

impl fmt::Display for GradleDependency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.group, self.name, self.version)
    }
}
