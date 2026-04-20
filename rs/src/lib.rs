use ndarray::{Array2, s};
use polygen::PolytopeBuilder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
}

// struct Rotor4D {

// }

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

fn perspective_project(points: &Array2<f32>, cam_dist: f32) -> Array2<f32> {
    let f = 2.0f32 / (&points.column(points.ncols() - 1) - cam_dist);
    let points_slice = points.slice(s![.., ..-1]);
    &points_slice * &f.broadcast((points_slice.ncols(), f.len())).unwrap().t()
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
