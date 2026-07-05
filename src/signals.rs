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
pub struct Getter<T> {
    pub value: Rc<RefCell<T>>,
}

impl<T> Getter<T> {
    pub fn get(&self) -> Ref<'_, T> {
        self.value.borrow()
    }
}

#[derive(Clone)]
pub struct Setter<T> {
    pub value: Rc<RefCell<T>>,
}

impl<T> Setter<T> {
    pub fn set(&self, value: T) {
        *self.value.borrow_mut() = value;
        DIRTY.with(|f| f.set(Flags::SIGNALED));
    }
}

pub struct Signal;

impl Signal {
    pub fn create<T>(value: T) -> Result<(Getter<T>, Setter<T>)> {
        let data = Rc::new(RefCell::new(value));

        let getter = Getter {
            value: Rc::clone(&data),
        };
        let setter = Setter { value: data };
        Ok((getter, setter))
    }
}
