use vector::Vector;
use ray::Ray;
use hitable::Intersection;

pub trait Material: Sync + Send {
    // produce a scattered ray unless the incident
    // ray is absorbed, in which case None is returned
    fn scatter(&self,
               incident: &Ray,
               intersection: &Intersection,
               ++++++++++++++
               attenuation: &mut Vector)
               -> Option<Ray>;
}

#[derive(Copy, Clone, Debug)]
pub struct Lambertian {
    pub albedo: Vector,
}

impl Material for Lambertian {
    fn scatter(&self,
               incident: &Ray,
               intersection: &Intersection,
               attenuation: &mut Vector)
               -> Option<Ray> {

        match *intersection {
            Intersection::Hit { position, normal, .. } => {
                let target = position + normal + Vector::random_in_unit_sphere();
                let scattered = Ray {
                    origin: position,
                    direction: target - position,
                };

                *attenuation = self.albedo;

                Some(scattered)
            }
            _ => None,
        }

    }
}

pub struct Metallic {
    pub albedo: Vector,
}

impl Material for Metallic {
    fn scatter(&self,
               incident: &Ray,
               intersection: &Intersection,
               attenuation: &mut Vector)
               -> Option<Ray> {

        match *intersection {
            Intersection::Hit { position, normal, .. } => {
                let reflected = incident.direction.normalize().reflect(&normal);
                let scattered = Ray {
                    origin: position,
                    direction: reflected,
                };

                *attenuation = self.albedo;

                if scattered.direction.dot(&normal) > 0.0 {
                    return Some(scattered);
                }
                None
            }
            _ => None,
        }

    }
}
