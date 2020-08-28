use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct Tetrad {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Tetrad {
    fn new(x: f64, y: f64, z: f64, w: f64) -> Tetrad {
        Tetrad { x, y, z, w }
    }

    pub fn point(x: f64, y: f64, z: f64) -> Tetrad {
        Tetrad::new(x, y, z, 1.0)
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Tetrad {
        Tetrad::new(x, y, z, 0.0)
    }

    pub fn is_point(&self) -> bool {
        (self.w - 1.0).abs() < f64::EPSILON
    }

    pub fn is_vector(&self) -> bool {
        (self.w - 0.0).abs() < f64::EPSILON
    }
}

impl Add<Tetrad> for Tetrad {
    type Output = Tetrad;

    fn add(self, rhs: Tetrad) -> Tetrad {
        Tetrad::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point() {
        let tetrad = Tetrad::point(1.0, 2.0, 3.0);
        assert_eq!(tetrad.is_point(), true);
        assert_eq!(tetrad.is_vector(), false);
    }

    #[test]
    fn test_vector() {
        let tetrad = Tetrad::vector(1.0, 2.0, 3.0);
        assert_eq!(tetrad.is_vector(), true);
        assert_eq!(tetrad.is_point(), false);
    }

    #[test]
    fn test_add() {
        let tetrad1 = Tetrad::new(3.0, -2.0, 5.0, 1.0);
        let tetrad2 = Tetrad::new(-2.0, 3.0, 1.0, 0.0);
        assert_eq!(tetrad1 + tetrad2, Tetrad::new(1.0, 1.0, 6.0, 1.0));
    }
}
