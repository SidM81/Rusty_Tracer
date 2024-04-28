use crate::vec3;

use vec3::Vec3;
use vec3::Point3;

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new (origin: Point3,direction: Vec3) -> Self{
        Ray {orig: origin,dir: direction}
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn at(&self,t: f64) -> Vec3 {
        let at_ = self.orig + self.dir*t;
        at_
    }
}