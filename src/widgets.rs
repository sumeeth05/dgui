pub enum WidgetType {
    Panel,
    ScrollArea,
    Tabs,
    Tab {
        label: String,
    },
    Collapsible {
        expand: bool,
        ontoggle: Option<Box<dyn Fn(bool)>>,
    },
    Splitter,
    Window,

    Button {
        onclick: Option<Box<dyn Fn()>>,
        onhover: Option<Box<dyn Fn()>>,
    },
    Checkbox {
        checked: bool,
        ontoggle: Option<Box<dyn Fn(bool)>>,
    },
    Switch {
        checked: bool,
        ontoggle: Option<Box<dyn Fn(bool)>>,
    },
    RadioButton {
        label: String,
        selected: bool,
        onchange: Option<Box<dyn Fn()>>,
    },
    Slider {
        value: f64,
        min: f64,
        max: f64,
        onchange: Option<Box<dyn Fn(f64)>>,
    },
    DragValue {
        value: f64,
        min: f64,
        max: f64,
        onchange: Option<Box<dyn Fn(f64)>>,
    },
    TextInput {
        value: String,
        onchange: Option<Box<dyn Fn(String)>>,
    },
    Select {
        label: String,
        options: Vec<String>,
        onchange: Option<Box<dyn Fn(String)>>,
    },

    Text {
        text: String,
    },

    Icon {
        source: String,
    },

    Image {
        source: String,
    },

    ProgressBar {
        value: f64,
        min: f64,
        max: f64,
    },
    Hyperlink {
        label: String,
        onclick: Option<Box<dyn Fn()>>,
    },

    Separator,

    Canvas,
}
