fn main() {
    println!("Hello, world!");

    let rect1 = Rectangle { width: 30, height: 50 };
    println!(
        "The area of rectangle is {} square pixels.",
        &rect1.area()
    );

}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle { x, y, radius }
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}


pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // &self = self: &Self
    // &Self = &Rectangle
    // &self 不可变借用, 如若修改&self, 需要使用&mut self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    // methods name can be same with the member name
    // fn width(&self) -> bool {
    //     self.width > 0
    // }

    // can be used as a getter
    pub fn width(&self) -> u32 {
        return self.width
    }

}
