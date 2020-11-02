use std::ops::{Add, Mul, Sub};

/// Representation of a color.
#[derive(Debug, PartialEq)]
pub struct RGB {
    /// red component
    r: f64,
    /// green component
    g: f64,
    /// blue component
    b: f64,
}

impl RGB {
    /// Returns a new color.
    ///
    /// # Arguments
    ///
    /// * `r` the red component
    /// * `g` the green component
    /// * `b` the blue component
    ///
    /// # Examples
    ///
    /// ```
    /// # use noray::color::RGB;
    /// let color: RGB = RGB::new(0.9, 0.6, 0.75);
    /// ```
    pub fn new(r: f64, g: f64, b: f64) -> RGB {
        RGB { r, g, b }
    }
}

impl Add<RGB> for RGB {
    type Output = RGB;

    /// Returns the sum of two RGB colors.
    ///
    /// # Arguments
    ///
    /// * `rhs` the other color
    ///
    /// # Example
    ///
    /// ```
    /// # use noray::color::RGB;
    /// let c1: RGB = RGB::new(1.0, 2.0, 3.0);
    /// let c2: RGB = RGB::new(9.0, 8.0, 7.0);
    /// let sum: RGB = c1 + c2;
    /// ```
    fn add(self, rhs: RGB) -> RGB {
        RGB::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl Sub<RGB> for RGB {
    type Output = RGB;

    /// Returns the difference of two RGB colors.
    ///
    /// # Arguments
    ///
    /// * `rhs` the other color
    ///
    /// # Example
    ///
    /// ```
    /// # use noray::color::RGB;
    /// let c1: RGB = RGB::new(1.0, 2.0, 3.0);
    /// let c2: RGB = RGB::new(9.0, 8.0, 7.0);
    /// let difference: RGB = c1 - c2;
    /// ```
    fn sub(self, rhs: RGB) -> RGB {
        RGB::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl Mul<f64> for RGB {
    type Output = RGB;

    /// Scales the color by the given factor.
    ///
    /// # Arguments
    ///
    /// * `rhs` the scaling factor
    ///
    /// # Example
    ///
    /// ```
    /// # use noray::color::RGB;
    /// let c: RGB = RGB::new(1.0, 2.0, 3.0);
    /// let scaled: RGB = c * 2.0;
    /// ```
    fn mul(self, rhs: f64) -> RGB {
        RGB::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl Mul<RGB> for RGB {
    type Output = RGB;

    /// Returns the product of two RGB colors.
    ///
    /// # Arguments
    ///
    /// * `rhs` the other color
    ///
    /// # Example
    ///
    /// ```
    /// # use noray::color::RGB;
    /// let c1: RGB = RGB::new(1.0, 2.0, 3.0);
    /// let c2: RGB = RGB::new(9.0, 8.0, 7.0);
    /// let difference: RGB = c1 * c2;
    /// ```
    fn mul(self, rhs: RGB) -> RGB {
        RGB::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let c1 = RGB::new(0.9, 0.6, 0.75);
        let c2 = RGB::new(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, RGB::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn test_sub() {
        let c1 = RGB::new(0.9, 0.6, 0.75);
        let c2 = RGB::new(0.7, 0.1, 0.25);
        assert_eq!(c1 - c2, RGB::new(0.20000000000000007, 0.5, 0.5))
    }

    #[test]
    fn test_mul_by_scalar() {
        let c = RGB::new(0.2, 0.3, 0.4);
        assert_eq!(c * 2.0, RGB::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn test_mul() {
        let c1 = RGB::new(1.0, 0.2, 0.4);
        let c2 = RGB::new(0.9, 1.0, 0.1);
        assert_eq!(c1 * c2, RGB::new(0.9, 0.2, 0.04000000000000001));
    }
}
