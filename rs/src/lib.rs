use ndarray::{Array1, Array2, array, s};
use polygen::PolytopeBuilder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
}

/// Returns the oriented area formed by the outer product of 2 vectors
///
/// Bivector element order will be as follows:
/// - n=1; [e12]
/// - n=2; [e12, e13, e23]
/// - n=3; [e12, e13, e14, e23, e24, e34]
/// - etc.
fn oriented_area(u: Array1<f32>, v: Array1<f32>) -> Array1<f32> {
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
fn oriented_volume(u: Array1<f32>, v: Array1<f32>, w: Array1<f32>) -> Array1<f32> {
    let n = u.len();
    let te_n = (n * (n + 1) * (n + 2)) / 6; // nth tetrahedral number
    let mut ov = Array1::<f32>::zeros(te_n);
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

/// Used to perform plane-angle rotations in 4D space
struct Rotor4D {
    /// Angle of rotation
    theta: f32,
    /// The unit oriented area lying in the plane of rotation
    plane: Array1<f32>,
    /// The rotation matrix
    rotation_matrix: Array2<f32>,
    /// Signals that the rotation matrix needs to be recalculated
    rotation_matrix_dirty: bool,
}

impl Rotor4D {
    fn new() -> Self {
        Self {
            theta: 0f32,
            plane: Array1::<f32>::zeros(6),
            rotation_matrix: Array2::<f32>::eye(4),
            rotation_matrix_dirty: false,
        }
    }

    /// Sets the plane of rotation, given two 4-vectors spanning it
    fn set_plane(&mut self, v1: Array1<f32>, v2: Array1<f32>) {
        self.plane = normalize(oriented_area(v1, v2));
        self.rotation_matrix_dirty = true;
    }

    /// Sets the angle of rotation (in radians)
    fn set_angle(&mut self, theta: f32) {
        self.theta = theta;
        self.rotation_matrix_dirty = true;
    }

    /// Calculates the rotation matrix
    fn calculate_rotation_matrix(&mut self) {
        // I computed this by hand, using the formula for a rotor R. See theorem 3.7.1 from:
        // <https://scholarworks.sjsu.edu/cgi/viewcontent.cgi?article=7943&context=etd_theses>
        let c_theta = (self.theta / 2f32).cos();
        let s_theta = (self.theta / 2f32).sin();

        let c0 = c_theta;
        let c = &self.plane * s_theta;

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

        self.rotation_matrix = array![
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
    }

    /// Rotates set of vectors in place by the rotation matrix
    fn rotate(&mut self, vecs: &Array2<f32>) -> Array2<f32> {
        if self.rotation_matrix_dirty {
            self.calculate_rotation_matrix();
        }
        vecs.dot(&self.rotation_matrix)
    }
}

fn norm_l2(v: &Array1<f32>) -> f32 {
    v.map(|&v| v * v).sum()
}

fn normalize(mut v: Array1<f32>) -> Array1<f32> {
    let norm = norm_l2(&v);
    v /= norm;
    v
}

#[wasm_bindgen]
struct HyperObjectWasm {
    n_vert: usize,
    n_edge: usize,
    n_face: usize,

    vertices: Array2<f32>,
    edge_vert: Vec<Vec<usize>>,
    face_vert: Vec<Vec<usize>>,

    vertex_depth_attr: Vec<f32>,
    vertex_instance_matricies: Vec<f32>,

    edge_depth1_attr: Vec<f32>,
    edge_depth2_attr: Vec<f32>,
    edge_instance_matricies: Vec<f32>,

    rotor: Rotor4D,
}

#[wasm_bindgen]
impl HyperObjectWasm {
    #[wasm_bindgen(constructor)]
    pub fn new(diagram: &str) -> HyperObjectWasm {
        let polytope = PolytopeBuilder::from_diagram(diagram)
            .normalize(true)
            .build()
            .unwrap();

        let vertices = polytope.vertices.map(|&el| el as f32);
        let edge_vert = polytope.subelements(1, 0).unwrap();
        let face_vert = polytope.subelements(2, 0).unwrap();

        let n_vert = vertices.nrows();
        let n_edge = edge_vert.len();
        let n_face = face_vert.len();

        let vertex_depth_attr = vec![0f32; n_vert];
        let edge_depth1_attr = vec![0f32; n_edge];
        let edge_depth2_attr = vec![0f32; n_edge];

        let mut vertex_instance_matricies = vec![0f32; n_vert * 16];
        let mut edge_instance_matricies = vec![0f32; n_edge * 16];

        // set instance matricies to default (identity)
        for i in 0..n_vert {
            let o = i * 16;
            vertex_instance_matricies[o] = 1f32;
            vertex_instance_matricies[o + 5] = 1f32;
            vertex_instance_matricies[o + 10] = 1f32;
            vertex_instance_matricies[o + 15] = 1f32;
        }

        for i in 0..n_edge {
            let o = i * 16;
            edge_instance_matricies[o] = 1f32;
            edge_instance_matricies[o + 5] = 1f32;
            edge_instance_matricies[o + 10] = 1f32;
            edge_instance_matricies[o + 15] = 1f32;
        }

        HyperObjectWasm {
            n_vert,
            n_edge,
            n_face,
            vertices,
            edge_vert,
            face_vert,
            vertex_depth_attr,
            edge_depth1_attr,
            edge_depth2_attr,
            vertex_instance_matricies,
            edge_instance_matricies,
            rotor: Rotor4D::new(),
        }
    }

    pub fn update(&mut self) {
        ///// INSTANCE MATRICIES /////
        let vert3d = perspective_project(&self.vertices, -2f32);

        let earr = &mut self.edge_instance_matricies;
        for i in 0..self.n_edge {
            let e = &self.edge_vert[i];
            let v0 = vert3d.row(e[0]);
            let v1 = vert3d.row(e[1]);

            let mut fx = v0[0] - v1[0];
            let mut fy = v0[1] - v1[1];
            let mut fz = v0[2] - v1[2];
            let magf = (fx * fx + fy * fy + fz * fz).sqrt();

            fx /= magf;
            fy /= magf;
            fz /= magf;

            let mut ux = 1f32;
            let mut uy = 0f32;
            // uz is always 0
            let mut rx = 0f32;
            let mut ry = 1f32;
            let mut rz = 0f32;

            if (1f32 - fz).abs() < 1e-15f32 {
                ry = -1f32;
            } else if !((-1f32 - fz).abs() < 1e-15f32) {
                let d = (1f32 - fz * fz).sqrt();
                ux = -fy / d;
                uy = fx / d;
                rx = uy * fz;
                ry = -ux * fz;
                rz = ux * fy - uy * fx;
            }

            let o = i * 16;
            earr[o] = rx;
            earr[o + 1] = ry;
            earr[o + 2] = rz;

            earr[o + 4] = ux;
            earr[o + 5] = uy;
            // earr[o + 6] = uz; but uz is always 0

            earr[o + 8] = fx * magf;
            earr[o + 9] = fy * magf;
            earr[o + 10] = fz * magf;

            earr[o + 12] = v0[0];
            earr[o + 13] = v0[1];
            earr[o + 14] = v0[2];
        }

        let varr = &mut self.vertex_instance_matricies;
        for i in 0..self.n_vert {
            let v = vert3d.row(i);
            let o16 = i * 16;
            varr[o16 + 12] = v[0];
            varr[o16 + 13] = v[1];
            varr[o16 + 14] = v[2];
        }

        ///// DEPTHS /////
        // update vertex depths
        let max_w = 1f32;
        for i in 0..self.n_vert {
            self.vertex_depth_attr[i] =
                ((self.vertices[[i, 3]] / max_w + 1f32) / 2f32).clamp(0f32, 1f32);
        }

        // update edge depths
        for i in 0..self.n_edge {
            let e = &self.edge_vert[i];
            self.edge_depth1_attr[i] = self.vertex_depth_attr[e[0]];
            self.edge_depth2_attr[i] = self.vertex_depth_attr[e[1]];
        }
    }

    pub fn rotate(&mut self, theta: f32) {
        self.rotor
            .set_plane(array![1.0, 0.0, 0.0, 0.0], array![0.0, 0.0, 0.0, 1.0]);

        self.rotor.set_angle(theta);

        self.vertices = self.rotor.rotate(&mut self.vertices);
    }

    pub fn get_arr_pointers(&self) -> Vec<F32ArrSizedPointer> {
        vec![
            F32ArrSizedPointer::new(&self.vertex_depth_attr),
            F32ArrSizedPointer::new(&self.vertex_instance_matricies),
        ]
    }

    pub fn get_edge_mat_pointers(&self) -> Vec<F32ArrSizedPointer> {
        vec![
            F32ArrSizedPointer::new(&self.edge_depth1_attr),
            F32ArrSizedPointer::new(&self.edge_depth2_attr),
            F32ArrSizedPointer::new(&self.edge_instance_matricies),
        ]
    }
}

/// Perspective project a set of ND vectors to (N-1)D
fn perspective_project(vecs: &Array2<f32>, cam_dist: f32) -> Array2<f32> {
    let f = 2.0f32 / (&vecs.column(vecs.ncols() - 1) - cam_dist);
    let vecs_slice = vecs.slice(s![.., ..-1]);
    &vecs_slice * &f.broadcast((vecs_slice.ncols(), f.len())).unwrap().t()
}

#[wasm_bindgen]
pub struct HyperObjectPointers {}

#[wasm_bindgen]
pub struct F32ArrSizedPointer {
    pub pointer: *const f32,
    pub length: usize,
}

impl F32ArrSizedPointer {
    fn new(arr: &Vec<f32>) -> Self {
        Self {
            pointer: arr.as_ptr(),
            length: arr.len(),
        }
    }
}

// struct Vectors {
//     data: Vec<f32>,
//     itemSize: usize,
//     itemCount: usize,
// }

// impl Vectors {
//     pub fn new(data: Vec<f32>, itemSize: usize) -> Self {
//         Self {
//             itemSize,
//             itemCount: data.len() / itemSize,
//             data,
//         }
//     }

//     /// plane_offset is usually 2
//     pub fn perspective_project(&self, cam_dist: f32, plane_offset: f32) -> Vectors {
//         let mut proj_data = vec![0f32; self.itemCount * self.itemSize - 1];
//         for i in 0..self.itemCount {
//             let o = i * self.itemSize;
//             let o_proj: usize = i * (self.itemSize - 1);
//             let d: f32 = plane_offset / (self.data[o + self.itemSize - 1] - cam_dist);
//             proj_data[o_proj] = self.data[o] * d;
//             proj_data[o_proj + 1] = self.data[o + 1] * d;
//             proj_data[o_proj + 2] = self.data[o + 2] * d;
//         }

//         Vectors::new(proj_data, self.itemSize - 1)
//     }
// }
