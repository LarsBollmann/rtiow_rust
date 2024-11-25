#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interval<T=f64>
 {
    pub min: T,
    pub max: T,
}

impl<T> Interval<T> {
    pub fn new(min: T, max: T) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, value: T) -> bool
    where
        T: PartialOrd,
    {
        self.min <= value && value <= self.max
    }

    pub fn surrounds(&self, element: T) -> bool
    where
        T: PartialOrd,
    {
        self.min < element && element < self.max
    }

    pub fn surrounds_range(&self, other: &Self) -> bool
    where
        T: PartialOrd,
    {
        self.min < other.min && other.max < self.max
    }

    pub fn clamp(&self, value: T) -> T
    where
        T: PartialOrd + Copy,
    {
        if value < self.min {
            self.min
        } else if value > self.max {
            self.max
        } else {
            value
        }
    }
}