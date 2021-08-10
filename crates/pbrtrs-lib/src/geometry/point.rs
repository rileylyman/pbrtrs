use crate::geometry::vector::*;

pub type Point2<T> = Vec2<T>;
pub type Point3<T> = Vec3<T>;
pub type Point4<T> = Vec4<T>;

pub type Point<T, const N: usize> = Vector<T, N>;
