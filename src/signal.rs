use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use crate::{DIRTY, Result};

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
    pub fn create(value: T) -> Result<Self> {
        let data = Rc::new(RefCell::new(value));

        Ok(Self { value: data })
    }

    pub fn value(&self) -> Ref<'_, T> {
        self.value.borrow()
    }

    pub fn update<R>(&self, f: impl FnOnce(&mut T)) {
        {
            let mut borrow = self.value.borrow_mut();
            f(&mut *borrow);
        }

        DIRTY.with(|flags| flags.set(Flags::SIGNALED));
    }
}
