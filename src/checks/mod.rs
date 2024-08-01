use std::sync::LazyLock;
use crate::Result;

mod email;

pub type CheckFn = fn(&str) -> Result<()>;

pub struct Check {
    pub key: &'static str,
    pub columns: &'static [&'static str],
    pub check: CheckFn,
}

impl Check {
    pub fn list_keys() -> Vec<&'static str> {
        CHECKS.iter().map(|check| check.key).collect()
    }

    pub fn get(key: &str) -> Option<&Check> {
        CHECKS.iter().find(|check| check.key == key)
    }

    pub fn get_key(key: &str) -> Option<&'static str> {
        CHECKS.iter().find(|check| check.key == key).map(|check| check.key)
    }

    pub fn get_for_column(header: &str) -> Vec<&Check> {
        CHECKS.iter().filter(|check| check.columns.contains(&header)).collect()
    }

    pub fn run(&self, data: &str) -> Result<()> {
        (self.check)(data)
    }
}

pub static CHECKS: LazyLock<Vec<Check>> = LazyLock::new(|| {
    vec![
        email::init(),
    ]
});
