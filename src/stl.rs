use crate::material::Material;
use crate::triangle::Triangle;
use crate::vec3::Vec3;
use crate::Num;

use regex::Regex;

use std::fs::File;
use std::io::{BufReader, Read};

fn get_u32(iter: &mut impl Iterator<Item = std::io::Result<u8>>) -> Option<u32> {
    Some(u32::from_le_bytes([
        iter.next()?.ok()?,
        iter.next()?.ok()?,
        iter.next()?.ok()?,
        iter.next()?.ok()?,
    ]))
}

pub fn read_bin(
    path: &str,
    scale: Vec3,
    offset: Vec3,
    material: Material,
) -> std::io::Result<Vec<Triangle>> {
    let file = BufReader::new(File::open(path)?);
    let mut bytes = file.bytes().skip(80);

    let n = get_u32(&mut bytes).unwrap();

    Ok((0..n)
        .map(|_| {
            let n1 = f32::from_bits(get_u32(&mut bytes).unwrap()) as Num;
            let n2 = f32::from_bits(get_u32(&mut bytes).unwrap()) as Num;
            let n3 = f32::from_bits(get_u32(&mut bytes).unwrap()) as Num;

            let a1 = f32::from_bits(get_u32(&mut bytes).unwrap()) as Num;
            let a2 = f32::from_bits(get_u32(&mut bytes).unwrap()) as Num;
            let a3 = f32::from_bits(get_u32(&mut bytes).unwrap()) as Num;

            let b1 = f32::from_bits(get_u32(&mut bytes).unwrap()) as Num;
            let b2 = f32::from_bits(get_u32(&mut bytes).unwrap()) as Num;
            let b3 = f32::from_bits(get_u32(&mut bytes).unwrap()) as Num;

            let c1 = f32::from_bits(get_u32(&mut bytes).unwrap()) as Num;
            let c2 = f32::from_bits(get_u32(&mut bytes).unwrap()) as Num;
            let c3 = f32::from_bits(get_u32(&mut bytes).unwrap()) as Num;

            bytes.next().unwrap().unwrap();
            bytes.next().unwrap().unwrap();

            Triangle::new(
                Vec3::new(n1, n2, n3) * scale,
                Vec3::new(a1, a2, a3) * scale + offset,
                Vec3::new(b1, b2, b3) * scale + offset,
                Vec3::new(c1, c2, c3) * scale + offset,
                material.clone(),
            )
        })
        .collect())
}

pub fn read(
    path: &str,
    scale: Num,
    offset: Vec3,
    material: Material,
) -> std::io::Result<Vec<Triangle>> {
    let mut file = BufReader::new(File::open(path)?);
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    let float = r"([\w\.\+\-]+)";
    let normal = format!(r"facet\s+normal\s+{0}\s+{0}\s+{0}", float);
    let vertex = format!(r"vertex\s+{0}\s+{0}\s+{0}", float);
    let facet = format!(
        r"{0}[[:space:]]*outer\s+loop[[:space:]]*{1}[[:space:]]*{1}[[:space:]]*{1}[[:space:]]*endloop[[:space:]]*endfacet",
        normal, vertex
    );
    let re_facet = Regex::new(&facet).unwrap();
    Ok(re_facet
        .captures_iter(&buffer)
        .map(|cap| {
            let n = Vec3::new(
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
                cap[3].parse().unwrap(),
            );

            let p1 = Vec3::new(
                cap[4].parse().unwrap(),
                cap[5].parse().unwrap(),
                cap[6].parse().unwrap(),
            ) * scale
                + offset;

            let p2 = Vec3::new(
                cap[7].parse().unwrap(),
                cap[8].parse().unwrap(),
                cap[9].parse().unwrap(),
            ) * scale
                + offset;

            let p3 = Vec3::new(
                cap[10].parse().unwrap(),
                cap[11].parse().unwrap(),
                cap[12].parse().unwrap(),
            ) * scale
                + offset;

            Triangle::new(n, p1, p2, p3, material.clone())
        })
        .collect::<Vec<_>>())
}
