use std::fmt::Debug;

trait ClosedShape: Debug {
    fn area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl ClosedShape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Debug for Rectangle {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        print!("Rectangle(w: {:?}, h: {:?})", self.width, self.height);
        Ok(())
    }
}
struct Circle {
    radius: f64,
}

impl ClosedShape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}
impl Debug for Circle {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        print!("Circle(r: {:?})", self.radius);
        Ok(())
    }
}

fn main() {
    let circle = Circle { radius: 45.3 };

    let rectangle = Rectangle {
        height: 65.2,
        width: 64.2,
    };

    let shapes: Vec<Box<dyn ClosedShape>> = vec![Box::new(circle), Box::new(rectangle)];

    println!("{:?}", shapes)
}
