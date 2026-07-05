use crate::{DIRTY, Result, signals::Flags};

pub struct Layout {}

impl Layout {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    pub fn flags() -> Result<Flags> {
        Ok(DIRTY.with(|f| f.get()))
    }

    pub fn compile() {}

    pub fn tessellate(&mut self) {
        DIRTY.with(|f| f.set(Flags::SIGNALED));
    }
}
