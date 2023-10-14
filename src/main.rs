mod ray;
mod vec3;

use vec3::*;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let aspect_ratio = 16. / 9.;
    let width = 256;
    let height = (width as f64 / aspect_ratio) as i32;

    let mut img_data = String::new();
    img_data.push_str(&format!("P3\n{} {}\n255\n", width, height));

    for j in 0..height {
        eprintln!("\rScanlines remaining: {} ", height - j);
        for i in 0..width {
            let r = 255 * i / (width - 1);
            let g = 255 * j / (height - 1);
            let b = 0;

            img_data.push_str(&format!("{r} {g} {b}\n"));
        }
    }

    File::create("img.ppm")
        .expect("Create failed.")
        .write_all(img_data.as_bytes())
        .expect("Write failed.");
}
