mod vec3;

fn main() {
    let (w, h) = (256, 256);

    println!("P3\n{} {}\n255", w, h);

    for j in 0..h {
        eprintln!("\rScanlines remaining: {} ", h - j);
        for i in 0..w {
            let r = 255 * i / (w - 1);
            let g = 255 * j / (h - 1);
            let b = 0;

            println!("{} {} {}", r, g, b);
        }
    }
}
