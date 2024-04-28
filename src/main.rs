pub mod color;
pub mod ray;
pub mod vec3;
pub mod hittable;
pub mod hittable_list;
pub mod sphere;
pub mod interval;
pub mod camera;
use camera::*;
use vec3::Point3;
use hittable_list::HittableList;
use sphere::Sphere;
use std::sync::Arc;

fn main() {
    let mut world : HittableList = HittableList::new();

    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let mut cam: Camera = Default::default();

    cam.aspect_ratio = 16.0 / 9.0 ;
    cam.image_width = 400 ;
    cam.sample_per_pixel = 100 ;
    cam.depth = 50 ;

    cam.render(&world);

}

