// use std::sync::Arc;
use crate::ray;
use crate::vec3;
use crate::interval;
// use crate::material::*;
use interval::Interval;
use vec3::{Vec3,Point3,dot};
use ray::Ray;


#[derive(Debug, Clone,Copy,Default)]
pub struct HitRecord { 
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool ;
}

impl HitRecord {
    pub fn set_face_normal(&mut self,r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(*r.direction(),*outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        }
        else{
           -*outward_normal
        };
    }

}