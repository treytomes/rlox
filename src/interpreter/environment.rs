use std::collections::HashMap;

use crate::debug::{FileLocation, HasFileLocation};

use super::{Object, RuntimeError};

pub struct Environment {
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    fn assert_not_defined(
        &self,
        loc: &dyn HasFileLocation,
        name: &str,
    ) -> Result<(), RuntimeError> {
        if self.is_defined(name) {
            return Err(RuntimeError::new(
                format!("variable {} already defined", name).as_str(),
                loc.get_line(),
                loc.get_column(),
            ));
        }
        Ok(())
    }

    fn assert_defined(&self, loc: &dyn HasFileLocation, name: &str) -> Result<(), RuntimeError> {
        if !self.is_defined(name) {
            return Err(RuntimeError::new(
                format!("variable {} not defined", name).as_str(),
                loc.get_line(),
                loc.get_column(),
            ));
        }
        Ok(())
    }

    pub fn define(
        &mut self,
        loc: &dyn HasFileLocation,
        name: &str,
        value: Object,
    ) -> Result<Object, RuntimeError> {
        self.assert_not_defined(loc, name)?;
        self.values.insert(name.to_string(), value);
        self.get(loc, name)
    }

    pub fn get(&self, loc: &dyn HasFileLocation, name: &str) -> Result<Object, RuntimeError> {
        self.assert_defined(loc, name)?;
        Ok(self.values.get(name).unwrap().clone())
    }

    pub fn assign(
        &mut self,
        loc: &dyn HasFileLocation,
        name: &str,
        value: Object,
    ) -> Result<Object, RuntimeError> {
        self.assert_defined(loc, name)?;
        self.values.insert(name.to_string(), value);
        self.get(loc, name)
    }

    pub fn delete(
        &mut self,
        loc: &dyn HasFileLocation,
        name: &str,
    ) -> Result<Object, RuntimeError> {
        self.assert_defined(loc, name)?;
        self.values.remove(name);
        self.get(loc, name)
    }

    pub fn is_defined(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }
}
