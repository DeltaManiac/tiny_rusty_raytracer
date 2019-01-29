mod vec3;
use std::fs::File;
use std::io::BufWriter;
use vec3::Vec3;

fn render() {
    let file = File::open("a.ppm").expect("Could not open file");
    let buffer = BufWriter::new(file);
    let (width, height) = (1024, 786);
    let mut framebuffer: Vec<Vec3> = Vec::new();
    for i in 0..width {
        for j in 0..height {
            framebuffer.push(Vec3::new(
                i as f32 / width as f32,
                j as f32 / height as f32,
                0.0,
            ));
        }
    }
}

fn main() {
    println!("Hello, world!");
}
