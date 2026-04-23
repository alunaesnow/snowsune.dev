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

/// Rotor in 4D space. this represents plane-angle rotation in 4D space,
/// like the 4D version of a quaternion.
pub struct Rotor4 {
    /// Scalar part
    scalar: f32,
    /// Bivector part [e12, e13, e14, e23, e24, e34]
    bivector: Array1<f32>,
}

// impl Mul<Rotor4> for Rotor4 {

// }

/// Used to perform plane-angle rotations in 4D space
pub struct Rotor4D {
    /// The rotation matrix
    rotation_matrix: Array2<f32>,
}

impl Rotor4D {
    /// Creates a rotor that rotates by angle `theta` in the plane spanned by vectors
    /// `v1` and `v2`
    pub fn new(v1: &ArrayRef1<f32>, v2: &ArrayRef1<f32>, theta: f32) -> Self {
        let plane = normalize(oriented_area(v1, v2));

        // I computed this by hand, using the formula for a rotor R. See theorem 3.7.1 from:
        // <https://scholarworks.sjsu.edu/cgi/viewcontent.cgi?article=7943&context=etd_theses>
        let c_theta = (theta / 2f32).cos();
        let s_theta = (theta / 2f32).sin();

        let c0 = c_theta;
        let c = plane * s_theta;

        let c11 = c0 * c0;
        let c22 = c[0] * c[0];
        let c33 = c[1] * c[1];
        let c44 = c[2] * c[2];
        let c55 = c[3] * c[3];
        let c66 = c[4] * c[4];
        let c77 = c[5] * c[5];

        let c12 = c0 * c[0];
        let c13 = c0 * c[1];
        let c14 = c0 * c[2];
        let c15 = c0 * c[3];
        let c16 = c0 * c[4];
        let c17 = c0 * c[5];

        let c23 = c[0] * c[1];
        let c24 = c[0] * c[2];
        let c25 = c[0] * c[3];
        let c26 = c[0] * c[4];
        // let c27 = c[0] * c[5];

        let c34 = c[1] * c[2];
        let c35 = c[1] * c[3];
        // let c36 = c[1] * c[4];
        let c37 = c[1] * c[5];

        // let c45 = c[2] * c[3];
        let c46 = c[2] * c[4];
        let c47 = c[2] * c[5];

        let c56 = c[3] * c[4];
        let c57 = c[3] * c[5];

        let c67 = c[4] * c[5];

        let rotation_matrix = array![
            [
                c11 - c22 - c33 - c44 + c55 + c66 + c77,
                -2f32 * (c12 + c46 + c35),
                2f32 * (-c13 - c47 + c25),
                2f32 * (-c14 + c26 + c37),
            ],
            [
                2f32 * (c12 - c35 - c46),
                -c22 + c11 + c33 + c44 - c55 - c66 + c77,
                -2f32 * (c23 + c15 + c67),
                -2f32 * (c24 + c16 - c57),
            ],
            [
                2f32 * (c13 + c25 - c47),
                -2f32 * (c23 - c15 + c67),
                -c33 + c11 + c22 + c44 - c55 + c66 - c77,
                -2f32 * (c34 + c56 + c17),
            ],
            [
                2f32 * (c14 + c26 + c37),
                -2f32 * (c24 - c16 - c57),
                -2f32 * (c34 + c56 - c17),
                -c44 + c11 + c22 + c33 - c66 + c55 - c77,
            ],
        ];

        Self { rotation_matrix }
    }

    /// Rotates set of vectors in place by the rotation matrix
    pub fn rotate(&self, vecs: &Array2<f32>) -> Array2<f32> {
        vecs.dot(&self.rotation_matrix)
    }
}
