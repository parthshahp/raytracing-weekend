use rand::{Rng, RngExt};

use crate::{
    math::{
        interval::Interval,
        ray::Ray,
        vec3::{Vec3, random_on_hemisphere, unit_vector},
    },
    objects::hittable::Hittable,
    render::color::{Color, write_color},
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    image_height: u32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            image_height: 0,
            center: Vec3::default(),
            pixel00_loc: Vec3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
            pixel_samples_scale: 0.0,
        }
    }
}

impl Camera {
    pub fn render(&mut self, world: &impl Hittable) {
        self.initialize();

        // Render
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        let mut rng = rand::rng();

        for j in 0..self.image_height {
            tracing::info!("\rScanlines Remaining: {} ", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::from(0.0, 0.0, 0.0);
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j, &mut rng);
                    pixel_color += Camera::ray_color(&r, self.max_depth, world);
                }
                println!("{}", write_color(pixel_color * self.pixel_samples_scale));
            }
        }
    }

    fn initialize(&mut self) {
        self.image_height = ((f64::from(self.image_width) / self.aspect_ratio) as u32).max(1);
        self.pixel_samples_scale = 1.0 / f64::from(self.samples_per_pixel);

        // Camera
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width =
            viewport_height * (f64::from(self.image_width) / f64::from(self.image_height));
        self.center = Vec3::new();

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::from(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::from(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / f64::from(self.image_width);
        self.pixel_delta_v = viewport_v / f64::from(self.image_height);

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - Vec3::from(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn get_ray(&self, i: u32, j: u32, rng: &mut impl Rng) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location i, j.

        let offset = Camera::sample_square(rng);
        let pixel_sample = self.pixel00_loc
            + ((f64::from(i) + offset.x()) * self.pixel_delta_u)
            + ((f64::from(j) + offset.y()) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn ray_color(r: &Ray, max_depth: u32, world: &impl Hittable) -> Vec3 {
        if max_depth == 0 {
            return Color::from(0.0, 0.0, 0.0);
        }
        // If anything in the world is hit, return the "correct" color
        if let Some(rec) = world.hit(r, Interval::new(0.001, f64::INFINITY)) {
            let direction = random_on_hemisphere(&rec.normal);
            return 0.5 * Self::ray_color(&Ray::new(rec.p, direction), max_depth - 1, world);
            // return 0.5 * (rec.normal + Color::from(1.0, 1.0, 1.0));
        }

        let unit_direction = unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        // This is that sky blue color
        (1.0 - a) * Color::from(1.0, 1.0, 1.0) + a * Color::from(0.5, 0.7, 1.0)
    }

    fn sample_square(rng: &mut impl Rng) -> Vec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        // TODO: Make this nicer
        let x: f64 = rng.random();
        let y: f64 = rng.random();
        Vec3::from(x - 0.5, y - 0.5, 0.0)
    }
}
