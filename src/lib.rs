use std::cell::Cell;

use thiserror::Error;

use crate::{signal::Flags, styles::Style, widgets::WidgetType};

pub mod layout;
pub mod signal;
pub mod styles;
mod text;
mod widgets;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {}

pub type Result<T> = std::result::Result<T, Error>;

thread_local! {
    static DIRTY: Cell<Flags> = const { Cell::new(Flags::UNSIGNALED) };
}

pub struct Widget {
    pub type_of: WidgetType,
    pub children: Option<Vec<Widget>>,
    pub styles: Option<Style>,
}

impl Widget {
    pub fn panel(children: Vec<Widget>, styles: Option<Style>) -> Self {
        Self {
            type_of: WidgetType::Panel,
            children: Some(children),
            styles,
        }
    }

    pub fn scrollarea(children: Vec<Widget>, styles: Option<Style>) -> Self {
        Self {
            type_of: WidgetType::ScrollArea,
            children: Some(children),
            styles,
        }
    }

    pub fn tabs(children: Vec<Widget>, styles: Option<Style>) -> Self {
        Self {
            type_of: WidgetType::Tabs,
            children: Some(children),
            styles,
        }
    }

    pub fn tab(children: Vec<Widget>, styles: Option<Style>, label: String) -> Self {
        Self {
            type_of: WidgetType::Tab { label },
            children: Some(children),
            styles,
        }
    }

    pub fn collapsible(
        children: Vec<Widget>,
        styles: Option<Style>,
        expand: bool,
        ontoggle: Box<dyn Fn(bool)>,
    ) -> Self {
        Self {
            type_of: WidgetType::Collapsible {
                expand,
                ontoggle: Some(ontoggle),
            },
            children: Some(children),
            styles,
        }
    }

    pub fn splitter(children: Vec<Widget>, styles: Option<Style>) -> Self {
        Self {
            type_of: WidgetType::Splitter,
            children: Some(children),
            styles,
        }
    }

    pub fn window(children: Vec<Widget>, styles: Option<Style>) -> Self {
        Self {
            type_of: WidgetType::Window,
            children: Some(children),
            styles,
        }
    }

    pub fn button(
        children: Vec<Widget>,
        styles: Option<Style>,
        onclick: Option<Box<dyn Fn()>>,
        onhover: Option<Box<dyn Fn()>>,
    ) -> Self {
        Self {
            type_of: WidgetType::Button { onclick, onhover },
            children: Some(children),
            styles,
        }
    }

    pub fn switch(checked: bool, styles: Option<Style>, ontoggle: Box<dyn Fn(bool)>) -> Self {
        Self {
            type_of: WidgetType::Switch {
                checked,
                ontoggle: Some(ontoggle),
            },
            children: None,
            styles,
        }
    }

    pub fn checkbox(
        checked: bool,
        ontoggle: Option<Box<dyn Fn(bool)>>,
        styles: Option<Style>,
    ) -> Self {
        Self {
            type_of: WidgetType::Checkbox { checked, ontoggle },
            children: None,
            styles,
        }
    }

    pub fn radio_button(
        label: impl Into<String>,
        selected: bool,
        onchange: Option<Box<dyn Fn()>>,
        styles: Option<Style>,
    ) -> Self {
        Self {
            type_of: WidgetType::RadioButton {
                label: label.into(),
                selected,
                onchange,
            },
            children: None,
            styles,
        }
    }

    pub fn slider(
        value: f64,
        min: f64,
        max: f64,
        onchange: Option<Box<dyn Fn(f64)>>,
        styles: Option<Style>,
    ) -> Self {
        Self {
            type_of: WidgetType::Slider {
                value,
                min,
                max,
                onchange,
            },
            children: None,
            styles,
        }
    }

    pub fn drag_value(
        value: f64,
        min: f64,
        max: f64,
        onchange: Option<Box<dyn Fn(f64)>>,
        styles: Option<Style>,
    ) -> Self {
        Self {
            type_of: WidgetType::DragValue {
                value,
                min,
                max,
                onchange,
            },
            children: None,
            styles,
        }
    }

    pub fn text_input(
        value: impl Into<String>,
        onchange: Option<Box<dyn Fn(String)>>,
        styles: Option<Style>,
    ) -> Self {
        Self {
            type_of: WidgetType::TextInput {
                value: value.into(),
                onchange,
            },
            children: None,
            styles,
        }
    }

    pub fn select(
        label: impl Into<String>,
        options: Vec<String>,
        onchange: Option<Box<dyn Fn(String)>>,
        styles: Option<Style>,
    ) -> Self {
        Self {
            type_of: WidgetType::Select {
                label: label.into(),
                options,
                onchange,
            },
            children: None,
            styles,
        }
    }

    pub fn text(text: impl Into<String>, styles: Option<Style>) -> Self {
        Self {
            type_of: WidgetType::Text { text: text.into() },
            children: None,
            styles,
        }
    }

    pub fn icon(source: impl Into<String>, styles: Option<Style>) -> Self {
        Self {
            type_of: WidgetType::Icon {
                source: source.into(),
            },
            children: None,
            styles,
        }
    }

    pub fn image(source: impl Into<String>, styles: Option<Style>) -> Self {
        Self {
            type_of: WidgetType::Image {
                source: source.into(),
            },
            children: None,
            styles,
        }
    }

    pub fn progress(value: f64, min: f64, max: f64, styles: Option<Style>) -> Self {
        Self {
            type_of: WidgetType::ProgressBar { value, min, max },
            children: None,
            styles,
        }
    }

    pub fn link(
        label: impl Into<String>,
        onclick: Option<Box<dyn Fn()>>,
        styles: Option<Style>,
    ) -> Self {
        Self {
            type_of: WidgetType::Hyperlink {
                label: label.into(),
                onclick,
            },
            children: None,
            styles,
        }
    }

    pub fn separator(styles: Option<Style>) -> Self {
        Self {
            type_of: WidgetType::Separator,
            children: None,
            styles,
        }
    }

    pub fn canvas(children: Vec<Widget>, styles: Option<Style>) -> Self {
        Self {
            type_of: WidgetType::Canvas,
            children: Some(children),
            styles,
        }
    }
}
