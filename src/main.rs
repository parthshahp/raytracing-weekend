mod color;
mod vec3;
use color::{Color, write_color};
use vec3::Vec3;

fn main() {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .init();

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    let image_height = ((f64::from(image_width) / aspect_ratio) as u32).max(1);

    let viewport_height = 2.0;
    let viewport_width = viewport_height * f64::from(image_width / image_height);

    // Render
    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in 0..image_height {
        tracing::info!("\rScanlines Remaining: {} ", image_height - j);
        for i in 0..image_width {
            let pixel_color: Color = Vec3::from(
                f64::from(i) / f64::from(image_width - 1),
                f64::from(j) / f64::from(image_height - 1),
                0.0,
            );

            println!("{}", write_color(pixel_color));
        }
    }
    tracing::info!("\rDone.\n");
}
