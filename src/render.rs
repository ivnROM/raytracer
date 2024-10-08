#![allow(dead_code)]
use super::vec3::{Color, Vec3, Point3};
use super::objects::ray::Ray;
use super::utils::colors::write_color;

const ASPECT_R: f64 = 16.00 / 9.00;

pub fn create_image() -> String {
    // imagen
    let image_width = 400;
    let image_height = (image_width as f64 / (ASPECT_R.max(1.0))) as i32;

    // camara
    let focal_length = 1.0;
    let camera_center = Point3::default();

    // Alto del viewport (escena que capta la camara)
    let viewport_height = 2.0;

    // Largo del viewport (escena que capta la camara)
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

    // Vector que atraviesa el rango horizontal del viewport
    let viewport_u = Vec3::new([viewport_width, 0.0, 0.0]);

    // Vector que atraviesa el rango vertical del viewport
    let viewport_v = Vec3::new([0.0, -viewport_height, 0.0]);

    // Separación horizontal de los pixeles dentro del viewport
    let pixel_delta_u = viewport_u / image_width as f64;

    // Separación vertical de los pixeles dentro del viewport
    let pixel_delta_v = viewport_v / image_height as f64;

    // extremo izquierdo superior del viewport
    let viewport_upper_left: Point3 = camera_center - Vec3::new([0.0, 0.0, focal_length]) - viewport_u / 2.0 - viewport_v / 2.0;

    // ubicacion del primer pixel dentro del viewport
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) / 2.0;


    let mut content = String::new(); content.push_str(&format!("P3\n{} {}\n255\n", image_width, image_height).to_string());
    // render
    for j in 0..image_height {
        println!("Scanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            
            let pixel_center: Point3 = pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(&r);
            content.push_str(&write_color(&pixel_color));
        }
    }
    println!("Process finished!");
    content
}

/// Returns the color of the given ray
fn ray_color(r: &Ray) -> Color {

    let t =  hit_sphere(Point3::new([0.0, 0.0, -1.0]), 0.5, r);
    if t > 0.0 {
        let nrmal = Vec3::unit_v(r.at(t) - Vec3::new([0.0, 0.0, -1.0]));
        return 0.5*Color::new([nrmal.x() + 1.0, nrmal.y() + 1.0, nrmal.z() + 1.0])
    }

    let unit_v = Vec3::unit_v(*r.direction());
    let alpha = (unit_v.y() + 1.0) / 2.0;
    (1.0 - alpha) * Color::new([1.0, 1.0, 1.0]) + alpha * Color::new([0.5, 0.7, 1.0])
}

//fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
//    let oc = center - *r.origin();
//
//    let a = r.direction().length_squared();
//    let h = Vec3::dotp(*r.direction(), oc);
//    let c = oc.length_squared() - radius*radius;
//
//    let discriminant = h*h - a*c;
//
//    if discriminant < 0.0 {
//        return -1.0
//    } else {
//        return (h - discriminant.sqrt()) / (2.0 * a);
//    }
//}
