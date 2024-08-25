mod utils;
mod vec3;
mod objects;
mod render;

fn main() {
    let content = render::create_image();
    utils::file_operations::save_image(content);
}

