use utils;
use regex::Regex;
use image::{ImageBuffer, Rgb};

#[derive(Debug)]
struct Star {
    x_origin: i32,
    y_origin: i32,
    x_velocity: i32,
    y_velocity: i32,
    x: i32,
    y: i32,
}

impl Star {
    fn new(x_origin: i32, y_origin: i32, x_velocity: i32, y_velocity: i32) -> Self {
        Star{
            x_origin, y_origin, x_velocity, y_velocity, x: x_origin, y: y_origin
        }
    }

    fn load() -> Vec<Self> {
        let coordinate_inputs = utils::lines_from_file(String::from("day10.txt")).expect("Unable to load results from file");

        let mut stars: Vec<Star> = Vec::new();
        let matcher = Regex::new(r"^position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>").expect("Unable to create star regex");

        for coordinate_input in &coordinate_inputs{
            let captures = matcher.captures(coordinate_input).unwrap();

            let x = captures[1].parse::<i32>().expect("Unable to parse x coordinate");
            let y = captures[2].parse::<i32>().expect("Unable to parse y coordinate");
            let x_velocity = captures[3].parse::<i32>().expect("Unable to parse x velocity");
            let y_velocity = captures[4].parse::<i32>().expect("Unable to parse y velocity");

            stars.push(Star::new(x, y, x_velocity, y_velocity));
        }

        stars
    }

    fn time_tick(&mut self, seconds: i32) {
        self.x = self.x_origin + seconds * self.x_velocity;
        self.y = self.y_origin + seconds * self.y_velocity;
    }
}

fn implementation() {
    let mut stars = Star::load();

    let mut min_bounds = 1000000;
    let mut closest_bounds = 0;

    // Start by looping over a large number of possibly timesteps. We'll pick the one that has the smallest overall size, so find the timestep
    // that achieves that.
    for timeline in 0..=20000 {
        for star in &mut stars {
            star.time_tick(timeline);
        }

        // Work out min/max width and height so that we can choose an appropriate sized window to draw in
        let min_x = stars.iter().min_by_key(|star| star.x).unwrap().x;
        let min_y = stars.iter().min_by_key(|star| star.y).unwrap().y;
        let max_x = stars.iter().max_by_key(|star| star.x).unwrap().x;
        let max_y = stars.iter().max_by_key(|star| star.y).unwrap().y;

        let width: u32 = (max_x - min_x) as u32;
        let height: u32 = (max_y - min_y) as u32;

        if width + height < min_bounds {
            min_bounds = width + height;
            closest_bounds = timeline;
        }
    }

    // We should now have a rough guestimate of the closest bounds, so try a bit on either side and write out a hacky PNG with the results
    for timeline in (closest_bounds - 20)..(closest_bounds + 20) {
        for star in &mut stars {
            star.time_tick(timeline);
        }

        // Work out min/max width and height so that we can choose an appropriate sized window to draw in
        let min_x = stars.iter().min_by_key(|star| star.x).unwrap().x;
        let min_y = stars.iter().min_by_key(|star| star.y).unwrap().y;
        let max_x = stars.iter().max_by_key(|star| star.x).unwrap().x;
        let max_y = stars.iter().max_by_key(|star| star.y).unwrap().y;

        let width: u32 = (max_x - min_x) as u32;
        let height: u32 = (max_y - min_y) as u32;

        let timeline_string = timeline.to_string() + ".png";

        let black_pixel = Rgb([0, 0, 0]);
        let white_pixel = Rgb([255, 255, 255]);

        let mut img = ImageBuffer::<Rgb<u8>, Vec<u8>>::from_pixel(width + 1, height + 1, black_pixel);

        for star in &stars {
            img.put_pixel((star.x - min_x) as u32, (star.y - min_y) as u32, white_pixel);
        }

        img.save(timeline_string).unwrap();
    }

    println!("Smallest bounding box is at timestep {}", closest_bounds);
}


fn main() {
    implementation();
}
