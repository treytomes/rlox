use std::collections::HashMap;

use crate::debug::HasFileLocation;

use super::{Object, RuntimeError};

pub struct Environment {
    enclosing: Option<Box<Environment>>,
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn root() -> Self {
        Self {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn child(enclosing: Environment) -> Self {
        Self {
            enclosing: Some(Box::new(enclosing)),
            values: HashMap::new(),
        }
    }

    fn err_already_defined(&self, loc: &dyn HasFileLocation, name: &str) -> RuntimeError {
        RuntimeError::new(
            format!("variable {} already defined", name).as_str(),
            loc.get_line(),
            loc.get_column(),
        )
    }

    fn err_not_defined(&self, loc: &dyn HasFileLocation, name: &str) -> RuntimeError {
        RuntimeError::new(
            format!("variable {} not defined", name).as_str(),
            loc.get_line(),
            loc.get_column(),
        )
    }

    fn assert_not_defined(
        &self,
        loc: &dyn HasFileLocation,
        name: &str,
    ) -> Result<(), RuntimeError> {
        if self.is_defined(name) {
            return Err(self.err_already_defined(loc, name));
        }
        Ok(())
    }

    fn assert_not_locally_defined(
        &self,
        loc: &dyn HasFileLocation,
        name: &str,
    ) -> Result<(), RuntimeError> {
        if self.is_locally_defined(name) {
            return Err(self.err_already_defined(loc, name));
        }
        Ok(())
    }

    fn assert_locally_defined(
        &self,
        loc: &dyn HasFileLocation,
        name: &str,
    ) -> Result<(), RuntimeError> {
        if !self.is_defined(name) {
            return Err(self.err_not_defined(loc, name));
        }
        Ok(())
    }

    fn assert_defined(&self, loc: &dyn HasFileLocation, name: &str) -> Result<(), RuntimeError> {
        if !self.is_defined(name) {
            return Err(self.err_not_defined(loc, name));
        }
        Ok(())
    }

    pub fn define(
        &mut self,
        loc: &dyn HasFileLocation,
        name: &str,
        value: Object,
    ) -> Result<Object, RuntimeError> {
        self.assert_not_locally_defined(loc, name)?;
        self.values.insert(name.to_string(), value);
        self.get(loc, name)
    }

    pub fn get(&self, loc: &dyn HasFileLocation, name: &str) -> Result<Object, RuntimeError> {
        if self.is_locally_defined(name) {
            return Ok(self.values.get(name).unwrap().clone());
        } else if self.enclosing.is_some() {
            return self.enclosing.as_ref().unwrap().get(loc, name);
        }
        Err(self.err_not_defined(loc, name))
    }

    pub fn assign(
        &mut self,
        loc: &dyn HasFileLocation,
        name: &str,
        value: Object,
    ) -> Result<Object, RuntimeError> {
        if self.is_locally_defined(name) {
            self.values.insert(name.to_string(), value);
            return self.get(loc, name);
        } else if self.enclosing.is_some() {
            return self.enclosing.as_mut().unwrap().assign(loc, name, value);
        } else {
            return Err(self.err_not_defined(loc, name));
        }
    }

    pub fn delete(
        &mut self,
        loc: &dyn HasFileLocation,
        name: &str,
    ) -> Result<Object, RuntimeError> {
        self.assert_locally_defined(loc, name)?;
        self.values.remove(name);
        self.get(loc, name)
    }

    pub fn is_locally_defined(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }

    pub fn is_defined(&self, name: &str) -> bool {
        if self.is_locally_defined(name) {
            return true;
        } else if self.enclosing.is_some() {
            return self.enclosing.as_ref().unwrap().is_defined(name);
        }
        false
    }
}
