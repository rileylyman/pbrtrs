#![allow(dead_code)]

use std::ops;

pub type Vec2<T> = Vector<T, 2>;
pub type Vec3<T> = Vector<T, 3>;
pub type Vec4<T> = Vector<T, 4>;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Vector<T: Copy, const N: usize> {
    data: [T; N],
}

impl<T: Copy + Default, const N: usize> Default for Vector<T, N> {
    fn default() -> Self {
        Self {
            data: [T::default(); N],
        }
    }
}

impl<T: Copy, const N: usize> Vector<T, N> {
    fn new(v: T) -> Self {
        Self { data: [v; N] }
    }
}

impl<T: Copy> Vec2<T> {
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

impl<T: Copy> Vec3<T> {
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

impl<
        T: ops::Add<Output = T>
            + ops::Mul<Output = T>
            + ops::Sub<Output = T>
            + ops::Neg<Output = T>
            + Copy,
    > Vec3<T>
{
    fn cross(self, other: Self) -> Self {
        let [u1, u2, u3] = self.data;
        let [v1, v2, v3] = other.data;
        Self {
            data: [u2 * v3 - u3 * v2, -(u1 * v3 - u3 * v1), u1 * v2 - u2 * v1],
        }
    }
}

impl<T: Copy> Vec4<T> {
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

impl<T: ops::Add<Output = T> + Default + Copy, const N: usize> ops::Add for Vector<T, N> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut new_data = [T::default(); N];
        for i in 0..N {
            new_data[i] = self.data[i] + other.data[i];
        }
        Self { data: new_data }
    }
}

impl<T: ops::AddAssign + Copy, const N: usize> ops::AddAssign for Vector<T, N> {
    fn add_assign(&mut self, other: Self) {
        for i in 0..N {
            self.data[i] += other.data[i];
        }
    }
}

impl<T: ops::Sub<Output = T> + Default + Copy, const N: usize> ops::Sub for Vector<T, N> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut new_data = [T::default(); N];
        for i in 0..N {
            new_data[i] = self.data[i] - other.data[i];
        }
        Self { data: new_data }
    }
}

impl<T: ops::SubAssign + Copy, const N: usize> ops::SubAssign for Vector<T, N> {
    fn sub_assign(&mut self, other: Self) {
        for i in 0..N {
            self.data[i] -= other.data[i];
        }
    }
}

impl<T: ops::Mul<Output = T> + Default + Copy, const N: usize> ops::Mul<T> for Vector<T, N> {
    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        let mut new_data = [T::default(); N];
        for i in 0..N {
            new_data[i] = self.data[i] * other;
        }
        Self { data: new_data }
    }
}

impl<T: ops::MulAssign + Copy, const N: usize> ops::MulAssign<T> for Vector<T, N> {
    fn mul_assign(&mut self, other: T) {
        for i in 0..N {
            self.data[i] *= other;
        }
    }
}

impl<T: ops::Div<Output = T> + Default + Copy, const N: usize> ops::Div<T> for Vector<T, N> {
    type Output = Self;

    fn div(self, other: T) -> Self::Output {
        let mut new_data = [T::default(); N];
        for i in 0..N {
            new_data[i] = self.data[i] / other;
        }
        Self { data: new_data }
    }
}

impl<T: ops::DivAssign + Copy, const N: usize> ops::DivAssign<T> for Vector<T, N> {
    fn div_assign(&mut self, other: T) {
        for i in 0..N {
            self.data[i] /= other;
        }
    }
}

impl<T: ops::Neg<Output = T> + Copy + Default, const N: usize> ops::Neg for Vector<T, N> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut new_data = [T::default(); N];
        for i in 0..N {
            new_data[i] = -self.data[i];
        }
        Self { data: new_data }
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
        let v2 = Vec2::<u32>::elements(1, 2);
        assert_eq!(v2.x(), 1);
        assert_eq!(v2.y(), 2);

        let v3 = Vec3::<u32>::elements(4, 5, 6);
        assert_eq!(v3.x(), 4);
        assert_eq!(v3.y(), 5);
        assert_eq!(v3.z(), 6);

        let v4 = Vec4::<u32>::elements(7, 8, 9, 10);
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
        assert_eq!(x.cross(y), Vec3::<f32>::elements(0.0, 0.0, 1.0));
    }
}
