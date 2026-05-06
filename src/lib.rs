use thiserror::Error;

mod input;
mod layout;
mod styles;
mod tessellation;
mod widgets;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Gui {}

impl Gui {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
}
