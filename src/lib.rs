#![allow(incomplete_features)]
#![feature(const_for, adt_const_params, generic_const_exprs)]

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn vslice_works() {
        let v = Vector::new([1.0f64, 2.0, 3.0]);
        let w = Vector::new([4.0, 6.0]);
        let u = 2.0f64 * v.vslice::<{ 1..3 }>();

        assert_eq!(u, w);
    }

    #[test]
    fn vslice_mut_works() {
        let mut v = Vector::new([1.0f64, 2.0, 3.0]);
        let w = Vector::new([1.0f64, 4.0, 6.0]);
        v.vslice_mut::<{ 1..3 }>().vmul_assign(2.0f64);
        assert_eq!(v, w);
    }
}

const fn const_len(r: core::ops::Range<usize>) -> usize {
    r.end - r.start
}

#[derive(Debug, PartialEq)]
struct Vector<const N: usize> {
    data: [f64; N],
}

struct VSlice<'a, const N: usize, const R: core::ops::Range<usize>> where
[(); const_len(R)]: Sized,
{

    vec: &'a Vector<N>,
    marker: core::marker::PhantomData<[(); const_len(R)]>,
}

struct VSliceMut<'a, const N: usize, const R: core::ops::Range<usize>> where
[(); const_len(R)]: Sized,
{

    vec: &'a mut Vector<N>,
    marker: core::marker::PhantomData<[(); const_len(R)]>,
}

impl<const N: usize> Vector<N> where
{
    fn new(data: [f64; N]) -> Self {
        Vector {
            data: data,
        }
    }
    fn vslice<'a, const R: core::ops::Range<usize>>(&'a self) -> VSlice<'a, N, R> where
    [(); const_len(R)]: Sized,
    {
        VSlice {
            vec: self,
            marker: core::marker::PhantomData,
        }
    }
    fn vslice_mut<'a, const R: core::ops::Range<usize>>(&'a mut self) -> VSliceMut<'a, N, R> where
    [(); const_len(R)]: Sized,
    {
        VSliceMut {
            vec: self,
            marker: core::marker::PhantomData,
        }
    }
}

impl<'a, const N: usize, const R: core::ops::Range<usize>> std::ops::Mul<f64> for VSlice<'a, N, R> where
[(); const_len(R)]: Sized,
{
    type Output = Vector<{ const_len(R) }>;
    fn mul(self, other: f64) -> Self::Output {
        let mut rv = [0f64; const_len(R)];
        for i in R {
            rv[i - R.start] = self.vec.data[i] * other;
        }
        Vector {
            data : rv,
        }
    }
}

impl<'a, const N: usize, const R: core::ops::Range<usize>> std::ops::Mul<VSlice<'a, N, R>> for f64 where
[(); const_len(R)]: Sized,
{
    type Output = Vector<{ const_len(R) }>;
    fn mul(self, other: VSlice<'a, N, R>) -> Self::Output {
        other.mul(self)
    }
}


impl<'a, const N: usize, const R: core::ops::Range<usize>> VSliceMut<'a, N, R>  where
[(); const_len(R)]: Sized,
{
    fn vmul_assign(self, other: f64) {
        for i in R {
            self.vec.data[i] *= other;
        }
    }
}
