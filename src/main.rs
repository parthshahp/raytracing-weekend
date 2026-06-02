mod color;
mod ray;
mod vec3;

use crate::color::{Color, write_color};
use crate::ray::Ray;
use crate::vec3::{Vec3, unit_vector};

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

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (f64::from(image_width) / f64::from(image_height));
    let camera_center = Vec3::new();

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::from(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::from(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / f64::from(image_width);
    let pixel_delta_v = viewport_v / f64::from(image_height);

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vec3::from(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render
    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in 0..image_height {
        tracing::info!("\rScanlines Remaining: {} ", image_height - j);
        for i in 0..image_width {
            // let pixel_color: Color = Vec3::from(
            //     f64::from(i) / f64::from(image_width - 1),
            //     f64::from(j) / f64::from(image_height - 1),
            //     0.0,
            // );

            let pixel_center =
                pixel00_loc + (f64::from(i) * pixel_delta_u) + (f64::from(j) * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(&r);

            println!("{}", write_color(pixel_color));
        }
    }
    tracing::info!("\rDone.\n");
}

fn ray_color(r: &Ray) -> Vec3 {
    let unit_direction = unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - a) * Color::from(1.0, 1.0, 1.0) + a * Color::from(0.5, 0.7, 1.0);
}
