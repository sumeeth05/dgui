use std::cell::Cell;

use thiserror::Error;

use crate::{signal::Flags, styles::Style, widgets::Kind};

pub mod layout;
pub mod signal;
pub mod styles;
mod widgets;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {}

pub type Result<T> = std::result::Result<T, Error>;

thread_local! {
    static DIRTY: Cell<Flags> = const { Cell::new(Flags::UNSIGNALED) };
}

pub struct Widget {
    pub kind: Kind,
    pub children: Option<Vec<Widget>>,
    pub styles: Option<Style>,
}

impl Widget {
    pub fn panel(children: Option<Vec<Widget>>, styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::Panel,
            children,
            styles,
        })
    }

    pub fn scrollarea(children: Option<Vec<Widget>>, styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::ScrollArea,
            children,
            styles,
        })
    }

    pub fn tabs(children: Option<Vec<Widget>>, styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::Tabs,
            children,
            styles,
        })
    }

    pub fn tab(children: Option<Vec<Widget>>, styles: Option<Style>, _title: &str) -> Result<Self> {
        Ok(Self {
            kind: Kind::Tab,
            children,
            styles,
        })
    }

    pub fn collapsible(children: Option<Vec<Widget>>, styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::Collapsible,
            children,
            styles,
        })
    }

    pub fn splitter(children: Option<Vec<Widget>>, styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::Splitter,
            children,
            styles,
        })
    }

    pub fn window(children: Option<Vec<Widget>>, styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::Window,
            children,
            styles,
        })
    }

    pub fn button(children: Option<Vec<Widget>>, styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::Button,
            children,
            styles,
        })
    }

    pub fn switch(styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::Switch,
            children: None,
            styles,
        })
    }

    pub fn checkbox(styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::Checkbox,
            children: None,
            styles,
        })
    }

    pub fn radio_button(children: Option<Vec<Widget>>, styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::RadioButton,
            children,
            styles,
        })
    }

    pub fn slider(styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::Slider,
            children: None,
            styles,
        })
    }

    pub fn drag_value(styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::DragValue,
            children: None,
            styles,
        })
    }

    pub fn text_input(styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::TextInput,
            children: None,
            styles,
        })
    }

    pub fn color_picker(styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::ColorPicker,
            children: None,
            styles,
        })
    }

    pub fn combo_box(children: Option<Vec<Widget>>, styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::ComboBox,
            children,
            styles,
        })
    }

    pub fn text(styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::Text,
            children: None,
            styles,
        })
    }

    pub fn rich_text(styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::RichText,
            children: None,
            styles,
        })
    }

    pub fn icon(styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::Icon,
            children: None,
            styles,
        })
    }

    pub fn image(styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::Image,
            children: None,
            styles,
        })
    }

    pub fn progress(styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::ProgressBar,
            children: None,
            styles,
        })
    }

    pub fn link(styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::Hyperlink,
            children: None,
            styles,
        })
    }

    pub fn separator(styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::Separator,
            children: None,
            styles,
        })
    }

    pub fn canvas(children: Option<Vec<Widget>>, styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::Canvas,
            children,
            styles,
        })
    }

    pub fn node(styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::Node,
            children: None,
            styles,
        })
    }
}
