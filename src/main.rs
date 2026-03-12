use std::fmt::{self, Write};

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = [1u8, 2, 3, 4, 5];
    let mut ptr: *const u8 = data.as_ptr();
    let step = 2;
    let end_rounded_up = ptr.wrapping_offset(6);

    let mut out = String::new();
    while ptr != end_rounded_up {
        unsafe {
            write!(&mut out, "{}, ", *ptr)?;
        }
        ptr = ptr.wrapping_offset(step);
    }
    assert_eq!(out.as_str(), "1, 3, 5, ");
    Ok(())
}
