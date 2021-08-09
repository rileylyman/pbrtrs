#![allow(dead_code)]

mod numeric;

use numeric::*;
use std::ops;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Vector<T, const N: usize>
where
    T: Numeric,
{
    data: [T; N],
}

impl<T, const N: usize> Vector<T, N>
where
    T: Numeric,
{
    fn new(v: T) -> Self {
        Self { data: [v; N] }
    }

    fn dot(&self, other: &Self) -> T {
        let mut accum = T::default();
        for i in 0..N {
            accum += self.data[i] * other.data[i];
        }
        accum
    }

    fn abs_dot(&self, other: &Self) -> T {
        self.dot(other).m_abs()
    }

    fn mag2(&self) -> T {
        self.dot(self)
    }

    fn min_component(&self) -> (usize, T) {
        let (mut dim, mut min) = (0, T::m_max_value());
        for i in 0..N {
            if min > self.data[i] {
                dim = i;
                min = self.data[i];
            }
        }
        (dim, min)
    }

    fn max_component(&self) -> (usize, T) {
        let (mut dim, mut max) = (0, T::m_min_value());
        for i in 0..N {
            if max < self.data[i] {
                dim = i;
                max = self.data[i];
            }
        }
        (dim, max)
    }

    fn face_towards_same_hemisphere(&self, other: &Self) -> Self {
        if self.dot(other) < T::default() {
            -*self
        } else {
            *self
        }
    }

    fn square_distance_to(&self, other: &Self) -> T {
        (*other - *self).mag2()
    }

    fn abs(&self) -> Self {
        let mut new_data = [T::default(); N];
        for i in 0..N {
            new_data[i] = self.data[i].m_abs();
        }
        Self { data: new_data }
    }
}

impl<T, const N: usize> Vector<T, N>
where
    T: NumericFloat,
{
    fn mag(&self) -> T {
        self.mag2().m_sqrt()
    }

    fn cos_theta(&self, other: &Self) -> T {
        self.dot(other) / self.mag() / other.mag()
    }

    fn theta(&self, other: &Self) -> T {
        self.cos_theta(other).m_acos()
    }

    fn normalized(&self) -> Self {
        *self / self.mag()
    }

    fn distance_to(&self, other: &Self) -> T {
        (*other - *self).mag()
    }

    fn lerp(&self, t: T, other: &Self) -> Self {
        *self - *self * t + *other * t
    }

    fn floor(&self) -> Self {
        let mut new_data = [T::default(); N];
        for i in 0..N {
            new_data[i] = self.data[i].m_floor();
        }
        Self { data: new_data }
    }

    fn ceil(&self) -> Self {
        let mut new_data = [T::default(); N];
        for i in 0..N {
            new_data[i] = self.data[i].m_ceil();
        }
        Self { data: new_data }
    }
}

impl<T, const N: usize> Default for Vector<T, N>
where
    T: Numeric,
{
    fn default() -> Self {
        Self {
            data: [T::default(); N],
        }
    }
}

impl<T, const N: usize> ops::Add for Vector<T, N>
where
    T: Numeric,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut new_data = [T::default(); N];
        for i in 0..N {
            new_data[i] = self.data[i] + other.data[i];
        }
        Self { data: new_data }
    }
}

impl<T, const N: usize> ops::AddAssign for Vector<T, N>
where
    T: Numeric,
{
    fn add_assign(&mut self, other: Self) {
        for i in 0..N {
            self.data[i] += other.data[i];
        }
    }
}

impl<T, const N: usize> ops::Sub for Vector<T, N>
where
    T: Numeric,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut new_data = [T::default(); N];
        for i in 0..N {
            new_data[i] = self.data[i] - other.data[i];
        }
        Self { data: new_data }
    }
}

impl<T, const N: usize> ops::SubAssign for Vector<T, N>
where
    T: Numeric,
{
    fn sub_assign(&mut self, other: Self) {
        for i in 0..N {
            self.data[i] -= other.data[i];
        }
    }
}

impl<T, const N: usize> ops::Mul<T> for Vector<T, N>
where
    T: Numeric,
{
    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        let mut new_data = [T::default(); N];
        for i in 0..N {
            new_data[i] = self.data[i] * other;
        }
        Self { data: new_data }
    }
}

impl<T, const N: usize> ops::MulAssign<T> for Vector<T, N>
where
    T: Numeric,
{
    fn mul_assign(&mut self, other: T) {
        for i in 0..N {
            self.data[i] *= other;
        }
    }
}

impl<T, const N: usize> ops::Div<T> for Vector<T, N>
where
    T: Numeric,
{
    type Output = Self;

    fn div(self, other: T) -> Self::Output {
        let mut new_data = [T::default(); N];
        for i in 0..N {
            new_data[i] = self.data[i] / other;
        }
        Self { data: new_data }
    }
}

impl<T, const N: usize> ops::DivAssign<T> for Vector<T, N>
where
    T: Numeric,
{
    fn div_assign(&mut self, other: T) {
        for i in 0..N {
            self.data[i] /= other;
        }
    }
}

impl<T, const N: usize> ops::Neg for Vector<T, N>
where
    T: Numeric,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut new_data = [T::default(); N];
        for i in 0..N {
            new_data[i] = -self.data[i];
        }
        Self { data: new_data }
    }
}

pub type Vec2<T> = Vector<T, 2>;

impl<T> From<(T, T)> for Vec2<T>
where
    T: Numeric,
{
    fn from(other: (T, T)) -> Self {
        Self {
            data: [other.0, other.1],
        }
    }
}

impl<T> Vec2<T>
where
    T: Numeric,
{
    fn elements(x: T, y: T) -> Self {
        Self { data: [x, y] }
    }
    fn x(&self) -> T {
        self.data[0]
    }
    fn y(&self) -> T {
        self.data[1]
    }
}

pub type Vec3<T> = Vector<T, 3>;

impl<T> From<(T, T, T)> for Vec3<T>
where
    T: Numeric,
{
    fn from(other: (T, T, T)) -> Self {
        Self {
            data: [other.0, other.1, other.2],
        }
    }
}

impl<T> Vec3<T>
where
    T: Numeric,
{
    fn elements(x: T, y: T, z: T) -> Self {
        Self { data: [x, y, z] }
    }
    fn x(&self) -> T {
        self.data[0]
    }
    fn y(&self) -> T {
        self.data[1]
    }
    fn z(&self) -> T {
        self.data[2]
    }
}

impl<T> Vec3<T>
where
    T: Numeric,
{
    fn cross(&self, other: &Self) -> Self {
        let [u1, u2, u3] = self.data;
        let [v1, v2, v3] = other.data;
        Self {
            data: [u2 * v3 - u3 * v2, -(u1 * v3 - u3 * v1), u1 * v2 - u2 * v1],
        }
    }
}

impl<T> Vec3<T>
where
    T: NumericFloat,
{
    fn sin_theta(&self, other: &Self) -> T {
        self.cross(other).mag() / self.mag() / other.mag()
    }

    fn spanning_set(&self) -> (Self, Self, Self) {
        let first = self.normalized();
        let second = if first.x().m_abs() > first.y().m_abs() {
            Vec3::<T>::elements(-first.z(), T::default(), first.x())
                / Vec2::<T>::elements(first.x(), first.z()).mag()
        } else {
            Vec3::<T>::elements(T::default(), first.z(), -first.y())
                / Vec2::<T>::elements(first.y(), first.z()).mag()
        };
        (first, second, first.cross(&second))
    }
}

pub type Vec4<T> = Vector<T, 4>;

impl<T> From<(T, T, T, T)> for Vec4<T>
where
    T: Numeric,
{
    fn from(other: (T, T, T, T)) -> Self {
        Self {
            data: [other.0, other.1, other.2, other.3],
        }
    }
}

impl<T> Vec4<T>
where
    T: Numeric,
{
    fn elements(x: T, y: T, z: T, w: T) -> Self {
        Self { data: [x, y, z, w] }
    }
    fn x(&self) -> T {
        self.data[0]
    }
    fn y(&self) -> T {
        self.data[1]
    }
    fn z(&self) -> T {
        self.data[2]
    }
    fn w(&self) -> T {
        self.data[3]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_construction() {
        let v2 = Vec2::<f32>::new(0.0);
        assert_eq!(&v2.data[..], &vec![0.0, 0.0]);

        let v3 = Vec3::<f32>::new(10.0);
        assert_eq!(&v3.data[..], &vec![10.0, 10.0, 10.0]);

        let v4 = Vec4::<f32>::new(-1.0);
        assert_eq!(&v4.data[..], &vec![-1.0, -1.0, -1.0, -1.0]);
    }

    #[test]
    fn test_defaults() {
        let v3 = Vec3::<f32>::default();
        assert_eq!(&v3.data[..], &vec![f32::default(); 3]);
    }

    #[test]
    fn test_accessors() {
        let v2 = Vec2::<i32>::elements(1, 2);
        assert_eq!(v2.x(), 1);
        assert_eq!(v2.y(), 2);

        let v3 = Vec3::<i32>::elements(4, 5, 6);
        assert_eq!(v3.x(), 4);
        assert_eq!(v3.y(), 5);
        assert_eq!(v3.z(), 6);

        let v4 = Vec4::<i32>::elements(7, 8, 9, 10);
        assert_eq!(v4.x(), 7);
        assert_eq!(v4.y(), 8);
        assert_eq!(v4.z(), 9);
        assert_eq!(v4.w(), 10);
    }

    #[test]
    fn test_ops() {
        let v = Vec3::<i32>::elements(1, 2, 3);
        let u = Vec3::<i32>::elements(4, 5, 6);
        assert_eq!(v + u, Vec3::<i32>::elements(5, 7, 9));

        let mut v = Vec3::<i32>::elements(1, 2, 3);
        v += Vec3::<i32>::elements(1, 1, 1);
        assert_eq!(v, Vec3::<i32>::elements(2, 3, 4));

        let v = Vec3::<i32>::elements(1, 2, 3);
        let u = Vec3::<i32>::elements(4, 5, 6);
        assert_eq!(v - u, Vec3::<i32>::elements(-3, -3, -3));

        let mut v = Vec3::<i32>::elements(1, 2, 3);
        v -= Vec3::<i32>::elements(1, 1, 1);
        assert_eq!(v, Vec3::<i32>::elements(0, 1, 2));

        let v = Vec3::<i32>::elements(5, 6, 7);
        assert_eq!(v * 10, Vec3::<i32>::elements(50, 60, 70));

        let mut v = Vec3::<i32>::elements(5, 6, 7);
        v *= 5;
        assert_eq!(v, Vec3::<i32>::elements(25, 30, 35));

        let v = Vec3::<i32>::elements(10, 20, 30);
        assert_eq!(v / 10, Vec3::<i32>::elements(1, 2, 3));

        let mut v = Vec3::<i32>::elements(50, 50, 50);
        v /= 5;
        assert_eq!(v, Vec3::<i32>::elements(10, 10, 10));
    }

    #[test]
    fn test_cross() {
        let x = Vec3::<f32>::elements(1.0, 0.0, 0.0);
        let y = Vec3::<f32>::elements(0.0, 1.0, 0.0);
        assert_eq!(x.cross(&y), Vec3::<f32>::elements(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_dot() {
        assert_eq!(
            Vec3::<f32>::elements(1.0, 2.0, 3.0).dot(&Vec3::<f32>::elements(3.0, 4.0, 5.0)),
            26.0
        );
        assert_eq!(
            Vec3::<f32>::elements(-1.0, -2.0, -3.0).abs_dot(&Vec3::<f32>::elements(3.0, 4.0, 5.0)),
            26.0
        );
        assert_eq!(Vec3::<f32>::elements(2.0, 3.0, 4.0).mag2(), 29.0);
        assert_eq!(
            Vec3::<f32>::elements(2.0, 3.0, 4.0).mag(),
            (29.0 as f32).sqrt()
        );
        assert_eq!(Vec2::<f32>::elements(3.0, 4.0).mag(), 5.0);
        assert_eq!(
            Vec3::<f32>::elements(5.0, 3.0, 15.0).normalized().mag(),
            1.0
        );
    }

    #[test]
    fn test_min_max() {
        assert_eq!(
            Vec3::<f32>::elements(10.0, 50.0, 3.0).max_component(),
            (1, 50.0)
        );
        assert_eq!(
            Vec3::<f32>::elements(10.0, 50.0, 3.0).min_component(),
            (2, 3.0)
        );
    }

    #[test]
    fn test_from_tuple() {
        assert_eq!(Vec2::<f32>::elements(1.0, 3.0), (1.0, 3.0).into());
        assert_eq!(Vec3::<f32>::elements(1.0, 2.0, 3.0), (1.0, 2.0, 3.0).into());
        assert_eq!(
            Vec4::<f32>::elements(1.0, 2.0, 3.0, 4.0),
            (1.0, 2.0, 3.0, 4.0).into()
        );
    }

    #[test]
    fn test_angles() {
        assert_eq!(
            Vec3::<f32>::elements(1.0, 0.0, 0.0)
                .theta(&Vec3::<f32>::elements(0.0, 1.0, 0.0))
                .to_degrees(),
            90.0
        );
    }

    #[test]
    fn test_span() {
        let (x, y, z) = Vec3::<f32>::elements(1.0, 2.0, 3.0).spanning_set();
        assert_eq!(x.dot(&y), 0.0);
        assert_eq!(x.dot(&z), 0.0);
        assert_eq!(z.dot(&y), 0.0);
    }

    #[test]
    fn test_faceforward() {
        let normal = Vec3::<f32>::elements(0.0, 0.0, 1.0);
        assert_eq!(
            normal.face_towards_same_hemisphere(&(0.0, 0.5, -0.5).into()),
            (0.0, 0.0, -1.0).into()
        );
        assert_eq!(
            normal.face_towards_same_hemisphere(&(0.0, 0.5, 0.5).into()),
            (0.0, 0.0, 1.0).into()
        );
    }

    #[test]
    fn test_abs_floor_ceil() {
        assert_eq!(
            Vec3::<f32>::elements(-1.0, -2.0, -3.0).abs(),
            (1.0, 2.0, 3.0).into()
        );
        assert_eq!(
            Vec3::<f32>::elements(1.5, 2.5, 3.5).floor(),
            (1.0, 2.0, 3.0).into()
        );
        assert_eq!(
            Vec3::<f32>::elements(1.5, 2.5, 3.5).ceil(),
            (2.0, 3.0, 4.0).into()
        );
    }

    #[test]
    fn test_distance() {
        assert_eq!(
            Vec3::<f32>::elements(1.0, 0.0, 0.0).distance_to(&(2.0, 0.0, 0.0).into()),
            1.0
        );
        assert_eq!(
            Vec3::<f32>::elements(1.0, 0.0, 0.0).square_distance_to(&(5.0, 0.0, 0.0).into()),
            16.0
        );
    }

    #[test]
    fn test_lerp() {
        assert_eq!(
            Vec3::<f32>::elements(1.0, 0.0, 0.0).lerp(0.5, &(2.0, 0.0, 0.0).into()),
            (1.5, 0.0, 0.0).into()
        );
    }
}
