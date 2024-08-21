mod utils;
mod ppm;
mod vec3;
mod colors;
mod ray;

fn main() {
    let content = ppm::create_image();
    utils::file_ops::save_image(content);
}

