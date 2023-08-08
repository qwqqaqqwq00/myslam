use std::{borrow::Cow, io::BufRead, iter::FlatMap};

use cgmath::*;
use winit::{event_loop::EventLoop, window::Window};

use crate::trajectory::common::Vertex;

use super::common;

fn parse_file(filename: String) -> common::Data {
    let file = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file).lines();
    let mut v_list: Vec<common::Vertex> = Vec::new();
    let tb = vec![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    let mut mtb = tb.iter();
    for line in reader {
        let uline = line.unwrap();
        let lx = uline.split(' ').collect::<Vec<&str>>();
        let vtx: Vertex = Vertex {
            position: [lx[1].parse::<f32>().unwrap() * 10.0, lx[2].parse::<f32>().unwrap() * 10.0, 0.0],
            color: match mtb.next() {
                None => {
                    mtb = tb.iter();
                    *mtb.next().unwrap()
                },
                Some(v) => *v,
            },
            // color: match 
        };
        if v_list.len() > 50 {break}
        v_list.push(vtx);
    }
    // v_list.push(Vertex { position: [0.0, 0.0, 0.0], color: [0.0, 0.0, 1.0] });
    // let idx = (0..v_list.len())
    //     .map(|i| vec![i as u16, (i+1) as u16, (i+2) as u16])
    //     .flatten()
    //     .collect::<Vec<u16>>();
    common::Data {
        vertex: v_list.clone(),
        // index: idx,
        // nvert: idx.len() as u32,
        nvert: v_list.len() as u32,
    }
}

fn test_case() -> common::Data {
    common::Data { vertex: 
        vec![
            Vertex { position: [-0.63, 0.8, 0.0], color: [1.0, 0.0, 0.0]},
            Vertex { position: [-0.65, 0.2, 0.0], color: [0.0, 1.0, 0.0]},
            Vertex { position: [-0.2, 0.6, 0.0], color: [0.0, 0.0, 1.0]},
            Vertex { position: [-0.37, -0.07, 0.0], color: [1.0, 0.0, 0.0]},
            Vertex { position: [0.05, 0.18, 0.0], color: [0.0, 1.0, 0.0]},
            Vertex { position: [-0.13, -0.4, 0.0], color: [0.0, 0.0, 1.0]},
            Vertex { position: [0.3, -0.13, 0.0], color: [1.0, 0.0, 0.0]},
            Vertex { position: [0.13, -0.64, 0.0], color: [0.0, 1.0, 0.0]},
            Vertex { position: [0.7, -0.3, 0.0], color: [0.0, 0.0, 1.0]},
        ], nvert: 9 }
}

pub fn plot_tradjectory() -> (EventLoop<()>, Window, common::Input<'static>, common::Data) {
    println!("Working directory: {}", std::env::current_dir().unwrap().display());
    let evlp = EventLoop::new();
    let window = winit::window::Window::new(&evlp).unwrap();

    let trajectory_file: String = String::from("./data/trajectory.txt");
    let mut primitive_type = "point-list";
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        primitive_type = &args[1];
        println!("{}", primitive_type);
    }

    let mut topology = wgpu::PrimitiveTopology::PointList;
    let mut index_format = None;
    if  primitive_type == "line-list" {
        topology = wgpu::PrimitiveTopology::LineList;
        index_format = None;
    } else if  primitive_type == "line-strip" {
        topology = wgpu::PrimitiveTopology::LineStrip;
        index_format = Some(wgpu::IndexFormat::Uint32);
    } else if primitive_type == "triangle-strip" {
        topology = wgpu::PrimitiveTopology::TriangleStrip;
        index_format = Some(wgpu::IndexFormat::Uint32);
    }
    let inputs = common::Input {
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        topology: topology,
        strip_index_format: index_format,
    };
    let data = parse_file(trajectory_file);
    // let data = test_case();
    // println!("{:?}", data);
    // dbg!(data.vertex)
    // pollster::block_on(common::run(evlp, window, inputs, data));
    (evlp, window, inputs, data)
}

fn gl_draw() {
    
}

