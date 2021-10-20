//Color
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub fn to_rgba(&self) -> image::Rgba<u8> {
        image::Rgba::<u8>([self.red as u8,self.green as u8,self.blue as u8,0])
    }
    pub fn add(&self, color: Color) -> Color {
        let red = self.red + color.red;
        let green = self.green + color.green;
        let blue = self.blue + color.blue;
        Color {red, green, blue}
    }
    pub fn multiply(&self, color: Color) -> Color {
        let red = self.red * color.red;
        let green = self.green * color.green;
        let blue = self.blue * color.blue;
        Color {red, green, blue}
    }
    pub fn multiply_scalar(&self, scalar: f32) -> Color {
        Color {
            red: self.red * scalar,
            green: self.green * scalar,
            blue: self.blue * scalar,
        }
    }
    pub fn clamp(self) -> Color {
        let red = (self.red).min(255.);
        let green = (self.green).min(255.);
        let blue = (self.blue).min(255.);
        Color {red, green, blue}
    }
}