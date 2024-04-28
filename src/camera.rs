use std::fs::File;
use std::io::{self, Write};
use crate::interval::*;
use crate::vec3::*;
use crate::ray::*;
use crate::color::*;
use crate::hittable::*;
use rand::Rng;

#[derive(Debug, Clone, Copy,Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub sample_per_pixel: i32,
    pub depth: i32,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_sample_scale: f64,
}

fn random_double() -> f64 {
    rand::thread_rng().gen::<f64>()
}

fn _random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

impl Camera {

    pub fn render(&mut self,world: &dyn Hittable) {
        self.initialize();
        let file = File::create("output2.ppm").expect("Failed");

        write!(&file, "P3\n{} {}\n255\n", self.image_width, self.image_height).expect("Failed to write");

        println!("P3\n  {}  {}  \n255\n", self.image_width, self.image_height);

        for i in 0..self.image_height {
            eprintln!("\rScanlines remaining {}", self.image_height - i);
            io::stdout().flush().unwrap();
            for j in 0..self.image_width {                
                let mut pixel_color : Color = Color::new(0.0,0.0,0.0);
                for _sample in 0..self.sample_per_pixel {
                    let r: Ray = self.get_ray(j,i);
                    pixel_color += self.ray_color(&r,&self.depth ,world);
                }
                let _ = write_color(&file, &(self.pixel_sample_scale * pixel_color));
            }
        }
        eprintln!("\rDone.           \n");
    }

    fn initialize(&mut self) {
        self.image_height = self.image_width / self.aspect_ratio as i32;
        self.image_height = if self.image_height < 1 { 1 } else { self.image_height };

        self.pixel_sample_scale = 1.0 / self.sample_per_pixel as f64;

        self.center =  Point3::new(0.0,0.0,0.0);

        let focal_length = 1.0; // Distance between camera and viewport
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);
    
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0); 

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left = self.center - Vec3::new(0.0,0.0,focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left +  0.5 *  (self.pixel_delta_u + self.pixel_delta_v) ;
    }

    
    fn ray_color(&self,r: &Ray,depth: &i32,world: &dyn Hittable) -> Color {
        
        if *depth <= 0 {
            return Color::new(0.0 ,0.0 ,0.0);
        }

        
        let mut rec : HitRecord  = HitRecord::default() ;

        if world.hit(r,Interval::new(0.001,std::f64::INFINITY),&mut rec) {
            let dir = rec.normal + random_unit_vector();
            return 0.5*self.ray_color(&Ray::new(rec.p,dir),&(depth - 1), world);
        }

        let unit_dir = unit_vector(*r.direction());
        let a = 0.5 * (unit_dir.y() + 1.0);
        return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
    }

    fn get_ray(&self,i:i32,j:i32) -> Ray {
        let offset = self.sample_square() ;
        let pixel_sample = self.pixel00_loc + ((i as f64 + offset.x()) * self.pixel_delta_u) + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_orgin = self.center ;
        let ray_direction = pixel_sample - ray_orgin ;

        return Ray::new(ray_orgin,ray_direction) ;
    }

    fn sample_square(&self) -> Vec3 {
        let ran = random_double() ;
        return Vec3::new(ran - 0.5, ran - 0.5 , 0.0) ;
    }

}