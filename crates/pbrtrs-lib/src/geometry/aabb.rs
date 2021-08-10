use crate::geometry::point::*;
use crate::geometry::vector::*;

struct Bounds<T, const N: usize>
where
    T: Numeric,
{
    p_min: Point<T, N>,
    p_max: Point<T, N>,
}

impl<T, const N: usize> Bounds<T, N>
where
    T: Numeric,
{
    pub fn new(p_min: Point<T, N>, p_max: Point<T, N>) -> Self {
        Self {
            p_min: Point::<T, N>::from_min_components(&p_min, &p_max),
            p_max: Point::<T, N>::from_max_components(&p_min, &p_max),
        }
    }

    pub fn from_single(p: Point<T, N>) -> Self {
        Self { p_min: p, p_max: p }
    }

    // Returns a new AABB that has been expanded to contain the
    // given point.
    pub fn union_with_point(&self, p: Point<T, N>) -> Self {
        Self {
            p_min: Point::<T, N>::from_min_components(&self.p_min, &p),
            p_max: Point::<T, N>::from_max_components(&self.p_max, &p),
        }
    }

    pub fn union(&self, bb: &Self) -> Self {
        Self {
            p_min: Point::<T, N>::from_min_components(&self.p_min, &bb.p_min),
            p_max: Point::<T, N>::from_max_components(&self.p_max, &bb.p_max),
        }
    }

    pub fn intersect(&self, bb: &Self) -> Self {
        Self {
            p_min: Point::<T, N>::from_max_components(&self.p_min, &bb.p_min),
            p_max: Point::<T, N>::from_min_components(&self.p_max, &bb.p_max),
        }
    }
}

impl<T, const N: usize> Default for Bounds<T, N>
where
    T: Numeric,
{
    fn default() -> Self {
        // Create an invalid bounds.
        Self {
            p_min: Point::<T, N>::new(T::m_max_value()),
            p_max: Point::<T, N>::new(T::m_min_value()),
        }
    }
}
