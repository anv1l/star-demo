use glam::Vec3;
use rand::prelude::*;

pub struct Star {
    pub velocity: Vec3,
    pub position: Vec3,
    pub radius: f32,
    pub width: f32,
    pub height: f32,
}

impl Star {
    // Note this is an associated function - no self.
    pub fn new(width: f32, height: f32, rng: &mut ThreadRng) -> Self {
        Self {
            velocity: Vec3::new(
                Self::get_random(-5.0, 5.0, rng),
                Self::get_random(-5.0, 5.0, rng),
                Self::get_random(1.0, 5.0, rng),
            ),
            position: Vec3::new(0.0, 0.0, Self::get_random(0.0, width, rng)),
            radius: Self::get_random(0.1, 5.0, rng),
            width,
            height,
        }
    }

    #[allow(clippy::if_same_then_else)]
    pub fn update(
        &mut self,
        width: f32,
        height: f32,
        screenrotation: f32,
        speed: f32,
        rng: &mut ThreadRng,
    ) {
        if (self.position.x + width / screenrotation) < 0.0
            || (self.position.x - width / screenrotation) >= width
        {
            self.reset(width, height, rng);
        } else if (self.position.y + height / screenrotation) < 0.0
            || (self.position.y - height / screenrotation) >= screenrotation
        {
            self.reset(width, height, rng);
        } else if self.position.z <= 0.0 {
            self.reset(width, height, rng);
        } else {
            self.update_position(width, speed);
        }
    }

    fn update_position(&mut self, width: f32, speed: f32) {
        let multiplier = 1.0 - (width / self.position.z);
        self.position.x += self.velocity.x * speed * multiplier;
        self.position.y += self.velocity.y * speed * multiplier;
        if (self.position.z - self.velocity.z * speed) > 0.0 {
            self.position.z -= self.velocity.z * speed;
        }
    }

    fn get_random(min: f32, max: f32, rng: &mut ThreadRng) -> f32 {
        min + rng.gen_range(0.0..=1.0) * (max - min)
    }

    pub fn reset(&mut self, width: f32, height: f32, rng: &mut ThreadRng) {
        self.velocity = Vec3::new(
            Self::get_random(-5.0, 5.0, rng),
            Self::get_random(-5.0, 5.0, rng),
            Self::get_random(1.0, 5.0, rng),
        );
        self.position = Vec3::new(0.0, 0.0, width);
        self.radius = Self::get_random(1.0, 5.0, rng);
        self.width = width;
        self.height = height;
    }
}
