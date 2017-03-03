use vector::Vector;
use ray::Ray;
use material::Material;
use material::Lambertian;

use std::sync::Arc;

#[derive(Clone)]
pub enum Intersection {
    Miss,
    Hit {
        t: f64,
        position: Vector,
        normal: Vector,
        material: Arc<Material>,
    },
}

pub trait Hitable: Sync + Send {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Intersection;
}

#[derive(Clone)]
pub struct Sphere {
    pub center: Vector,
    pub radius: f64,
    pub material: Arc<Material>,
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Intersection {
        // sphere: dot((p - c), (p - c)) = r * r;
        // ray: a + b * t = p
        // substitute: dot((a + b * t - c), (a + b * t - c)) = r * r
        // expand: dot(b, b) * t * t + 2 * t * dot(b, a - c) + dot(a - c, a - c) - r * r = 0

        // the discriminant of the resulting quadratic equation will either be
        // positive (two real solutions), negative (no real solutions), or zero
        // (one real solution)

        let oc = r.origin - self.center;
        let a = r.direction.dot(&r.direction);
        let b = oc.dot(&r.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let t: f64 = temp;
                let position = r.point_at(t);
                let normal = (position - self.center) / self.radius;
                return Intersection::Hit {
                    t: t,
                    position: position,
                    normal: normal,
                    material: self.material.clone(),
                };
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let t: f64 = temp;
                let position = r.point_at(t);
                let normal = (position - self.center) / self.radius;
                return Intersection::Hit {
                    t: t,
                    position: position,
                    normal: normal,
                    material: self.material.clone(),
                };
            }
        }
        Intersection::Miss
    }
}

impl Default for Sphere {
    fn default() -> Sphere {
        Sphere {
            center: Vector::origin(),
            radius: 1.0,
            material: Arc::new(Lambertian { albedo: Vector::one() }),
        }
    }
}

pub struct HitableList {
    pub items: Vec<Box<Hitable>>,
}

impl HitableList {
    pub fn new() -> HitableList {
        HitableList { items: Vec::new() }
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Intersection {
        let mut intersect = Intersection::Miss;
        let mut closest_so_far = t_max;

        // test against every object and find the closest point of intersection
        for i in &self.items {
            match i.hit(&r, t_min, t_max) {
                Intersection::Hit { t, position, normal, ref material } if t < closest_so_far => {
                    closest_so_far = t;
                    intersect = Intersection::Hit {
                        t: t,
                        position: position,
                        normal: normal,
                        material: material.clone(),
                    };
                }
                _ => continue,
            }
        }
        intersect
    }
}
