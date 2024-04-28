use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::vec3::{Point3,dot};
use crate::interval;
use interval::Interval;
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Sphere {
            center,
            radius: radius.max(0.0),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - *r.origin();
        let a = r.direction().length_squared();
        let h = dot(*r.direction(),oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discrminant = h*h-a*c;
        if discrminant < 0.0 {
            return false;
        }

        let sqrtd = discrminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal ( r, &outward_normal) ;

        true
    }
}