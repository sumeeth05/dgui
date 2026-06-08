use thiserror::Error;

use crate::{styles::Style, widgets::Kind};

mod styles;
mod widgets;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Widget {
    kind: Kind,
    children: Option<Vec<Widget>>,
    styles: Option<Style>,
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

    pub fn collapsible(children: Option<Vec<Widget>>, styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::Collapsible,
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

    pub fn checkbox(children: Option<Vec<Widget>>, styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::Checkbox,
            children,
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

    pub fn slider(children: Option<Vec<Widget>>, styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::Slider,
            children,
            styles,
        })
    }

    pub fn drag_value(children: Option<Vec<Widget>>, styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::DragValue,
            children,
            styles,
        })
    }

    pub fn text_input(children: Option<Vec<Widget>>, styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::TextInput,
            children,
            styles,
        })
    }

    pub fn color_picker(children: Option<Vec<Widget>>, styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::ColorPicker,
            children,
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

    pub fn text(children: Option<Vec<Widget>>, styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::Text,
            children,
            styles,
        })
    }

    pub fn rich_text(children: Option<Vec<Widget>>, styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::RichText,
            children,
            styles,
        })
    }

    pub fn image(children: Option<Vec<Widget>>, styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::Image,
            children,
            styles,
        })
    }

    pub fn progress(children: Option<Vec<Widget>>, styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::ProgressBar,
            children,
            styles,
        })
    }

    pub fn link(children: Option<Vec<Widget>>, styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::Hyperlink,
            children,
            styles,
        })
    }

    pub fn separator(children: Option<Vec<Widget>>, styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::Separator,
            children,
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

    pub fn node(children: Option<Vec<Widget>>, styles: Option<Style>) -> Result<Self> {
        Ok(Self {
            kind: Kind::Node,
            children,
            styles,
        })
    }
}
