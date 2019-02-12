use crate::vec3::Vec3;
use std::fs::File;
use std::io::{BufWriter, Write};
mod vec3;

type Num = f32;
type Int = i32;

fn main() -> std::io::Result<()> {
    let mut file = BufWriter::new(File::create("hello.ppm")?);

    let nx = 800;
    let ny = 600;

    write!(&mut file, "P3\n{} {}\n255\n", nx, ny)?;
    for j in (0..ny).rev() {
        for i in 0..nx {
            let vec = Vec3::new(
                i as Num,
                j as Num,
                0.0,
                (i as Num) / (nx as Num),
                (j as Num) / (ny as Num),
                0.2,
            );

            let ir = (255.99 * vec.r) as Int;
            let ig = (255.99 * vec.g) as Int;
            let ib = (255.99 * vec.b) as Int;
            write!(&mut file, "{} {} {}\n", ir, ig, ib)?;
        }
    }
    Ok(())
}
