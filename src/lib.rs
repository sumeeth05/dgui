use std::cell::Cell;

use thiserror::Error;

use crate::{
    events::Event,
    signal::{Flags, Value},
    styles::Style,
    widgets::WidgetType,
};

mod events;
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

    pub fn collapsible<F>(
        children: Vec<Widget>,
        styles: Option<Style>,
        expand: impl Into<Value<bool>>,
        ontoggle: F,
    ) -> Self
    where
        F: Fn() + 'static,
    {
        Self {
            type_of: WidgetType::Collapsible {
                expand: expand.into(),
                ontoggle: Box::new(ontoggle),
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

    pub fn button<F1, F2>(
        children: Vec<Widget>,
        styles: Option<Style>,
        onclick: F1,
        onhover: F2,
    ) -> Self
    where
        F1: Fn() + 'static,
        F2: Fn() + 'static,
    {
        Self {
            type_of: WidgetType::Button {
                onclick: Box::new(onclick),
                onhover: Box::new(onhover),
            },
            children: Some(children),
            styles,
        }
    }

    pub fn switch<F>(value: impl Into<Value<bool>>, styles: Option<Style>, ontoggle: F) -> Self
    where
        F: Fn() + 'static,
    {
        Self {
            type_of: WidgetType::Switch {
                checked: value.into(),
                ontoggle: Box::new(ontoggle),
            },
            children: None,
            styles,
        }
    }

    pub fn checkbox<F>(value: impl Into<Value<bool>>, ontoggle: F, styles: Option<Style>) -> Self
    where
        F: Fn() + 'static,
    {
        Self {
            type_of: WidgetType::Checkbox {
                checked: value.into(),
                ontoggle: Box::new(ontoggle),
            },
            children: None,
            styles,
        }
    }

    pub fn radio_button<F>(
        label: impl Into<String>,
        value: impl Into<Value<bool>>,
        onchange: F,
        styles: Option<Style>,
    ) -> Self
    where
        F: Fn() + 'static,
    {
        Self {
            type_of: WidgetType::RadioButton {
                label: label.into(),
                selected: value.into(),
                onchange: Box::new(onchange),
            },
            children: None,
            styles,
        }
    }

    pub fn slider<F>(
        value: impl Into<Value<f64>>,
        min: f64,
        max: f64,
        onchange: F,
        styles: Option<Style>,
    ) -> Self
    where
        F: Fn() + 'static,
    {
        Self {
            type_of: WidgetType::Slider {
                value: value.into(),
                min,
                max,
                onchange: Box::new(onchange),
            },
            children: None,
            styles,
        }
    }

    pub fn drag_value<F>(
        value: impl Into<Value<f64>>,
        min: f64,
        max: f64,
        onchange: F,
        styles: Option<Style>,
    ) -> Self
    where
        F: Fn() + 'static,
    {
        Self {
            type_of: WidgetType::DragValue {
                value: value.into(),
                min,
                max,
                onchange: Box::new(onchange),
            },
            children: None,
            styles,
        }
    }

    pub fn text_input<F>(
        value: impl Into<Value<String>>,
        onchange: F,
        styles: Option<Style>,
    ) -> Self
    where
        F: Fn(Event) + 'static,
    {
        Self {
            type_of: WidgetType::TextInput {
                value: value.into(),
                onchange: Box::new(onchange),
            },
            children: None,
            styles,
        }
    }

    pub fn select<F>(
        label: impl Into<String>,
        options: Vec<String>,
        onchange: F,
        styles: Option<Style>,
    ) -> Self
    where
        F: Fn() + 'static,
    {
        Self {
            type_of: WidgetType::Select {
                label: label.into(),
                options,
                onchange: Box::new(onchange),
            },
            children: None,
            styles,
        }
    }

    pub fn text(text: impl Into<Value<String>>, styles: Option<Style>) -> Self {
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

    pub fn progress(
        value: impl Into<Value<f64>>,
        min: f64,
        max: f64,
        styles: Option<Style>,
    ) -> Self {
        Self {
            type_of: WidgetType::ProgressBar {
                value: value.into(),
                min,
                max,
            },
            children: None,
            styles,
        }
    }

    pub fn link<F>(label: impl Into<String>, onclick: F, styles: Option<Style>) -> Self
    where
        F: Fn() + 'static,
    {
        Self {
            type_of: WidgetType::Hyperlink {
                label: label.into(),
                onclick: Box::new(onclick),
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
