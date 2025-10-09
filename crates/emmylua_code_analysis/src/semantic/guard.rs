use std::{cell::RefCell, collections::HashSet, rc::Rc};

use crate::{InferFailReason, LuaTypeDeclId};

pub type InferGuardRef = Rc<InferGuard>;

/// Guard to prevent infinite recursion
/// Some type may reference itself, so we need to check if we have already inferred this type
///
/// This guard supports inheritance through Rc parent chain, allowing child guards to see
/// parent's visited types while maintaining their own independent tracking for branch protection.
#[derive(Debug, Clone)]
pub struct InferGuard {
    /// Current level's visited types
    current: RefCell<HashSet<LuaTypeDeclId>>,
    /// Parent guard (shared reference)
    parent: Option<Rc<InferGuard>>,
}

impl InferGuard {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            current: RefCell::new(HashSet::default()),
            parent: None,
        })
    }

    /// Create a child guard that inherits from parent
    /// This allows branching while preventing infinite recursion across the entire call stack
    pub fn fork(self: &Rc<Self>) -> Rc<Self> {
        Rc::new(Self {
            current: RefCell::new(HashSet::default()),
            parent: Some(Rc::clone(self)),
        })
    }

    /// Create a child guard from a non-Rc guard
    /// This is a convenience method for when you have a stack-allocated guard
    pub fn fork_owned(&self) -> Self {
        Self {
            current: RefCell::new(HashSet::default()),
            parent: self.parent.clone(),
        }
    }

    /// Check if a type has been visited in current branch or any parent
    pub fn check(&self, type_id: &LuaTypeDeclId) -> Result<(), InferFailReason> {
        // Check in all parent levels first
        if self.contains_in_parents(type_id) {
            return Err(InferFailReason::RecursiveInfer);
        }

        // Check in current level
        let mut current = self.current.borrow_mut();
        if current.contains(type_id) {
            return Err(InferFailReason::RecursiveInfer);
        }

        // Mark as visited in current level
        current.insert(type_id.clone());
        Ok(())
    }

    /// Check if a type has been visited in parent chain
    fn contains_in_parents(&self, type_id: &LuaTypeDeclId) -> bool {
        let mut current_parent = self.parent.as_ref();
        while let Some(parent) = current_parent {
            if parent.current.borrow().contains(type_id) {
                return true;
            }
            current_parent = parent.parent.as_ref();
        }
        false
    }

    /// Check if a type has been visited (without modifying the guard)
    pub fn contains(&self, type_id: &LuaTypeDeclId) -> bool {
        self.current.borrow().contains(type_id) || self.contains_in_parents(type_id)
    }

    /// Get the depth of current level
    pub fn current_depth(&self) -> usize {
        self.current.borrow().len()
    }

    /// Get the total depth of the entire guard chain
    pub fn total_depth(&self) -> usize {
        let mut depth = self.current.borrow().len();
        let mut current_parent = self.parent.as_ref();
        while let Some(parent) = current_parent {
            depth += parent.current.borrow().len();
            current_parent = parent.parent.as_ref();
        }
        depth
    }

    /// Get the level of the guard chain (how many parents)
    pub fn level(&self) -> usize {
        let mut level = 0;
        let mut current_parent = self.parent.as_ref();
        while let Some(parent) = current_parent {
            level += 1;
            current_parent = parent.parent.as_ref();
        }
        level
    }
}
