use crate::vec3::Color;
use crate::colors::write_color;

const ASPECT_R: f64 = 16.00 / 9.00;

pub fn create_image() -> String {
    // imagen
    let image_width = 400;
    let image_height = (image_width as f64 / (ASPECT_R.max(1.0))) as i32;
    //dbg!(image_width, image_height, ASPECT_R, image_width / image_height);
    //
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

    let mut content = String::new();

    content.push_str(&format!("P3\n{} {}\n255\n", image_width, image_height).to_string());
    // render
    for j in 0..image_height {
        //println!("Scanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.0;
            let pixel_color = Color::new([r, g, b]);
            content.push_str(&write_color(&pixel_color));
        }
    }
    println!("Process finished!");
    content
}

