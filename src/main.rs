use std::{fmt::{self}, mem, time::Instant};

#[derive(Clone)]
struct Particle {
    y: u8,
    z: u8,
    x: u8,
}

struct Particles {
    x: Vec<u8>,
    y: Vec<u8>,
    z: Vec<u8>,
}

#[derive(Clone)]
struct Vector {
    x: f64,
    y: f64,
}

#[repr(C)] 
struct Vertex2 {
    position: [f32;3],
    color: [f32;3],
}

#[repr(C)]
struct Vertex {
    position: [f32;3],
    color: [f32;3],
}

impl Vector {
    fn mul(&self, v: Vector) -> Vector {
        return Vector { x:self.x * v.x, y: self.y * v.y }
    }

    fn add(&self, v: Vector) -> Vector {
        return Vector {x:self.x + v.x, y: self.y + v.y}
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x = {0} y = {1}", self.x, self.y)
    }
}

fn instantiate_of_AOS() {
    // Fast initialization using the Clone trait
    let init_now = Instant::now();
    let mut particles1 = vec![Particle {x: 10, y: 10, z: 10}; 1_000_000_000_0];
    let init_elapsed = init_now.elapsed();
    println!("AoS initialization elapsed: {:.2?}", init_elapsed);

    // Start timer AFTER allocation
    let now = Instant::now();
    for particle in particles1.iter_mut() {
        particle.x = 20;
    }
    let elapsed = now.elapsed();
    println!("AoS iteration elapsed: {:.2?}", elapsed);
}

fn instantiate_of_SOA() {
    // Fast initialization using vec! macro
    let init_now = Instant::now();
    let mut a = Particles {
        x: vec![10; 1_000_000_000_0], 
        y: vec![10; 1_000_000_000_0], 
        z: vec![10; 1_000_000_000_0]
    };
    let init_elapsed = init_now.elapsed();
    println!("SoA initialization elapsed: {:.2?}", init_elapsed);

    // Start timer AFTER allocation
    let now = Instant::now();
    for el in a.x.iter_mut() {
        *el = 20;
    }
    let elapsed = now.elapsed();
    println!("SoA iteration elapsed: {:.2?}", elapsed);
}

fn main() {
}