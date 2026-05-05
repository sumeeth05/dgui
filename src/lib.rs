mod styles;
mod widgets;

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

pub struct Ui {}

impl Ui {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
}
