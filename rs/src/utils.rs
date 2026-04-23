use ndarray::Array1;

/// Computes the l2 (euclidean) norm of a vector
pub fn norm_l2(v: &Array1<f32>) -> f32 {
    v.map(|&v| v * v).sum().sqrt()
}

/// Normalizes a vector in place
pub fn normalize(mut v: Array1<f32>) -> Array1<f32> {
    let norm = norm_l2(&v);
    v /= norm;
    v
}
