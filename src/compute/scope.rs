//! Mapping from identifiers to functions.

use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};

use super::value::FuncValue;

/// A map from identifiers to functions.
pub struct Scope {
    functions: HashMap<String, FuncValue>,
    fallback: FuncValue,
}

impl Scope {
    // Create a new empty scope with a fallback function that is invoked when no
    // match is found.
    pub fn new(fallback: FuncValue) -> Self {
        Self {
            functions: HashMap::new(),
            fallback,
        }
    }

    /// Associate the given name with the function.
    pub fn insert(&mut self, name: impl Into<String>, function: FuncValue) {
        self.functions.insert(name.into(), function);
    }

    /// Return the function with the given name if there is one.
    pub fn func(&self, name: &str) -> Option<&FuncValue> {
        self.functions.get(name)
    }

    /// Return the function with the given name or the fallback if there is no
    /// such function.
    pub fn func_or_fallback(&self, name: &str) -> &FuncValue {
        self.func(name).unwrap_or_else(|| self.fallback())
    }

    /// Return the fallback function.
    pub fn fallback(&self) -> &FuncValue {
        &self.fallback
    }
}

impl Debug for Scope {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_set().entries(self.functions.keys()).finish()
    }
}