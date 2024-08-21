use std::fs::File;
use std::io::{BufWriter, Write};

mod ppm;
mod vec3;
mod colors;
mod ray;

fn main() {
    let content = ppm::create_image();
    save_image(content);
}

fn save_image(content: String) {
    let f = File::create("img/image.ppm").unwrap();
    let mut bfwr = BufWriter::new(f);
    for line in content.lines() {
        writeln!(bfwr, "{line}").expect("Error escribiendo linea");
    }
}
