use std::fs::File;
use std::io::{BufWriter, Write};

pub fn save_image(content: String) {
    let f = File::create("img/image.ppm").unwrap();
    let mut bfwr = BufWriter::new(f);
    for line in content.lines() {
        writeln!(bfwr, "{line}").expect("Error escribiendo linea");
    }
}
