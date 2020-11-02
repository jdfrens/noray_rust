use std::ops::{Add, Div, Mul, Neg, Sub};

/// Representation of a point.
#[derive(Debug, PartialEq)]
pub struct Point {
    /// x coordinate
    x: f64,
    /// y coordinate
    y: f64,
    /// z coordinate
    z: f64,
    /// w coordinate; always 1.0
    w: f64,
}

/// Representation of a vector.
#[derive(Debug, PartialEq)]
pub struct Vector {
    /// x coordinate
    x: f64,
    /// y coordinate
    y: f64,
    /// z coordinate
    z: f64,
    /// w coordinate; 1.0 for points, 0.0 for vectors
    w: f64,
}

impl Point {
    /// Returns a new point.
    ///
    /// # Arguments
    ///
    /// * `x` the x coordinate
    /// * `y` the y coordinate
    /// * `z` the z coordinate
    ///
    /// # Examples
    ///
    /// ```
    /// # use noray::math::Point;
    /// let point: Point = Point::new(1.0, 2.0, 3.0);
    /// ```
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z, w: 1.0 }
    }
}

impl Vector {
    /// Returns a new vector.
    ///
    /// # Arguments
    ///
    /// * `x` the x coordinate
    /// * `y` the y coordinate
    /// * `z` the z coordinate
    ///
    /// # Examples
    ///
    /// ```
    /// # use noray::math::Vector;
    /// let vector: Vector = Vector::new(1.0, 2.0, 3.0);
    /// ```
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z, w: 0.0 }
    }

    /// Returns the magnitude of a vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use noray::math::Vector;
    /// let vector: Vector = Vector::new(1.0, 2.0, 3.0);
    /// let magnitude: f64 = vector.magnitude();
    /// ```
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    /// Returns a normal version of a vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use noray::math::Vector;
    /// let vector = Vector::new(1.0, 2.0, 3.0);
    /// let normalized: Vector = vector.normalize();
    /// ```
    pub fn normalize(&self) -> Vector {
        self / self.magnitude()
    }

    /// Returns the dot product of two vectors.
    ///
    /// # Arguments
    ///
    /// * `rhs` the second vector in the product
    ///
    /// # Examples
    ///
    /// ```
    /// # use noray::math::Vector;
    /// let vector1: Vector = Vector::new(1.0, 2.0, 3.0);
    /// let vector2: Vector = Vector::new(7.0, 8.0, 9.0);
    /// let dot_product: f64 = vector1.dot(&vector2);
    /// ```
    pub fn dot(&self, rhs: &Vector) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    /// Returns the cross product of two vectors in a left-handed coordinate system.
    ///
    /// # Arguments
    ///
    /// * `rhs` the second vector in the product
    ///
    /// # Examples
    ///
    /// ```
    /// # use noray::math::Vector;
    /// let vector1: Vector = Vector::new(1.0, 2.0, 3.0);
    /// let vector2: Vector = Vector::new(7.0, 8.0, 9.0);
    /// let cross_product: Vector = vector1.cross(&vector2);
    /// ```
    pub fn cross(&self, rhs: &Vector) -> Vector {
        Vector::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    /// Returns the sum of two vectors.
    ///
    /// # Arguments
    ///
    /// * `rhs` the other vector
    ///
    /// # Example
    ///
    /// ```
    /// # use noray::math::Vector;
    /// let tetrad1: Vector = Vector::new(1.0, 2.0, 3.0);
    /// let tetrad2: Vector = Vector::new(9.0, 8.0, 7.0);
    /// let sum: Vector = tetrad1 + tetrad2;
    /// ```
    fn add(self, rhs: Vector) -> Vector {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    /// Returns the sum of a point and a vector.
    ///
    /// # Arguments
    ///
    /// * `rhs` the vector
    ///
    /// # Example
    ///
    /// ```
    /// # use noray::math::{Point, Vector};
    /// let point: Point = Point::new(1.0, 2.0, 3.0);
    /// let vector: Vector = Vector::new(9.0, 8.0, 7.0);
    /// let sum: Point = point + vector;
    /// ```
    fn add(self, rhs: Vector) -> Point {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Div<f64> for &Vector {
    type Output = Vector;

    /// Returns a new tetrad scaled by the inverse of the factor.
    ///
    /// # Arguments
    ///
    /// * `rhs` the divisor
    ///
    /// # Examples
    ///
    /// ```
    /// # use noray::math::Vector;
    /// let vector: Vector = Vector::new(1.0, 2.0, 3.0);
    /// let scaled_vector: Vector = &vector / 5.0;
    /// ```
    fn div(self, rhs: f64) -> Vector {
        Vector::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Mul<f64> for &Vector {
    type Output = Vector;

    /// Returns a new tetrad scale by the factor.
    ///
    /// # Arguments
    ///
    /// * `factor` the multiplicative factor
    ///
    /// # Examples
    ///
    /// ```
    /// # use noray::math::Vector;
    /// let vector: Vector = Vector::new(1.0, 2.0, 3.0);
    /// let scaled_vector: Vector = &vector * 5.0;
    /// ```
    fn mul(self, rhs: f64) -> Vector {
        Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Neg for Vector {
    type Output = Vector;

    /// Returns the negation of a vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use noray::math::Vector;
    /// let vector: Vector = Vector::new(1.0, 2.0, 3.0);
    /// let negation: Vector = -vector;
    /// ```
    fn neg(self) -> Vector {
        Vector::new(-self.x, -self.y, -self.z)
    }
}

impl Sub<Point> for Point {
    type Output = Vector;

    /// Returns the vector representing the difference between two points.
    ///
    /// # Arguments
    ///
    /// * `rhs` the other point
    ///
    /// # Example
    ///
    /// ```
    /// # use noray::math::{Point, Vector};
    /// let point1: Point = Point::new(1.0, 2.0, 3.0);
    /// let point2: Point = Point::new(9.0, 8.0, 7.0);
    /// let difference: Vector = point1 - point2;
    /// ```
    fn sub(self, rhs: Point) -> Vector {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    /// Returns the vector representing the difference between two points.
    ///
    /// # Arguments
    ///
    /// * `rhs` the other point
    ///
    /// # Example
    ///
    /// ```
    /// # use noray::math::{Point, Vector};
    /// let point: Point = Point::new(1.0, 2.0, 3.0);
    /// let vector: Vector = Vector::new(9.0, 8.0, 7.0);
    /// let difference: Point = point - vector;
    /// ```
    fn sub(self, rhs: Vector) -> Point {
        Point::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    /// Returns the vector representing the difference between two points.
    ///
    /// # Arguments
    ///
    /// * `rhs` the other point
    ///
    /// # Example
    ///
    /// ```
    /// # use noray::math::Vector;
    /// let vector1: Vector = Vector::new(1.0, 2.0, 3.0);
    /// let vector2: Vector = Vector::new(9.0, 8.0, 7.0);
    /// let difference: Vector = vector1 - vector2;
    /// ```
    fn sub(self, rhs: Vector) -> Vector {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magnitude() {
        let vector = Vector::new(1.0, 0.0, 0.0);
        assert_eq!(vector.magnitude(), 1.0);
        let vector = Vector::new(0.0, 1.0, 0.0);
        assert_eq!(vector.magnitude(), 1.0);
        let vector = Vector::new(0.0, 0.0, 1.0);
        assert_eq!(vector.magnitude(), 1.0);
        let vector = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(vector.magnitude(), 14.0_f64.sqrt());
    }

    #[test]
    fn test_normalize() {
        let vector = Vector::new(4.0, 0.0, 0.0);
        assert_eq!(vector.normalize(), Vector::new(1.0, 0.0, 0.0));
        let vector = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(
            vector.normalize(),
            Vector::new(0.2672612419124244, -0.5345224838248488, 0.8017837257372732)
        );
    }

    #[test]
    fn test_dot() {
        let vector1 = Vector::new(1.0, 2.0, 3.0);
        let vector2 = Vector::new(2.0, 3.0, 4.0);
        assert_eq!(vector1.dot(&vector2), 20.0);
        assert_eq!(vector2.dot(&vector1), 20.0);
    }

    #[test]
    fn test_cross() {
        let vector1 = Vector::new(1.0, 2.0, 3.0);
        let vector2 = Vector::new(2.0, 3.0, 4.0);
        assert_eq!(vector1.cross(&vector2), Vector::new(-1.0, 2.0, -1.0));
        assert_eq!(vector2.cross(&vector1), Vector::new(1.0, -2.0, 1.0));
    }

    #[test]
    fn test_add_point_and_vector() {
        let point = Point::new(3.0, -2.0, 5.0);
        let vector = Vector::new(-2.0, 3.0, 1.0);
        assert_eq!(point + vector, Point::new(1.0, 1.0, 6.0));
    }

    #[test]
    fn test_add_vector_and_vector() {
        let vector1 = Vector::new(3.0, -2.0, 5.0);
        let vector2 = Vector::new(-2.0, 3.0, 1.0);
        assert_eq!(vector1 + vector2, Vector::new(1.0, 1.0, 6.0));
    }

    #[test]
    fn test_div_f64() {
        let vector = Vector::new(3.0, -2.0, 5.0);
        assert_eq!(&vector / 2.0, Vector::new(1.5, -1.0, 2.5));
    }

    #[test]
    fn test_mul_f64() {
        let vector = Vector::new(3.0, -2.0, 5.0);
        assert_eq!(&vector * 2.0, Vector::new(6.0, -4.0, 10.0));
    }

    #[test]
    fn test_neg() {
        let vector = Vector::new(3.0, -2.0, 5.0);
        assert_eq!(-vector, Vector::new(-3.0, 2.0, -5.0));
    }

    #[test]
    fn test_sub_point_and_point() {
        let point1 = Point::new(3.0, -2.0, 5.0);
        let point2 = Point::new(-2.0, 3.0, 1.0);
        assert_eq!(point1 - point2, Vector::new(5.0, -5.0, 4.0));
    }

    #[test]
    fn test_sub_point_and_vector() {
        let point = Point::new(3.0, -2.0, 5.0);
        let vector = Vector::new(-2.0, 3.0, 1.0);
        assert_eq!(point - vector, Point::new(5.0, -5.0, 4.0));
    }

    #[test]
    fn test_sub_vector_and_vector() {
        let vector1 = Vector::new(3.0, -2.0, 5.0);
        let vector2 = Vector::new(-2.0, 3.0, 1.0);
        assert_eq!(vector1 - vector2, Vector::new(5.0, -5.0, 4.0));
    }
}
