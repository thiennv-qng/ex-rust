use std::{fmt::Debug, ops::Add};

fn add<T: Add<Output = T>>(a: T, b: T) -> T {
  a + b
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Point0 {
  x: u64,
  y: u64,
}

impl Add for Point0 {
  type Output = Self;
  fn add(self, p: Point0) -> Point0 {
    Point0 {
      x: self.x + p.x,
      y: self.y + p.y,
    }
  }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Point<T> {
  x: T,
  y: T,
}

impl<T: Add<Output = T>> Add for Point<T> {
  type Output = Self;
  fn add(self, p: Point<T>) -> Point<T> {
    Point {
      x: self.x + p.x,
      y: self.y + p.y,
    }
  }
}

fn longest<'a>(x: &'a String, y: &'a String) -> &'a String {
  if x.len() >= y.len() {
    x
  } else {
    y
  }
}

#[derive(Debug)]
struct Message<'a, T: Debug> {
  msg: &'a T,
}

impl<'a, T: Debug> Message<'a, T> {
  pub fn log(&self) {
    println!("{:?}", self.msg)
  }
}

fn main() {
  let a = add::<u64>(1, 2);
  let b = add::<f32>(1_f32, 2.0);
  println!("Add: {a} {b}");

  let c = Point0 { x: 1, y: 2 };
  let d = Point0 { x: 3, y: 4 };
  println!("Point0: {:?}", c + d);

  let e = Point { x: 5, y: 6 };
  let f = Point { x: 7, y: 8 };
  println!("Point: {:?}", e.add(f));

  let g = "long".to_string();
  let rs;
  {
    let h = "short ne".to_string();
    rs = longest(&g, &h).clone();
  }
  println!("Longest: {}", rs);

  let m: Message<String>;
  let r: String;
  {
    let i = "this is message".to_string();
    r = i;
    m = Message { msg: &r };
  }
  m.log()
}
