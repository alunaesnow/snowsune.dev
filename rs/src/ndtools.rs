use std::ops::Mul;

use ndarray::{Array1, Array2, ArrayRef1, array, s};

use crate::utils::normalize;

/// Perspective project a set of ND vectors to (N-1)D
pub fn perspective_project(vecs: &Array2<f32>, cam_dist: f32) -> Array2<f32> {
    let f = 2.0f32 / (&vecs.column(vecs.ncols() - 1) - cam_dist);
    let vecs_slice = vecs.slice(s![.., ..-1]);
    &vecs_slice * &f.broadcast((vecs_slice.ncols(), f.len())).unwrap().t()
}

/// Returns the oriented area formed by the outer product of 2 vectors
///
/// Bivector element order will be as follows:
/// - n=1; [e12]
/// - n=2; [e12, e13, e23]
/// - n=3; [e12, e13, e14, e23, e24, e34]
/// - etc.
pub fn oriented_area(u: &ArrayRef1<f32>, v: &ArrayRef1<f32>) -> Array1<f32> {
    let n = u.len();
    let t_nm1 = ((n - 1) * n) / 2; // (n-1)th triangular number
    let mut oa = Array1::<f32>::zeros(t_nm1);
    let mut idx = 0;
    for i in 0..n {
        for j in i + 1..n {
            oa[idx] = u[i] * v[j] - u[j] * v[i];
            idx += 1;
        }
    }
    oa
}

/// Returns the oriented volume formed by the outer product of 3 vectors
///
/// Trivector element order will be as follows:
/// - n=2; [e123]
/// - n=3; [e123, e124, e134, e234]
/// - etc.
pub fn oriented_volume(u: &ArrayRef1<f32>, v: &ArrayRef1<f32>, w: Array1<f32>) -> Array1<f32> {
    let n = u.len();
    let te_nm1: usize = ((n - 1) * n * (n + 1)) / 6; // (n-1)th tetrahedral number
    let mut ov = Array1::<f32>::zeros(te_nm1);
    let mut idx = 0;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                ov[idx] = u[i] * v[j] * w[k] + u[j] * v[k] * w[i] + u[k] * v[i] * w[j]
                    - u[i] * v[k] * w[j]
                    - u[j] * v[i] * w[k]
                    - u[k] * v[j] * w[i];
                idx += 1;
            }
        }
    }
    ov
}

pub struct Vec4 {
    e1: f32,
    e2: f32,
    e3: f32,
    e4: f32,
}

pub struct Scalar {
    value: f32,
}

pub struct BiVec4 {
    e12: f32,
    e13: f32,
    e14: f32,
    e23: f32,
    e24: f32,
    e34: f32,
}

pub struct QuadVec4 {
    e1234: f32,
}

/// Rotor in 4D space. this represents plane-angle rotation in 4D space,
/// like the 4D version of a quaternion.
pub struct Rotor4 {
    scalar: Scalar,
    bivec: BiVec4,
    quadvec: QuadVec4,
}

// impl Rotor4 {
//     pub fn from_rotation(v1:)
// }

impl Mul<&Rotor4> for &Rotor4 {
    type Output = Rotor4;

    fn mul(self, rhs: &Rotor4) -> Self::Output {
        // I want to explode
        let scalar = self.scalar.value * rhs.scalar.value - self.bivec.e12 * rhs.bivec.e12
            + self.quadvec.e1234 * rhs.quadvec.e1234
            - self.bivec.e13 * rhs.bivec.e13
            - self.bivec.e14 * rhs.bivec.e14
            - self.bivec.e23 * rhs.bivec.e23
            - self.bivec.e24 * rhs.bivec.e24
            - self.bivec.e34 * rhs.bivec.e34;

        let e12 = self.bivec.e12 * rhs.scalar.value + self.scalar.value * rhs.bivec.e12
            - self.bivec.e34 * rhs.quadvec.e1234
            + self.bivec.e23 * rhs.bivec.e13
            + self.bivec.e24 * rhs.bivec.e14
            - self.bivec.e13 * rhs.bivec.e23
            - self.bivec.e14 * rhs.bivec.e24
            - self.quadvec.e1234 * rhs.bivec.e34;

        let e13 = self.bivec.e13 * rhs.scalar.value - self.bivec.e23 * rhs.bivec.e12
            + self.bivec.e24 * rhs.quadvec.e1234
            + self.scalar.value * rhs.bivec.e13
            + self.bivec.e34 * rhs.bivec.e14
            + self.bivec.e12 * rhs.bivec.e23
            + self.quadvec.e1234 * rhs.bivec.e24
            - self.bivec.e14 * rhs.bivec.e34;

        let e14 = self.bivec.e14 * rhs.scalar.value
            - self.bivec.e24 * rhs.bivec.e12
            - self.bivec.e23 * rhs.quadvec.e1234
            - self.bivec.e34 * rhs.bivec.e13
            + self.scalar.value * rhs.bivec.e14
            - self.quadvec.e1234 * rhs.bivec.e23
            + self.bivec.e12 * rhs.bivec.e24
            + self.bivec.e13 * rhs.bivec.e34;

        let e23 = self.bivec.e23 * rhs.scalar.value + self.bivec.e13 * rhs.bivec.e12
            - self.bivec.e14 * rhs.quadvec.e1234
            - self.bivec.e12 * rhs.bivec.e13
            - self.quadvec.e1234 * rhs.bivec.e14
            + self.scalar.value * rhs.bivec.e23
            + self.bivec.e34 * rhs.bivec.e24
            - self.bivec.e24 * rhs.bivec.e34;

        let e24 = self.bivec.e24 * rhs.scalar.value
            + self.bivec.e14 * rhs.bivec.e12
            + self.bivec.e13 * rhs.quadvec.e1234
            + self.quadvec.e1234 * rhs.bivec.e13
            - self.bivec.e12 * rhs.bivec.e14
            - self.bivec.e34 * rhs.bivec.e23
            + self.scalar.value * rhs.bivec.e24
            + self.bivec.e23 * rhs.bivec.e34;

        let e34 = self.bivec.e34 * rhs.scalar.value
            - self.quadvec.e1234 * rhs.bivec.e12
            - self.bivec.e12 * rhs.quadvec.e1234
            + self.bivec.e14 * rhs.bivec.e13
            - self.bivec.e13 * rhs.bivec.e14
            + self.bivec.e24 * rhs.bivec.e23
            - self.bivec.e23 * rhs.bivec.e24
            + self.scalar.value * rhs.bivec.e34;

        let e1234 = self.quadvec.e1234 * rhs.scalar.value
            + self.bivec.e34 * rhs.bivec.e12
            + self.scalar.value * rhs.quadvec.e1234
            - self.bivec.e24 * rhs.bivec.e13
            + self.bivec.e23 * rhs.bivec.e14
            + self.bivec.e14 * rhs.bivec.e23
            - self.bivec.e13 * rhs.bivec.e24
            + self.bivec.e12 * rhs.bivec.e34;

        Rotor4 {
            scalar: Scalar { value: scalar },
            bivec: BiVec4 {
                e12,
                e13,
                e14,
                e23,
                e24,
                e34,
            },
            quadvec: QuadVec4 { e1234 },
        }
    }
}

impl Rotor4 {
    // m11 = A^2 v1-a12^2 v1-a1234^2 v1-a13^2 v1-a14^2 v1+a23^2 v1+a24^2 v1+a34^2 v1
    // m12 = 2 A a12 v2-2 a13 a23 v2-2 a14 a24 v2+2 a1234 a34 v2
    // m13 = 2 A a13 v3+2 a12 a23 v3-2 a1234 a24 v3-2 a14 a34 v3
    // m14 = 2 A a14 v4+2 a1234 a23 v4+2 a12 a24 v4+2 a13 a34 v4) e[1]
    //
    // m21 = -2 A a12 v1-2 a13 a23 v1-2 a14 a24 v1-2 a1234 a34 v1
    // m22 = A^2 v2-a12^2 v2-a1234^2 v2+a13^2 v2+a14^2 v2-a23^2 v2-a24^2 v2+a34^2 v2
    // m23 = 2 a12 a13 v3+2 a1234 a14 v3+2 A a23 v3-2 a24 a34 v3
    // m24 = 2 a1234 a13 v4-2 a12 a14 v4+2 A a24 v4+2 a23 a34 v4) e[2]
    //
    // m31 = -2 A a13 v1+2 a12 a23 v1+2 a1234 a24 v1-2 a14 a34 v1
    // m32 = 2 a12 a13 v2-2 a1234 a14 v2-2 A a23 v2-2 a24 a34 v2
    // m33 = A^2 v3+a12^2 v3-a1234^2 v3-a13^2 v3+a14^2 v3-a23^2 v3+a24^2 v3-a34^2 v3
    // m34 = 2 a12 a1234 v4-2 a13 a14 v4-2 a23 a24 v4+2 A a34 v4) e[3]
    //
    // m41 = -2 A a14 v1-2 a1234 a23 v1+2 a12 a24 v1+2 a13 a34 v1
    // m42 = 2 a1234 a13 v2-2 a12 a14 v2-2 A a24 v2+2 a23 a34 v2
    // m43 = 2 a12 a1234 v3-2 a13 a14 v3-2 a23 a24 v3-2 A a34 v3
    // m44 = A^2 v4+a12^2 v4-a1234^2 v4+a13^2 v4-a14^2 v4+a23^2 v4-a24^2 v4-a34^2 v4) e[4]

    // various factors;
    // let f1 = + a12 a23 - a14 a34;
    // let f2 = + a12 a24 + a13 a34;

    // m11 = ssq -e12sq -e1234sq -e13sq -e14sq +e23sq +e24sq +e34sq
    // m12 = 2*( A a12 - a13 a23 - a14 a24 + a1234 a34)
    // m13 = 2*( A a13 +f1 - a1234 a24)
    // m14 = 2*( A a14 +f2 + a1234 a23)
    //
    // m21 = 2*(- A a12 - a13 a23 - a14 a24 - a1234 a34)
    // m22 = ssq -e12sq -e1234sq +e13sq +e14sq -e23sq -e24sq +e34sq
    // m23 = 2*( a12 a13 + A a23 - a24 a34 + a1234 a14)
    // m24 = 2*(- a12 a14 + A a24 + a23 a34 + a1234 a13)
    //
    // m31 = 2*(- A a13 +f1 + a1234 a24)
    // m32 = 2*( a12 a13 - A a23 - a24 a34 - a1234 a14)
    // m33 = ssq +e12sq -e1234sq -e13sq +e14sq -e23sq +e24sq -e34sq
    // m34 = 2*(- a13 a14 - a23 a24 + A a34 + a12 a1234)
    //
    // m41 = 2*(- A a14 +f2 - a1234 a23)
    // m42 = 2*(- a12 a14 - A a24 + a23 a34 + a1234 a13)
    // m43 = 2*(- a13 a14 - a23 a24 - A a34 + a12 a1234)
    // m44 = ssq +e12sq -e1234sq +e13sq -e14sq +e23sq -e24sq -e34sq

    // m11 = g1 + g3;
    // m12 = 2.0*(f3 + f4)
    // m13 = 2.0*(f1 + f5)
    // m14 = 2.0*(f2 + f6)
    //
    // m21 = 2.0*(f3 - f4)
    // m22 = g1 - g3;
    // m23 = 2.0*(f7 + f8)
    // m24 = 2.0*( f10 + f12)
    //
    // m31 = 2.0*(f1 - f5)
    // m32 = 2.0*(f7 - f8)
    // m33 = g2 + g4
    // m34 = 2.0*(f9 + f11 )
    //
    // m41 = 2.0*(f2 - f6)
    // m42 = 2.0*(f10 - f12)
    // m43 = 2.0*(f9 - f11 )
    // m44 = g2 - g4

    pub fn from_plane_angle(v1: &ArrayRef1<f32>, v2: &ArrayRef1<f32>, theta: f32) -> Self {
        let c_theta = (theta / 2f32).cos();
        let s_theta = (theta / 2f32).sin();

        let bivec = normalize(oriented_area(v1, v2)) * s_theta;

        Self {
            scalar: Scalar { value: c_theta },
            bivec: BiVec4 {
                e12: bivec[0],
                e13: bivec[1],
                e14: bivec[2],
                e23: bivec[3],
                e24: bivec[4],
                e34: bivec[5],
            },
            quadvec: QuadVec4 { e1234: 0.0 },
        }
    }

    pub fn to_rotation_matrix(&self) -> Array2<f32> {
        let ssq = self.scalar.value.powi(2);
        let e12sq = self.bivec.e12.powi(2);
        let e13sq = self.bivec.e13.powi(2);
        let e14sq = self.bivec.e14.powi(2);
        let e23sq = self.bivec.e23.powi(2);
        let e24sq = self.bivec.e24.powi(2);
        let e34sq = self.bivec.e34.powi(2);
        let e1234sq = self.quadvec.e1234.powi(2);

        // various factors;
        let f1 = self.bivec.e12 * self.bivec.e23 - self.bivec.e14 * self.bivec.e34;
        let f2 = self.bivec.e12 * self.bivec.e24 + self.bivec.e13 * self.bivec.e34;
        let f3 = -self.bivec.e13 * self.bivec.e23 - self.bivec.e14 * self.bivec.e24;
        let f4 = self.scalar.value * self.bivec.e12 + self.quadvec.e1234 * self.bivec.e34;
        let f5 = self.scalar.value * self.bivec.e13 - self.quadvec.e1234 * self.bivec.e24;
        let f6 = self.scalar.value * self.bivec.e14 + self.quadvec.e1234 * self.bivec.e23;
        let f7 = self.bivec.e12 * self.bivec.e13 - self.bivec.e24 * self.bivec.e34;
        let f8 = self.scalar.value * self.bivec.e23 + self.quadvec.e1234 * self.bivec.e14;
        let f9 = -self.bivec.e13 * self.bivec.e14 - self.bivec.e23 * self.bivec.e24
            + self.bivec.e12 * self.quadvec.e1234;
        let f10 = self.bivec.e23 * self.bivec.e34 + self.quadvec.e1234 * self.bivec.e13
            - self.bivec.e12 * self.bivec.e14;

        let f11 = self.scalar.value * self.bivec.e34;
        let f12 = self.scalar.value * self.bivec.e24;

        let g1 = ssq - e12sq - e1234sq + e34sq;
        let g2 = ssq + e12sq - e1234sq - e34sq;
        let g3 = -e13sq - e14sq + e23sq + e24sq;
        let g4 = -e13sq + e14sq - e23sq + e24sq;

        array![
            [g1 + g3, 2.0 * (f3 + f4), 2.0 * (f1 + f5), 2.0 * (f2 + f6),],
            [2.0 * (f3 - f4), g1 - g3, 2.0 * (f7 + f8), 2.0 * (f10 + f12),],
            [2.0 * (f1 - f5), 2.0 * (f7 - f8), g2 + g4, 2.0 * (f9 + f11),],
            [
                2.0 * (f2 - f6),
                2.0 * (f10 - f12),
                2.0 * (f9 - f11),
                g2 - g4,
            ],
        ]
    }
}
