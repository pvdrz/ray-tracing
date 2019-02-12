use std::io::{Write, BufWriter};
use std::fs::File;

fn main() -> std::io::Result<()> {
    let mut file = BufWriter::new(File::create("hello.ppm")?);

    let nx = 800;
    let ny = 600;

    write!(&mut file ,"P3\n{} {}\n255\n", nx, ny)?;
    for j in (0..ny).rev() {
        for i in 0..nx {
            let r = (i as f32) / (nx as f32);
            let g = (j as f32) / (ny as f32);
            let b = 0.2;

            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;
            write!(&mut file, "{} {} {}\n", ir, ig, ib)?;
        }
    }
    Ok(())
}
