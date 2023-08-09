use std::{io::BufRead, f32::consts::LOG10_2};

use cgmath::{Quaternion, Vector3, Transform, Matrix4, Vector4, SquareMatrix};

fn log_mat4(mat: Matrix4<f32>) -> Matrix4<f32> {
    // let e = f32::exp(1.0);
    let v = Matrix4::<f32>::identity();
    // Matrix4 {
    //     x: mat.x.map(|v| v.ln()),
    //     y: mat.y.map(|v| v.ln()),
    //     z: mat.z.map(|v| v.ln()),
    //     w: 
    // }
    todo!()
}

fn read_traj(path: String) -> Vec<cgmath::Matrix4<f32>> {
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file).lines();
    let mut v_list: Vec<cgmath::Matrix4<f32>> = Vec::new();
    for line in reader {
        let pl: Vec<f32> = line.unwrap().split(' ').map(|x| x.parse::<f32>().unwrap()).collect();
        let q = Quaternion::new(pl[7], pl[4], pl[5], pl[6]);
        
        let v = Vector3::new(pl[1], pl[2], pl[3]);
        let m = cgmath::Matrix4::from_translation(v) * cgmath::Matrix4::from(q);
        
        v_list.push(m);
    }
    v_list
}

fn traj_err() {
    let groundtruth_file = String::from("../../data/groundtruth.txt");
    let estim_file = String::from("../../data/estimated.txt");
    let groundtruth = read_traj(groundtruth_file);
    let estim = read_traj(estim_file);
    // compute rmse
    let mut rmse = 0.0;
    for i in 0..estim.len() {
        let (p1, p2) = (estim[i], groundtruth[i]);
        let err = p2.inverse_transform().unwrap() * p1;
        
    }
}
