//##########################//
//###### DECLARATION #######//
//##########################//

pub type MatNM<const N: usize, const M: usize> = [[f32; M]; N];
pub type Mat2x2 = MatNM<2, 2>;
pub type Mat3x3 = MatNM<3, 3>;
pub type Mat4x4 = MatNM<4, 4>;

//##########################//
//######### TRAITS #########//
//##########################//

pub trait MatDefault {
    fn zero() -> Self;
    fn fill(val: f32) -> Self;
}
pub trait MatSqrDefault {
    fn ident() -> Self;
}

pub trait MatTranspose<O> {
    fn trans(self) -> O;
}

pub trait MatAdd {
    fn add(self, other: Self) -> Self;
}

pub trait MatSub {
    fn sub(self, other: Self) -> Self;
}

pub trait MatMult<R, O> {
    fn mult(self, other: R) -> O;
}

//##########################//
//##### IMPLEMENTATION #####//
//##########################//

impl<const N: usize, const M: usize> MatDefault for MatNM<N, M> {
    fn zero() -> Self {
        [[0.0; M]; N]
    }

    fn fill(val: f32) -> Self {
        [[val; M]; N]
    }
}

impl<const N: usize> MatSqrDefault for MatNM<N, N> {
    fn ident() -> Self {
        let mut out = MatNM::zero();
        for i in 0..N {
            out[i][i] = 1.0;
        }
        out
    }
}

impl<const N: usize, const M: usize> MatTranspose<MatNM<M, N>> for MatNM<N, M> {
    fn trans(self) -> MatNM<M, N> {
        let mut out = MatNM::zero();
        for i in 0..N {
            for j in 0..M {
                out[j][i] = self[i][j];
            }
        }
        out
    }
}

impl<const N: usize, const M: usize> MatAdd for MatNM<N, M> {
    fn add(self, other: Self) -> Self {
        let mut out = MatNM::zero();
        for i in 0..N {
            for j in 0..M {
                out[i][j] = self[i][j] + other[i][j];
            }
        }
        out
    }
}

impl<const N: usize, const M: usize> MatSub for MatNM<N, M> {
    fn sub(self, other: Self) -> Self {
        let mut out = MatNM::zero();
        for i in 0..N {
            for j in 0..M {
                out[i][j] = self[i][j] - other[i][j];
            }
        }
        out
    }
}

impl<const N: usize, const M: usize> MatMult<f32, Self> for MatNM<N, M> {
    fn mult(mut self, k: f32) -> Self {
        self.iter_mut()
            .for_each(|row| row.into_iter().for_each(|el| *el *= k));
        self
    }
}

impl<const N: usize, const X: usize, const M: usize> MatMult<MatNM<X, M>, MatNM<N, M>>
    for MatNM<N, X>
{
    fn mult(self, other: MatNM<X, M>) -> MatNM<N, M> {
        let mut out = MatNM::zero();
        for i in 0..N {
            for j in 0..M {
                out[i][j] = (0..X).map(|k| self[i][k] * other[k][j]).sum();
            }
        }
        out
    }
}

//##########################//
//######### TESTS ##########//
//##########################//

mod matrix_tests {
    use crate::math::mat::*;
    #[test]
    fn test_matrix_mult() {
        let m1: MatNM<2, 4> = [[10.0, 0.0, 10.0, 5.0], [10.0, 0.0, 10.0, 5.0]];

        let m2: MatNM<4, 3> = [
            [0.0, 1.0, 4.0],
            [2.0, -7.0, 0.0],
            [6.0, 1.0, 4.0],
            [0.0, 1.0, 4.0],
        ];

        assert_eq!([[60.0, 25.0, 100.0], [60.0, 25.0, 100.0],], m1.mult(m2));
    }

    #[test]
    fn test_num_mult() {
        let m = Mat4x4::ident().mult(5.0);
        assert_eq!(
            [
                [5.0, 0.0, 0.0, 0.0],
                [0.0, 5.0, 0.0, 0.0],
                [0.0, 0.0, 5.0, 0.0],
                [0.0, 0.0, 0.0, 5.0],
            ],
            m
        )
    }

    #[test]
    fn test_trans() {
        let m = [
            [0.0, 0.0, 1.0],
            [0.0, 2.0, 0.0],
            [3.0, 0.0, 0.0],
            [0.0, 4.0, 0.0],
        ]
        .trans();
        assert_eq!(
            [
                [0.0, 0.0, 3.0, 0.0],
                [0.0, 2.0, 0.0, 4.0],
                [1.0, 0.0, 0.0, 0.0],
            ],
            m
        );
    }

    #[test]
    fn test_add_sub() {
        let m1 = Mat4x4::fill(9.0);
        let m2 = Mat4x4::ident().mult(2.0);

        assert_eq!(
            [
                [11.0, 9.0, 9.0, 9.0],
                [9.0, 11.0, 9.0, 9.0],
                [9.0, 9.0, 11.0, 9.0],
                [9.0, 9.0, 9.0, 11.0],
            ],
            m1.add(m2)
        );

        assert_eq!(
            [
                [7.0, 9.0, 9.0, 9.0],
                [9.0, 7.0, 9.0, 9.0],
                [9.0, 9.0, 7.0, 9.0],
                [9.0, 9.0, 9.0, 7.0],
            ],
            m1.sub(m2)
        );
    }
}
