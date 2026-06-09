use crate::Result;

pub struct Rgba {
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
}

impl Rgba {
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Result<Self> {
        Ok(Self {
            red,
            green,
            blue,
            alpha,
        })
    }
}

pub struct Spacing {
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,
}

impl Spacing {
    pub fn new(top: f32, bottom: f32, left: f32, right: f32) -> Result<Self> {
        Ok(Self {
            top,
            bottom,
            left,
            right,
        })
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

#[derive(Default)]
pub struct Style {
    width: Option<f32>,
    height: Option<f32>,
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
    pub fn width(&mut self, width: Option<f32>) {
        self.width = width
    }

    pub fn height(&mut self, height: Option<f32>) {
        self.height = height
    }

    pub fn color(&mut self, color: Option<Rgba>) {
        self.color = color
    }

    pub fn background_color(&mut self, color: Option<Rgba>) {
        self.background_color = color
    }

    pub fn padding(&mut self, padding: Option<Spacing>) {
        self.padding = padding
    }

    pub fn margin(&mut self, margin: Option<Spacing>) {
        self.margin = margin
    }

    pub fn border_radius(&mut self, radius: Option<f32>) {
        self.border_radius = radius
    }

    pub fn border_color(&mut self, color: Option<Rgba>) {
        self.border_color = color
    }

    pub fn border_width(&mut self, width: Option<f32>) {
        self.border_width = width
    }

    pub fn position(&mut self, position: Option<Position>) {
        self.position = position
    }

    pub fn font_size(&mut self, size: Option<f32>) {
        self.font_size = size
    }

    pub fn font_weight(&mut self, weight: Option<f32>) {
        self.font_weight = weight
    }

    pub fn font_style(&mut self, style: Option<FontStyle>) {
        self.font_style = style
    }
}
