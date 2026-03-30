use std::{fmt::{self}, time::Instant};

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
    z: f64
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

#[derive(Clone, Copy)]
struct Matrix3 {
    m: [[f64; 3]; 3],
}

impl Matrix3 {
    fn identity() -> Matrix3 {
        Matrix3 {
            m: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }

    fn from_rows(r1: [f64; 3], r2: [f64; 3], r3: [f64; 3]) -> Matrix3 {
        Matrix3 { m: [r1, r2, r3] }
    }

    fn mul_vector(&self, v: Vector) -> Vector {
        Vector {
            x: self.m[0][0] * v.x + self.m[0][1] * v.y + self.m[0][2] * v.z,
            y: self.m[1][0] * v.x + self.m[1][1] * v.y + self.m[1][2] * v.z,
            z: self.m[2][0] * v.x + self.m[2][1] * v.y + self.m[2][2] * v.z,
        }
    }

    fn mul_matrix(&self, other: Matrix3) -> Matrix3 {
        let mut out = [[0.0; 3]; 3];

        for i in 0..3 {
            for j in 0..3 {
                out[i][j] = self.m[i][0] * other.m[0][j]
                    + self.m[i][1] * other.m[1][j]
                    + self.m[i][2] * other.m[2][j];
            }
        }

        Matrix3 { m: out }
    }
}

impl fmt::Display for Matrix3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{:.2}, {:.2}, {:.2}]\n[{:.2}, {:.2}, {:.2}]\n[{:.2}, {:.2}, {:.2}]",
            self.m[0][0],
            self.m[0][1],
            self.m[0][2],
            self.m[1][0],
            self.m[1][1],
            self.m[1][2],
            self.m[2][0],
            self.m[2][1],
            self.m[2][2]
        )
    }
}

impl Vector {
    fn mul(&self, v: Vector) -> Vector {
        return Vector { x:self.x * v.x, y: self.y * v.y, z: self.z * v.z }
    }

    fn add(&self, v: Vector) -> Vector {
        return Vector {x:self.x + v.x, y: self.y + v.y, z: self.z * v.z}
    }

    fn dotproduct(&self, v: Vector) -> f64 {
        return (self.x * v.x) + (self.y * v.y) + (self.z * v.z);
    }

    fn crossproduct(&self, v: Vector) -> Vector {
        return Vector { x: self.y * v.z - v.y * self.z, y: self.x * v.z - v.x * self.z, z: self.x * v.y - v.x * self.y}; 
    }

    fn normalization(&self) -> Vector {
        let strecth = (&self.x.powi(2) + &self.y.powi(2) + &self.z.powi(2)).sqrt();
        return Vector {x: self.x/strecth, y: self.y/strecth, z: self.z/strecth};
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x = {0} y = {1} z = {2}", self.x, self.y, self.z)
    }
}

fn instantiate_of_AOS() {
    let init_now = Instant::now();
    let mut particles1 = vec![Particle {x: 10, y: 10, z: 10}; 1_000_000_000_0];
    let init_elapsed = init_now.elapsed();
    println!("AoS initialization elapsed: {:.2?}", init_elapsed);

    let now = Instant::now();
    for particle in particles1.iter_mut() {
        particle.x = 20;
    }
    let elapsed = now.elapsed();
    println!("AoS iteration elapsed: {:.2?}", elapsed);
}

fn instantiate_of_SOA() {
    let init_now = Instant::now();
    let mut a = Particles {
        x: vec![10; 1_000_000_000_0], 
        y: vec![10; 1_000_000_000_0], 
        z: vec![10; 1_000_000_000_0]
    };
    let init_elapsed = init_now.elapsed();
    println!("SoA initialization elapsed: {:.2?}", init_elapsed);

    let now = Instant::now();
    for el in a.x.iter_mut() {
        *el = 20;
    }
    let elapsed = now.elapsed();
    println!("SoA iteration elapsed: {:.2?}", elapsed);
}

fn main() {
    let v1= Vector {x:2.0, y: 1.0, z: 3.0};
    let v2= Vector {x:4.0, y: 5.0, z:6.0};

    let v3 = v1.crossproduct(v2.clone());
    println!("{}", v3);

    let transform = Matrix3::from_rows(
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0],
        [0.0, 0.0, 3.0],
    );
    let transformed_v = transform.mul_vector(v1.clone());
    println!("Transformed vector: {}", transformed_v);

    let composed = Matrix3::identity().mul_matrix(transform);
    println!("Composed matrix:\n{}", composed);

    let vnormalized = v1.normalization();
    println!("{}", vnormalized);
}