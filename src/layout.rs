use crate::{DIRTY, Result, Widget, signal::Flags};

pub struct Layout {
    pub tree: Widget,
}

impl Layout {
    pub fn new(layout: Widget) -> Result<Self> {
        Ok(Self { tree: layout })
    }

    pub fn flags() -> Result<Flags> {
        Ok(DIRTY.with(|f| f.get()))
    }

    pub fn build(&mut self) {
        // building ui
        DIRTY.with(|f| f.set(Flags::UNSIGNALED));
    }
}
