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
    pub widget_type: WidgetType,
    pub children: Option<Vec<Widget>>,
    pub styles: Option<Style>,
}

impl Widget {
    pub fn panel(children: Vec<Widget>, styles: Option<Style>) -> Self {
        Self {
            widget_type: WidgetType::Panel,
            children: Some(children),
            styles,
        }
    }

    pub fn scrollarea(children: Vec<Widget>, styles: Option<Style>) -> Self {
        Self {
            widget_type: WidgetType::ScrollArea,
            children: Some(children),
            styles,
        }
    }

    pub fn tabs(children: Vec<Widget>, styles: Option<Style>) -> Self {
        Self {
            widget_type: WidgetType::Tabs,
            children: Some(children),
            styles,
        }
    }

    pub fn tab(children: Vec<Widget>, styles: Option<Style>, label: String) -> Self {
        Self {
            widget_type: WidgetType::Tab { label },
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
            widget_type: WidgetType::Collapsible {
                expand,
                ontoggle: Some(ontoggle),
            },
            children: Some(children),
            styles,
        }
    }

    pub fn splitter(children: Vec<Widget>, styles: Option<Style>) -> Self {
        Self {
            widget_type: WidgetType::Splitter,
            children: Some(children),
            styles,
        }
    }

    pub fn window(children: Vec<Widget>, styles: Option<Style>) -> Self {
        Self {
            widget_type: WidgetType::Window,
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
            widget_type: WidgetType::Button { onclick, onhover },
            children: Some(children),
            styles,
        }
    }

    pub fn switch(checked: bool, styles: Option<Style>, ontoggle: Box<dyn Fn(bool)>) -> Self {
        Self {
            widget_type: WidgetType::Switch {
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
            widget_type: WidgetType::Checkbox { checked, ontoggle },
            children: None,
            styles,
        }
    }

    pub fn radio_button(
        label: String,
        selected: bool,
        onchange: Option<Box<dyn Fn()>>,
        styles: Option<Style>,
    ) -> Self {
        Self {
            widget_type: WidgetType::RadioButton {
                label,
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
            widget_type: WidgetType::Slider {
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
            widget_type: WidgetType::DragValue {
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
        value: String,
        onchange: Option<Box<dyn Fn(String)>>,
        styles: Option<Style>,
    ) -> Self {
        Self {
            widget_type: WidgetType::TextInput { value, onchange },
            children: None,
            styles,
        }
    }

    pub fn select(
        label: String,
        options: Vec<String>,
        onchange: Option<Box<dyn Fn(String)>>,
        styles: Option<Style>,
    ) -> Self {
        Self {
            widget_type: WidgetType::Select {
                label,
                options,
                onchange,
            },
            children: None,
            styles,
        }
    }

    pub fn text(text: String, styles: Option<Style>) -> Self {
        Self {
            widget_type: WidgetType::Text { text },
            children: None,
            styles,
        }
    }

    pub fn icon(source: String, styles: Option<Style>) -> Self {
        Self {
            widget_type: WidgetType::Icon { source },
            children: None,
            styles,
        }
    }

    pub fn image(source: String, styles: Option<Style>) -> Self {
        Self {
            widget_type: WidgetType::Image { source },
            children: None,
            styles,
        }
    }

    pub fn progress(
        value: f64,
        min: f64,
        max: f64,
        onchange: Option<Box<dyn Fn()>>,
        styles: Option<Style>,
    ) -> Self {
        Self {
            widget_type: WidgetType::ProgressBar {
                value,
                min,
                max,
                onchange,
            },
            children: None,
            styles,
        }
    }

    pub fn link(label: String, onclick: Option<Box<dyn Fn()>>, styles: Option<Style>) -> Self {
        Self {
            widget_type: WidgetType::Hyperlink { label, onclick },
            children: None,
            styles,
        }
    }

    pub fn separator(styles: Option<Style>) -> Self {
        Self {
            widget_type: WidgetType::Separator,
            children: None,
            styles,
        }
    }

    pub fn canvas(children: Vec<Widget>, styles: Option<Style>) -> Self {
        Self {
            widget_type: WidgetType::Canvas,
            children: Some(children),
            styles,
        }
    }
}
