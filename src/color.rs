use std::io::Write;
use crate::vec3;
use crate::interval;
use interval::Interval;
use vec3::Vec3;
// use std::fs::OpenOptions;
pub type Color = Vec3;

pub fn linear_to_gamma(linear_comp: f64) -> f64 {
    if linear_comp > 0.0 {
        return f64::sqrt(linear_comp) ;
    }
    return 0.0;
}


pub fn write_color<W: Write>(mut out: W,pixel_color: &Color) -> std::io::Result<()>{
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    let intensity =  Interval::new(0.000, 0.999);

    let rbyte = (256.0 * intensity.clamp(r)) as i32;
    let gbyte = (256.0 * intensity.clamp(g)) as i32;
    let bbyte = (256.0 * intensity.clamp(b)) as i32;

    /* Testing */

    // let mut file = OpenOptions::new()
    //     .create(true)
    //     .append(true)
    //     .open("oo.txt")?;

    // let data = format!("{} {} {}\n", rbyte, gbyte, bbyte) ;
    // let __ = file.write_all(data.as_bytes());

    let _ =write!(out, "{} {} {}\n", rbyte, gbyte, bbyte);
    Ok(())
}