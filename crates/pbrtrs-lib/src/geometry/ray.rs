use crate::geometry::point::*;
use crate::geometry::vector::*;

struct Ray<T>
where
    T: NumericFloat,
{
    origin: Point3<T>,
    dir: Vec3<T>,

    t_max: T,
    time: T,
    // Medium
}

impl<T> Ray<T>
where
    T: NumericFloat,
{
    pub fn new(o: Point3<T>, d: Vec3<T>) -> Self {
        Self {
            origin: o,
            dir: d,
            ..Default::default()
        }
    }

    pub fn at(&self, t: T) -> Point3<T> {
        self.origin + self.dir * t
    }
}

impl<T> Default for Ray<T>
where
    T: NumericFloat,
{
    fn default() -> Self {
        Self {
            origin: Point3::<T>::new(T::default()),
            dir: Vec3::<T>::new(T::default()),
            t_max: T::m_max_value(),
            time: T::default(),
        }
    }
}
