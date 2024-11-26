
use std::ops::Range;

use rand::Rng;

use crate::interval::Interval;

pub type Color = Vec3;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn unit_vector(self) -> Self {
        self / self.length()
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();

        Self {
            x: rng.gen_range(0.0..1.0),
            y: rng.gen_range(0.0..1.0),
            z: rng.gen_range(0.0..1.0),
        }
    }

    pub fn random_range(range: Range<f64>) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            x: rng.gen_range(range.clone()),
            y: rng.gen_range(range.clone()),
            z: rng.gen_range(range.clone()),
        }
    }

    pub fn random_unit_vector() -> Self {
        loop {
            let vec = Self::random_range(-1.0..1.0);
            let length = vec.length_squared();
            if length < 1.0 && length > 1e-160 {
                return vec / length.sqrt();
            }
        }
    }

    pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
        let vec = Self::random_unit_vector();
        match vec.dot(normal) > 0.0 {
            true => vec,
            false => -vec
        }
    }

    pub fn near_zero(self) -> bool {
        const S: f64 = 1e-8;
        self.x.abs() < S && self.y.abs() < S && self.z.abs() < S
    }

    pub fn reflect(self, normal: Vec3) -> Vec3 {
        self - 2.0 * self.dot(normal) * normal
    }

    // inline vec3 refract(const vec3& uv, const vec3& n, double etai_over_etat) {
    //     auto cos_theta = std::fmin(dot(-uv, n), 1.0);
    //     vec3 r_out_perp =  etai_over_etat * (uv + cos_theta*n);
    //     vec3 r_out_parallel = -std::sqrt(std::fabs(1.0 - r_out_perp.length_squared())) * n;
    //     return r_out_perp + r_out_parallel;
    // }
    
    pub fn refract(self, normal: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-self).dot(normal).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * normal);
        let r_out_parallel = (-(1.0 - r_out_perp.length_squared()).abs().sqrt()) * normal;
        r_out_perp + r_out_parallel
    }

    pub fn linear_to_gamme(self) -> Self {
        Self {
            x: self.x.sqrt(),
            y: self.y.sqrt(),
            z: self.z.sqrt(),
        }
    }

    pub fn to_bytes_string(self) -> String {
        let interval = Interval::new(0.0, 0.999);
        let gamma = self.linear_to_gamme();
        format!(
            "{} {} {}",
            (interval.clamp(gamma.x) * 255.999) as i32,
            (interval.clamp(gamma.y) * 255.999) as i32,
            (interval.clamp(gamma.z) * 255.999) as i32
        )
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> std::ops::Add<T> for Vec3
where
    T: Into<f64>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Add<Vec3> for f64 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self + rhs.x,
            y: self + rhs.y,
            z: self + rhs.z,
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> std::ops::Sub<T> for Vec3
where
    T: Into<f64>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T> std::ops::Mul<T> for Vec3
where
    T: Into<f64>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl std::ops::Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl<T> std::ops::Div<T> for Vec3
where
    T: Into<f64>,
{
    type Output = Vec3;

    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {} {}", self.x, self.y, self.z)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        assert_eq!(v1 + v2, Vec3::new(5.0, 7.0, 9.0));
        assert_eq!(v1 - v2, Vec3::new(-3.0, -3.0, -3.0));
        assert_eq!(v1 * v2, Vec3::new(4.0, 10.0, 18.0));
        assert_eq!(v1 / v2, Vec3::new(0.25, 0.4, 0.5));
        assert_eq!(v1 + 1.0, Vec3::new(2.0, 3.0, 4.0));
        assert_eq!(v1 - 1.0, Vec3::new(0.0, 1.0, 2.0));
        assert_eq!(v1 * 2.0, Vec3::new(2.0, 4.0, 6.0));
        assert_eq!(v1 / 2.0, Vec3::new(0.5, 1.0, 1.5));
        assert_eq!(-v1, Vec3::new(-1.0, -2.0, -3.0));
    }

    #[test]
    fn test_cross_product() {
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(0.0, 1.0, 0.0);

        assert_eq!(v1.cross(v2), Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_dot_product() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        assert_eq!(v1.dot(v2), 32.0);
    }
}