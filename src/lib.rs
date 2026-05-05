mod styles;
mod widgets;

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

pub struct UI {}

impl UI {
    pub fn init() -> Result<Self> {
        Ok(Self {})
    }
}
