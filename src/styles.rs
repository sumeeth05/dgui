pub struct Rgba {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl Rgba {
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }
}

pub struct Spacing {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

impl Spacing {
    pub fn new(top: f32, bottom: f32, left: f32, right: f32) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
        }
    }

    pub fn uniform(value: f32) -> Self {
        Self {
            top: value,
            bottom: value,
            left: value,
            right: value,
        }
    }

    pub fn axis(x: f32, y: f32) -> Self {
        Self {
            top: y,
            bottom: y,
            left: x,
            right: x,
        }
    }
}

pub enum Position {
    Absolute,
    Relative,
    Sticky,
}

pub enum FontStyle {
    Normal,
    Italic,
}

pub enum Dimension {
    Auto,
    Px(f32),
    Percent(f32),
}

#[derive(Default)]
pub struct Style {
    width: Option<Dimension>,
    height: Option<Dimension>,
    color: Option<Rgba>,
    background_color: Option<Rgba>,
    padding: Option<Spacing>,
    margin: Option<Spacing>,
    border_radius: Option<f32>,
    border_color: Option<Rgba>,
    border_width: Option<f32>,
    position: Option<Position>,
    font_size: Option<f32>,
    font_weight: Option<f32>,
    font_style: Option<FontStyle>,
}

impl Style {
    pub fn width(&mut self, width: Dimension) {
        self.width = Some(width)
    }

    pub fn height(&mut self, height: Dimension) {
        self.height = Some(height)
    }

    pub fn color(&mut self, color: Rgba) {
        self.color = Some(color)
    }

    pub fn background_color(&mut self, color: Rgba) {
        self.background_color = Some(color)
    }

    pub fn padding(&mut self, padding: Spacing) {
        self.padding = Some(padding)
    }

    pub fn margin(&mut self, margin: Spacing) {
        self.margin = Some(margin)
    }

    pub fn border_radius(&mut self, radius: f32) {
        self.border_radius = Some(radius)
    }

    pub fn border_color(&mut self, color: Rgba) {
        self.border_color = Some(color)
    }

    pub fn border_width(&mut self, width: f32) {
        self.border_width = Some(width)
    }

    pub fn position(&mut self, position: Position) {
        self.position = Some(position)
    }

    pub fn font_size(&mut self, size: f32) {
        self.font_size = Some(size)
    }

    pub fn font_weight(&mut self, weight: f32) {
        self.font_weight = Some(weight)
    }

    pub fn font_style(&mut self, style: FontStyle) {
        self.font_style = Some(style)
    }
}
