use ndarray::Array2;
use polygen::PolytopeBuilder;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use crate::{ndtools::Rotor4, utils::norm_l2};

mod ndtools;
mod utils;

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
}

#[derive(Tsify, Deserialize, PartialEq)]
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
    rotation_matrix: Vec<f32>,
}

#[wasm_bindgen]
struct PolytopeWasm {
    rdat: RenderData,

    vertices: Array2<f32>,
    edge_vert: Vec<Vec<usize>>,
    face_vert: Vec<Vec<usize>>,

    world_axes: Array2<f32>,
    rotation_frame: RotationFrame,
    /// Like a quaternion, this holds the rotation of the 4D object
    rotor: Option<Rotor4>,

    needs_pdate: bool,
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

        let mut rotation_matrix = vec![0f32; 16];
        for i in 0..4 {
            rotation_matrix[i + i * 4] = 1.0;
        }

        PolytopeWasm {
            rdat: RenderData {
                vertex_pos,
                edge_pos_from,
                edge_pos_to,
                rotation_matrix,
            },
            vertices,
            edge_vert,
            face_vert,
            world_axes: Array2::<f32>::eye(4),
            rotation_frame: RotationFrame::WorldAxes,
            rotor: None,
            needs_pdate: true,
        }
    }

    pub fn update(&mut self) -> bool {
        if !self.needs_pdate {
            return false;
        }

        if let Some(rotor) = &self.rotor {
            let rot_mat = rotor.to_rotation_matrix();
            for i in 0..4 {
                for j in 0..4 {
                    self.rdat.rotation_matrix[i * 4 + j] = rot_mat[[i, j]];
                }
            }
        }
        // for i in 0..4 {
        //     for j in 0..4 {
        //         self.rdat.rotation_matrix[i * 4 + j] = self.rot_mat[[i, j]];
        //     }
        // }

        return true;
    }

    pub fn rotate(&mut self, theta: f32, axes1: usize, axes2: usize) {
        if theta < 1e-15 {
            return;
        }

        let mut v1 = self.world_axes.row(axes1).to_owned();
        let mut v2 = self.world_axes.row(axes2).to_owned();

        if self.rotation_frame == RotationFrame::ObjectAxes {
            if let Some(rotor) = &self.rotor {
                let rot_mat = rotor.to_rotation_matrix();
                v1 = v1.dot(&rot_mat);
                v2 = v2.dot(&rot_mat);
            }
        }

        let new_rotor = Rotor4::from_plane_angle(&v1, &v2, theta);

        match &self.rotor {
            Some(rotor) => self.rotor = Some(&new_rotor * rotor),
            None => self.rotor = Some(new_rotor),
        }

        // let rotor = Rotor4D::new(&v1, &v2, theta);

        // self.rot_mat = rotor.rotation_matrix.dot(&self.rot_mat);

        self.needs_pdate = true;
    }

    pub fn set_rotation_frame(&mut self, rotation_frame: RotationFrame) {
        self.rotation_frame = rotation_frame;
    }

    pub fn get_render_data_refs(&self) -> RenderDataRefs {
        RenderDataRefs {
            vertex_pos: self.rdat.vertex_pos.as_array_ref(),
            edge_pos_from: self.rdat.edge_pos_from.as_array_ref(),
            edge_pos_to: self.rdat.edge_pos_to.as_array_ref(),
            rotation_matrix: self.rdat.rotation_matrix.as_array_ref(),
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
    rotation_matrix: ArrayRef,
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
