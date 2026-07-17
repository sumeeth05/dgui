use std::{cell::RefCell, rc::Rc};

use crate::DIRTY;

#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Flags {
    SIGNALED,
    UNSIGNALED,
}

#[derive(Clone)]
pub struct Signal<T> {
    pub value: Rc<RefCell<T>>,
}

impl<T> Signal<T> {
    pub fn create(value: T) -> Self {
        let data = Rc::new(RefCell::new(value));

        Self { value: data }
    }

    pub fn value(&self) -> T
    where
        T: Clone,
    {
        self.value.borrow().clone()
    }
    pub fn update<R>(&self, f: impl FnOnce(&mut T)) {
        {
            let mut borrow = self.value.borrow_mut();
            f(&mut *borrow);
        }

        DIRTY.with(|flags| flags.set(Flags::SIGNALED));
    }
}
