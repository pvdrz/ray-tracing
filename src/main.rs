mod vector;

use std::fs::File;
use std::io::{BufWriter, Write};

use vector::{Float, Vector};

fn main() {
    const NX: usize = 200;
    const NY: usize = 100;

    let file = File::create("out/hello_world.ppm").unwrap();
    let mut buffer = BufWriter::new(file);
    // P3 means the colors are in ascii
    // NX is the number of columns
    // NY is the number of rows
    // 255 is the maximum channel value
    write!(buffer, "P3\n{} {}\n255\n", NX, NY).unwrap();
    // rows go from top to bottom
    for j in (0..NY).rev() {
        // columns go from left to right
        for i in 0..NX {
            let color = Vector::new(
                (i as Float) / (NX as Float),
                (j as Float) / (NY as Float),
                0.2,
            );

            let ir = (255.99 * color.r()) as u8;
            let ig = (255.99 * color.g()) as u8;
            let ib = (255.99 * color.b()) as u8;

            write!(buffer, "{} {} {}\n", ir, ig, ib).unwrap();
        }
    }
}
