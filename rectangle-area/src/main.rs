#[derive(Debug)]
struct Origin(i32, i32);

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
  origin: Origin,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.height * self.width
  }
  fn can_hold(&self, rect: &Rectangle) -> bool {
    self.area() > rect.area()
  }
  fn square(size: u32) -> Rectangle {
    Rectangle {
      width: size,
      height: size,
      origin: Origin(0, 0),
    }
  }
}

fn main() {
  let origin1 = Origin(0, 0);
  let origin2 = Origin(0, 0);
  let rect1 = build_rect(20, 30, origin1);
  let rect2 = build_rect(30, 40, origin2);
  let square1 = Rectangle::square(20);
  let surface = rect1.area();
  println!("Surface: {}", surface);
  println!("{:#?}", rect1);
  println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
  println!("Square is: {:#?}", square1);
}

fn build_rect(w: u32, h: u32, o: Origin) -> Rectangle {
  Rectangle {
    width: w,
    height: h,
    origin: o,
  }
}
