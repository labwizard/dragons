type Point = (i32, i32);

pub trait Curve {
    fn start(&self) -> Point;

    fn end(&self) -> Point;

    fn top_left(&self) -> Point;

    fn bottom_right(&self) -> Point;

    fn translate(&mut self, dx: i32, dy: i32) -> &mut Self;

    fn rotate_left(&mut self) -> &mut Self;

    fn rotate_right(&mut self) -> &mut Self;

    fn move_start_to(&mut self, new_start: Point) -> &mut Self;

    fn move_end_to(&mut self, new_end: Point) -> &mut Self;

    fn justify(&mut self) -> &mut Self;

    fn expand(self, factor: i32) -> Self;

    fn join(self, other: Self) -> Self;
}

impl Curve for Vec<Point> {
    fn start(&self) -> Point {
        *self.first().unwrap()
    }

    fn end(&self) -> Point {
        *self.last().unwrap()
    }

    fn top_left(&self) -> Point {
        let x = self.iter().map(|(x, _)| *x).min().unwrap();
        let y = self.iter().map(|(_, y)| *y).min().unwrap();
        (x, y)
    }

    fn bottom_right(&self) -> Point {
        let x = self.iter().map(|(x, _)| *x).max().unwrap();
        let y = self.iter().map(|(_, y)| *y).max().unwrap();
        (x, y)
    }

    fn translate(&mut self, dx: i32, dy: i32) -> &mut Self {
        for (x, y) in self.iter_mut() {
            let (new_x, new_y) = (*x + dx, *y + dy);
            (*x, *y) = (new_x, new_y);
        }
        self
    }

    fn rotate_left(&mut self) -> &mut Self {
        for (x, y) in self.iter_mut() {
            let (new_x, new_y) = (-*y, *x);
            (*x, *y) = (new_x, new_y);
        }
        self
    }

    fn rotate_right(&mut self) -> &mut Self {
        for (x, y) in self.iter_mut() {
            let (new_x, new_y) = (*y, -*x);
            (*x, *y) = (new_x, new_y);
        }
        self
    }

    fn move_start_to(&mut self, new_start: Point) -> &mut Self {
        let dx = new_start.0 - self.start().0;
        let dy = new_start.1 - self.start().1;
        self.translate(dx, dy)
    }

    fn move_end_to(&mut self, new_end: Point) -> &mut Self {
        let dx = new_end.0 - self.end().0;
        let dy = new_end.1 - self.end().1;
        self.translate(dx, dy)
    }

    fn justify(&mut self) -> &mut Self {
        let top_left = self.top_left();
        self.translate(-top_left.0, -top_left.1)
    }

    fn expand(self, factor: i32) -> Self {
        let first = self.start();
        let mut new_curve = vec![(first.0 * factor, first.1 * factor)];
        for [point_1, point_2] in self.array_windows::<2>() {
            for i in 1..=factor {
                let j = factor - i;
                let new_x = j * point_1.0 + i * point_2.0;
                let new_y = j * point_1.1 + i * point_2.1;
                new_curve.push((new_x, new_y));
            }
        }
        new_curve
    }

    fn join(mut self, mut other: Self) -> Self {
        let end = self.pop().unwrap();
        other.move_start_to(end);
        self.append(&mut other);
        self
    }
}

pub fn dragon(iterations: usize) -> Vec<Point> {
    match iterations {
        0 => vec![(0, 0), (0, 1)],
        _ => {
            let first_half = dragon(iterations - 1);
            let mut second_half = first_half.clone();
            second_half.reverse();
            second_half.rotate_right();
            let mut full = first_half.join(second_half);
            full.justify();
            full
        }
    }
}
