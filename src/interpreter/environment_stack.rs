use crate::debug::HasFileLocation;

use super::{Environment, Object, RuntimeError};

pub struct EnvironmentStack {
    stack: Vec<Environment>,
}

impl EnvironmentStack {
    pub fn new() -> Self {
        Self {
            stack: vec![Environment::new()],
        }
    }

    pub fn enter_scope(&mut self) {
        self.stack.push(Environment::new());
    }

    pub fn leave_scope(&mut self, loc: &dyn HasFileLocation) -> Result<(), RuntimeError> {
        if self.stack.len() == 1 {
            return Err(RuntimeError::new(
                "cannot leave global scope",
                loc.get_line(),
                loc.get_column(),
            ));
        }
        self.stack.pop();
        Ok(())
    }

    pub fn define_global(
        &mut self,
        loc: &dyn HasFileLocation,
        name: &str,
        value: Object,
    ) -> Result<Object, RuntimeError> {
        if let Some(env) = self.stack.first_mut() {
            return env.define(loc, name, value);
        }
        Err(RuntimeError::new(
            format!("cannot retrieve global environment for variable: {}", name).as_str(),
            0,
            0,
        ))
    }

    pub fn define(
        &mut self,
        loc: &dyn HasFileLocation,
        name: &str,
        value: Object,
    ) -> Result<Object, RuntimeError> {
        // Only define a variable in the top environment.
        if let Some(env) = self.stack.last_mut() {
            return env.define(loc, name, value);
        }
        Err(RuntimeError::new(
            format!("cannot retrieve environment for variable: {}", name).as_str(),
            loc.get_line(),
            loc.get_column(),
        ))
    }

    pub fn get(&self, loc: &dyn HasFileLocation, name: &str) -> Result<Object, RuntimeError> {
        // Starting from the last item in `stack`, work backwards looking for a definition of `name`
        for env in self.stack.iter().rev() {
            match env.get(loc, name) {
                Ok(value) => return Ok(value),
                Err(_) => continue,
            }
        }
        Err(RuntimeError::new(
            format!("undefined variable: {}", name).as_str(),
            loc.get_line(),
            loc.get_column(),
        ))
    }

    pub fn assign(
        &mut self,
        loc: &dyn HasFileLocation,
        name: &str,
        value: Object,
    ) -> Result<Object, RuntimeError> {
        for env in self.stack.iter_mut().rev() {
            match env.assign(loc, name, value.clone()) {
                Ok(_) => return Ok(value),
                Err(_) => continue,
            }
        }
        Err(RuntimeError::new(
            format!("undefined variable: {}", name).as_str(),
            loc.get_line(),
            loc.get_column(),
        ))
    }

    pub fn delete(
        &mut self,
        loc: &dyn HasFileLocation,
        name: &str,
    ) -> Result<Object, RuntimeError> {
        // Only delete the variable if it is defined in the top environment.
        if let Some(env) = self.stack.last_mut() {
            if env.is_defined(name) {
                return env.delete(loc, name);
            }

            return Err(RuntimeError::new(
                format!("undefined variable: {}", name).as_str(),
                loc.get_line(),
                loc.get_column(),
            ));
        }

        Err(RuntimeError::new(
            format!("cannot retrieve environment for variable: {}", name).as_str(),
            loc.get_line(),
            loc.get_column(),
        ))
    }

    // pub fn is_locally_defined(&self, name: &str) -> bool {
    //     self.stack.last().unwrap().is_defined(name)
    // }

    pub fn is_defined(&self, name: &str) -> bool {
        for env in self.stack.iter().rev() {
            if env.is_defined(name) {
                return true;
            }
        }
        false
    }
}
