use std::sync::Mutex;

use futures::executor::block_on;

#[derive(Debug, Clone)]
struct Point(i32, i32);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut a = Mutex::new(Point(5, 5));
    let b = Point(10, 20);

    let d = block_on(a.lock())?;
    let c = add(block_on(a.lock()), &b);
    d.0 = 1;

    println!("The sum of {:?} and {:?} is {:?}", a, b, c);
}

fn add(a: &mut Point, b: &Point) -> Point {
    a.0 = 0;
    Point(a.0 + b.0, a.1 + b.1)
}
