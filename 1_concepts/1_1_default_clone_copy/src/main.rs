use crate::task::{Point, Polyline};

mod task {
    #[derive(Clone, Copy, Default)]
    pub struct Point {
        x: isize,
        y: isize,
    }

    impl Point {
        pub fn x(&self) -> isize {
            self.x
        }

        pub fn y(&self) -> isize {
            self.y
        }
    }

    #[derive(Clone)]
    pub struct Polyline {
        points: Vec<Point>,
    }

    impl Polyline {
        pub fn new(init_point: Point) -> Self {
            Self {
                points: vec![init_point],
            }
        }

        pub fn len(&self) -> usize {
            self.points.len()
        }

        pub fn print_points(&self) {
            for p in &self.points {
                println!("Point: {} {}", p.x(), p.y());
            }
        }
    }
}

fn main() {
    let p = Point::default();
    
    // does trully have default?
    assert!(p.x() == 0 && p.y() == 0);

    let p_copy = p;

    // is a copy? p is still accessable
    println!("Point p: {} {}", p.x(), p.y());


    let polyline = Polyline::new(
        p
    );

    // is empty?
    assert!(polyline.len() != 0);

    // if we move polyline, it becomes inaccessible because the type is not Copy
    // let polypine_copy = polyline;
    // println!("polyline len: {}", polyline.points.len());

    let polyline_clone = polyline.clone();

    // polyline is still accessable after clonning
    println!("Polyline len: {}", polyline.len());

    polyline.print_points();
}
