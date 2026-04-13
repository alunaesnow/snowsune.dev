use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn fib(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

#[wasm_bindgen]
pub fn vectest(mul: u8) -> *const u8 {
    let mut a = vec![0u8; 10];
    for i in 0..10u8 {
        a[i as usize] = i * mul;
    }
    return a.as_ptr();
}

#[wasm_bindgen]
struct HyperObjectWasm {
    n_vert: usize,
    n_edge: usize,

    vertices: Vec<f32>,
    edges: Vec<usize>,

    vertex_depth_attr: Vec<f32>,
    vertex_instance_matricies: Vec<f32>,

    edge_depth1_attr: Vec<f32>,
    edge_depth2_attr: Vec<f32>,
    edge_instance_matricies: Vec<f32>,
}

#[wasm_bindgen]
impl HyperObjectWasm {
    #[wasm_bindgen(constructor)]
    pub fn new(vertices: Vec<f32>, edges: Vec<usize>) -> HyperObjectWasm {
        let n_vert = vertices.len() / 4;
        let n_edge = edges.len() / 2;

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
            vertices,
            edges,
            vertex_depth_attr,
            edge_depth1_attr,
            edge_depth2_attr,
            vertex_instance_matricies,
            edge_instance_matricies,
        }
    }

    pub fn update(&mut self) {
        ///// INSTANCE MATRICIES /////
        let vert3d = perspective_project4(&self.vertices, -2f32, 2f32);

        let earr = &mut self.edge_instance_matricies;
        for i in 0..self.n_edge {
            let o0 = self.edges[i * 2] * 3;
            let o1 = self.edges[i * 2 + 1] * 3;

            let mut fx = vert3d[o0] - vert3d[o1];
            let mut fy = vert3d[o0 + 1] - vert3d[o1 + 1];
            let mut fz = vert3d[o0 + 2] - vert3d[o1 + 2];
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

            earr[o + 12] = vert3d[o0];
            earr[o + 13] = vert3d[o0 + 1];
            earr[o + 14] = vert3d[o0 + 2];
        }

        let varr = &mut self.vertex_instance_matricies;
        for i in 0..self.n_vert {
            let o3 = i * 3;
            let o16 = i * 16;
            varr[o16 + 12] = vert3d[o3];
            varr[o16 + 13] = vert3d[o3 + 1];
            varr[o16 + 14] = vert3d[o3 + 2];
        }

        ///// DEPTHS /////
        // update vertex depths
        let max_w = 1f32;
        for i in 0..self.n_vert {
            self.vertex_depth_attr[i] =
                ((self.vertices[i * 4 + 3] / max_w + 1f32) / 2f32).clamp(0f32, 1f32);
        }

        // update edge depths
        for i in 0..self.n_edge {
            let o = i * 2;
            self.edge_depth1_attr[i] = self.vertex_depth_attr[self.edges[o]];
            self.edge_depth2_attr[i] = self.vertex_depth_attr[self.edges[o + 1]];
        }
    }

    pub fn get_arr_pointers(&self) -> Vec<F32ArrSizedPointer> {
        vec![
            pointerify_f32_arr(&self.vertex_depth_attr),
            pointerify_f32_arr(&self.vertex_instance_matricies),
        ]
    }

    pub fn get_edge_mat_pointers(&self) -> Vec<F32ArrSizedPointer> {
        vec![
            pointerify_f32_arr(&self.edge_depth1_attr),
            pointerify_f32_arr(&self.edge_depth2_attr),
            pointerify_f32_arr(&self.edge_instance_matricies),
        ]
    }
}

fn perspective_project4(points4d: &Vec<f32>, cam_dist: f32, plane_offset: f32) -> Vec<f32> {
    let n_points = points4d.len() / 4;
    let mut points3d = vec![0f32; n_points * 3];
    for i in 0..n_points {
        let o4d = i * 4;
        let o3d: usize = i * 3;
        let d: f32 = plane_offset / (points4d[o4d + 3] - cam_dist);
        points3d[o3d] = points4d[o4d] * d;
        points3d[o3d + 1] = points4d[o4d + 1] * d;
        points3d[o3d + 2] = points4d[o4d + 2] * d;
    }

    points3d
}

#[wasm_bindgen]
pub struct F32ArrSizedPointer {
    pub pointer: *const f32,
    pub length: usize,
}

fn pointerify_f32_arr(arr: &Vec<f32>) -> F32ArrSizedPointer {
    F32ArrSizedPointer {
        pointer: arr.as_ptr(),
        length: arr.len(),
    }
}
