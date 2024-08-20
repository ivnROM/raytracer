pub fn create_image() -> String {
    // imagen
    let image_width = 256;
    let image_height = 256;

    let mut content = String::new();
    // render
    content.push_str(&format!("P3\n{} {}\n255\n", image_width, image_height).to_string());

    for j in 0..image_height {
        println!("Scanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            content.push_str(&format!("{} {} {}\n", ir, ig, ib).to_string());
        }
    }
    println!("Process finished!");
    content
}

