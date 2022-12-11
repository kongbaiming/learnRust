pub struct  Rectangle {
    width: u32,
    length: u32,
}


impl Rectangle {
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }
    pub fn set_length(&mut self, length: u32) {
        self.length = length;
    }

    pub fn new(width: u32, length: u32) -> Self {
        Self { width, length }
    }

    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn length(&self) -> u32 {
        self.length
    }
    pub fn area(&self) -> u32 {
        self.width * self.length
    }
}


pub fn area1(rect: &Rectangle) -> u32 {
    rect.width * rect.length
}