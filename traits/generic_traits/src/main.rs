// Use `Associated`` traits (traits that use the `type` keyword) when there should be only a single
// implementation of a trait per type. However, if there are many possible implementations of the
// trait per type, then use `Generic` traits

trait Addition<T, U> {
    fn add(&self, rhs: T) -> U;
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Addition<Point, Point> for Point {
    fn add(&self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Addition<i32, Point> for Point {
    fn add(&self, rhs: i32) -> Point {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl Addition<Point, Line> for Point {
    fn add(&self, rhs: Point) -> Line {
        Line {
            start: self.to_owned(),
            end: rhs,
        }
    }
}

fn main() {
    // Adding two points
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 2, y: 5 };

    let p3: Point = p1.add(p2);

    println!("p1 + p2 = {:?}", p3);

    // Adding integer to a point
    let p1 = Point { x: 2, y: 3 };
    let p2 = p1.add(3);

    println!("Adding 3 to p1 = {:?}", p2);

    // Creating a Line by adding two Points
    let p1 = Point { x: 2, y: 1 };
    let p2 = Point { x: 3, y: 5 };

    let line: Line = p1.add(p2);

    println!("Start: {:?}   End: {:?}", line.start, line.end);
}
