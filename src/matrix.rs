use std::f64::consts::PI;

#[derive(Clone, Copy)]
pub enum Axis {
    X, Y, Z
}

pub struct Matrix4 {
    pub m: [[f64; 4]; 4]
}

impl Matrix4 {

    fn new(m: [[f64; 4]; 4]) -> Matrix4 {
        Matrix4 { m: m }
    }

    fn create_identity() -> Matrix4 {
        Matrix4::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ])
    }

    pub fn rot(axis: Axis, angle: f64) -> Matrix4 {
        let mut matrix = Matrix4::create_identity();

        match axis {
          Axis::X => {
              matrix.m[1][1] = angle.cos();
              matrix.m[1][2] = -angle.sin();
              matrix.m[2][1] = angle.sin();
              matrix.m[2][2] = angle.cos();
          },

          Axis::Y => {
              matrix.m[0][0] = angle.cos();
              matrix.m[0][2] = -angle.sin();
              matrix.m[2][0] = angle.sin();
              matrix.m[2][2] = angle.cos();
          },

          Axis::Z => {
              matrix.m[0][0] = angle.cos();
              matrix.m[0][1] = -angle.sin();
              matrix.m[1][0] = angle.sin();
              matrix.m[1][1] = angle.cos();
          }
        }

        return matrix;
    }
}

#[cfg(test)]
mod tests {
    use hamcrest::prelude::*;
    use std::f64::consts::PI;
    use matrix::Matrix4;
    use matrix::Axis::{X, Y, Z};

    // #[test]
    fn identity_matrix_can_be_created() {
        let m = Matrix4::create_identity();

        assert_that!(m.m[0][0], is(equal_to(1.0)));
        assert_that!(m.m[1][1], is(equal_to(1.0)));
        assert_that!(m.m[2][2], is(equal_to(1.0)));
        assert_that!(m.m[3][3], is(equal_to(1.0)));
    }

    // #[test]
    fn should_create_rotation_matrix_for_x_axis() {
        let m = Matrix4::rot(X, PI);

        assert_that!(m.m[1][1], is(equal_to(PI.cos())));
        assert_that!(m.m[1][2], is(equal_to(-PI.sin())));
        assert_that!(m.m[2][2], is(equal_to(PI.cos())));
        assert_that!(m.m[2][1], is(equal_to(PI.sin())));
    }

    // #[test]
    fn should_create_rotation_matrix_for_y_axis() {
        let m = Matrix4::rot(Y, PI);

        assert_that!(m.m[0][0], is(equal_to(PI.cos())));
        assert_that!(m.m[0][2], is(equal_to(-PI.sin())));
        assert_that!(m.m[2][2], is(equal_to(PI.cos())));
        assert_that!(m.m[2][0], is(equal_to(PI.sin())));
    }

    // #[test]
    fn should_create_rotation_matrix_for_z_axis() {
        let m = Matrix4::rot(Z, PI);

        assert_that!(m.m[0][0], is(equal_to(PI.cos())));
        assert_that!(m.m[0][1], is(equal_to(-PI.sin())));
        assert_that!(m.m[1][1], is(equal_to(PI.cos())));
        assert_that!(m.m[1][0], is(equal_to(PI.sin())));
    }
}
