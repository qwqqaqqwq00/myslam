use cgmath::{prelude::*, Matrix4, Quaternion, Vector3, Matrix3, Euler, Vector4, BaseFloat};


pub trait Rotate<T>: Sized {
    fn rot(self) -> Matrix3<T>;
}

pub trait Translate<T>: Sized {
    fn trans(self) -> Vector3<T>;
}

impl<T> Rotate<T> for Matrix4<T> {
    fn rot(self) -> Matrix3<T> {
        Matrix3::new(self.x.x, self.x.y, self.x.z, self.y.x, self.y.y, self.y.z, self.z.x, self.z.y, self.z.z)
    }
}

impl<T> Translate<T> for Matrix4<T> {
    fn trans(self) -> Vector3<T> {
        Vector3 { x: self.w.x, y: self.w.y, z: self.w.z }
    }
}

pub trait FillRotate<T>: Sized {
    fn fill_rot(&mut self, mat: Matrix3<T>) -> Self;
}

impl<T> FillRotate<T> for Matrix4<T> 
where
    T: BaseFloat + Clone + Copy
{
    fn fill_rot(&mut self, mat: Matrix3<T>) -> Self {
        self.x.x = mat.x.x;
        self.x.y = mat.x.y;
        self.x.z = mat.x.z;
        self.y.x = mat.y.x;
        self.y.y = mat.y.y;
        self.y.z = mat.y.z;
        self.z.x = mat.z.x;
        self.z.y = mat.z.y;
        self.z.z = mat.z.z;
        *self
    }
}

pub trait FillTranslate<T>: Sized {
    fn fill_trans(&mut self, mat: Vector3<T>) -> Self;
}

impl<T> FillTranslate<T> for Matrix4<T> 
where
    T: BaseFloat + Copy + Clone
{
    fn fill_trans(&mut self, mat: Vector3<T>) -> Self {
        self.w.x = mat.x;
        self.w.y = mat.y;
        self.w.z = mat.z;
        *self
    }
}

fn sim<T>(mat: &mut Matrix4<T>, scale: T) -> Matrix4<T> 
where T: std::ops::MulAssign + BaseFloat + Copy + Clone
{
    let mut q = mat.rot();
    q *= scale;
    mat.fill_trans(mat.trans()).fill_rot(q)
}

// fn affine<S>(mat: Matrix4<S>, )

#[cfg(test)]
mod tests {
    use cgmath::Rad;

    use super::*;
    #[test]
    fn test_sim() {
        let mut m: Matrix4<f32> = Matrix4::from_angle_y(Rad(30.0));
        m.x.w = 1.0;
        // dbg!(m);
        let qm = sim(&mut m, 12.0);
        // dbg!(qm);
        assert_ne!(m.x * 12.0, qm.x);
        assert_eq!(m.y * 12.0, qm.y);
        assert_eq!(m.z * 12.0, qm.z);
        assert_eq!(m.w, qm.w);
    }
}