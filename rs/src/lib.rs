use ndarray::{Array1, Array2, array};
use polygen::PolytopeBuilder;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use crate::{
    ndtools::{Rotor4D, perspective_project},
    utils::norm_l2,
};

mod ndtools;
mod utils;

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
}

#[derive(Tsify, Deserialize)]
#[tsify(from_wasm_abi)]
enum RotationFrame {
    WorldAxes,
    ObjectAxes,
}

struct PolytopeData {}

struct RenderData {
    vertex_pos: Vec<f32>,
    edge_pos_from: Vec<f32>,
    edge_pos_to: Vec<f32>,
}

#[wasm_bindgen]
struct PolytopeWasm {
    rdat: RenderData,

    vertices: Array2<f32>,
    edge_vert: Vec<Vec<usize>>,
    face_vert: Vec<Vec<usize>>,

    axes: Array2<f32>,
    world_axes: Array2<f32>,
    rotation_frame: RotationFrame,

    needsUpdate: bool,
}

enum Indices {
    UInt16(Vec<u16>),
    UInt32(Vec<u32>),
}

#[wasm_bindgen]
impl PolytopeWasm {
    #[wasm_bindgen(constructor)]
    pub fn new(diagram: &str) -> PolytopeWasm {
        let polytope = PolytopeBuilder::from_diagram(diagram)
            .normalize(true)
            .build()
            .unwrap();

        let vertices = polytope.vertices.map(|&el| el as f32);
        let edge_vert = polytope.subelements(1, 0).unwrap();
        let face_vert = polytope.subelements(2, 0).unwrap();

        // let iter = face_vert.iter().flatten();
        // let a: Indices = if vertices.len() <= u16::MAX as usize {
        //     Indices::UInt16(iter.map(|&v| u16::try_from(v).unwrap()).collect())
        // } else {
        //     Indices::UInt32(iter.map(|&v| u32::try_from(v).unwrap()).collect())
        // };

        let n_vert = vertices.nrows();
        let n_edge = edge_vert.len();

        let mut vertex_pos = vec![0f32; n_vert * 4];
        let mut edge_pos_from = vec![0f32; n_edge * 4];
        let mut edge_pos_to = vec![0f32; n_edge * 4];

        for i in 0..n_edge {
            let o4 = i * 4;
            let e = &edge_vert[i];
            let v0 = vertices.row(e[0]);
            let v1 = vertices.row(e[1]);

            edge_pos_from[o4] = v0[0];
            edge_pos_from[o4 + 1] = v0[1];
            edge_pos_from[o4 + 2] = v0[2];
            edge_pos_from[o4 + 3] = v0[3];
            edge_pos_to[o4] = v1[0];
            edge_pos_to[o4 + 1] = v1[1];
            edge_pos_to[o4 + 2] = v1[2];
            edge_pos_to[o4 + 3] = v1[3];
        }

        for i in 0..n_vert {
            let v = vertices.row(i);
            let o4 = i * 4;
            vertex_pos[o4] = v[0];
            vertex_pos[o4 + 1] = v[1];
            vertex_pos[o4 + 2] = v[2];
            vertex_pos[o4 + 3] = v[3];
        }

        PolytopeWasm {
            rdat: RenderData {
                vertex_pos,
                edge_pos_from,
                edge_pos_to,
            },
            vertices,
            edge_vert,
            face_vert,
            axes: Array2::<f32>::eye(4),
            world_axes: Array2::<f32>::eye(4),
            rotation_frame: RotationFrame::WorldAxes,
            needsUpdate: true,
        }
    }

    pub fn update(&mut self) -> bool {
        if !self.needsUpdate {
            return false;
        }

        return true;
    }

    pub fn rotate(&mut self, theta: f32, axes1: usize, axes2: usize) {
        if theta < 1e-15 {
            return;
        }

        let axes = match self.rotation_frame {
            RotationFrame::WorldAxes => &self.world_axes,
            RotationFrame::ObjectAxes => &self.axes,
        };

        let rotor = Rotor4D::new(&axes.row(axes1), &axes.row(axes2), theta);

        self.vertices = rotor.rotate(&mut self.vertices);
        self.axes = rotor.rotate(&mut self.axes);

        self.needsUpdate = true;
    }

    pub fn set_rotation_frame(&mut self, rotation_frame: RotationFrame) {
        self.rotation_frame = rotation_frame;
    }

    pub fn get_render_data_refs(&self) -> RenderDataRefs {
        RenderDataRefs {
            vertex_pos: self.rdat.vertex_pos.as_array_ref(),
            edge_pos_from: self.rdat.edge_pos_from.as_array_ref(),
            edge_pos_to: self.rdat.edge_pos_to.as_array_ref(),
        }
    }

    pub fn get_ideal_thickness(&self) -> f32 {
        let mut total_face_area = 0.0;
        for face in &self.face_vert {
            let v0 = self.vertices.row(face[0]);
            for i in 1..face.len() - 1 {
                let v1 = self.vertices.row(face[i]);
                let v2 = self.vertices.row(face[i + 1]);
                let a = &v1 - &v0;
                let b = &v2 - &v0;
                let mag_a = norm_l2(&a);
                let mag_b = norm_l2(&b);
                let angle = (a.dot(&b) / (mag_a * mag_b)).acos();
                total_face_area += mag_a * mag_b * angle;
            }
        }

        (total_face_area / self.face_vert.len() as f32).sqrt() * 0.025
    }
}

// pub struct BufferAttribute<T> {
//     data: Vec<T>,
// }

#[derive(Tsify, Serialize)]
#[tsify(into_wasm_abi)]
pub struct RenderDataRefs {
    vertex_pos: ArrayRef,
    edge_pos_from: ArrayRef,
    edge_pos_to: ArrayRef,
}

#[derive(Tsify, Serialize)]
#[tsify(into_wasm_abi)]
pub enum ArrayItemType {
    Float32,
    UInt16,
    UInt32,
}

#[derive(Tsify, Serialize)]
#[tsify(into_wasm_abi)]
pub struct ArrayRef {
    pub pointer: usize,
    pub length: usize,
    pub item_type: ArrayItemType,
}

trait AsArrayRef {
    fn as_array_ref(&self) -> ArrayRef;
}

impl AsArrayRef for Vec<f32> {
    fn as_array_ref(&self) -> ArrayRef {
        ArrayRef {
            pointer: self.as_ptr() as usize,
            length: self.len(),
            item_type: ArrayItemType::Float32,
        }
    }
}
