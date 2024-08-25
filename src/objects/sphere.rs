use crate::vec3::{Vec3, Point3};
use super::hittable::Hittable;

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Hittable for Sphere {

    pub fn hit(&self, r: &super::ray::Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut super::hittable::HitRecord) -> bool {
        let oc = self.center - *r.origin();

        let a = r.direction().length_squared();
        let h = Vec3::dotp(*r.direction(), oc);
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant = h*h - a*c;

        if discriminant < 0.0 {
            return false
        } 

        let sqrtd = discriminant.sqrt();
        let root = (h - sqrtd) / a;
        if (root <= ray_tmin) || (ray_tmax <= root) {
            let root = (h + sqrtd) / a;
            if (root <= ray_tmin) || (ray_tmax <= root) {
                return false
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;
        true
    }
}
