pub fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest: &i32 = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

pub fn largest_char(list: &[char]) -> &char {
    let mut largest: &char = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

pub fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest: &T = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

pub struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    pub fn point(x: T, y: T) -> Self {
        return Point { x: x, y: y };
    }

    pub fn get_x(&self) -> &T {
        return &self.x;
    }

    pub fn get_y(&self) -> &T {
        return &self.y;
    }
}

impl Point<f32> {
    pub fn distance_from_origin(&self) -> f32 {
        return (self.x.powi(2) + self.y.powi(2)).sqrt();
    }
}

pub struct PointMixed<X1, Y1> {
    pub x: X1,
    pub y: Y1,
}

impl<X1, Y1> PointMixed<X1, Y1> {
    pub fn mixup<X2, Y2>(self, other: PointMixed<X2, Y2>) -> PointMixed<X1, Y2> {
        PointMixed {
            x: self.x,
            y: other.y,
        }
    }
}
