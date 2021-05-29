use rand::{RngCore, SeedableRng};
use rand_seeder::Seeder;
use rand_xoshiro::Xoshiro256StarStar;

#[cfg(all(feature = "bevy-nightly", not(feature = "bevy-stable")))]
use bevy_nightly as bevy;
#[cfg(all(feature = "bevy-stable", not(feature = "bevy-nightly")))]
use bevy_stable as bevy;

use bevy::prelude::*;


pub use rand::Rng as _;

/// `RngPlugin` allows you to inject a (optionally seeded) random number
/// generator into your systems.
///
/// Once the plugin is active, you can use `Local<bevy_rng::Rng>` to get the
/// rng instance.
///
/// Using `Local<T>` ensures you get a unique copy of the rng for each
/// individual system, which is important, because systems have no deterministic
/// order, which will result in non-deterministic rng results.
///
/// You are still responsible for deterministically generating random numbers
/// _inside_ an individual system, which (currently) means you can't generate
/// random numbers when iterating over entities, as entity iteration also isn't
/// ordered currently.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RngPlugin {
    seed: Option<Seed>,
}

impl From<String> for RngPlugin {
    fn from(seed: String) -> Self {
        Self {
            seed: Some(Seed::String(seed)),
        }
    }
}

impl From<&str> for RngPlugin {
    fn from(seed: &str) -> Self {
        Self {
            seed: Some(Seed::String(seed.to_owned())),
        }
    }
}

impl From<u64> for RngPlugin {
    fn from(seed: u64) -> Self {
        Self {
            seed: Some(Seed::Number(seed)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Seed {
    Number(u64),
    String(String),
}

impl Plugin for RngPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let rng = match &self.seed {
            Some(Seed::String(seed)) => Seeder::from(seed.as_str()).make_rng(),
            Some(Seed::Number(num)) => Xoshiro256StarStar::seed_from_u64(*num),
            None => Xoshiro256StarStar::from_entropy(),
        };

        app.insert_resource(RootRng { rng });
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RootRng {
    rng: Xoshiro256StarStar,
}

/// The Rng resource.
///
/// This wraps a random number generator.
///
/// See the `rand::Rng` trait for more details on how to generate random data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rng {
    inner: Xoshiro256StarStar,
}

impl Rng {
    pub fn next_u32(&mut self) -> u32 {
        self.inner.next_u32()
    }
    pub fn next_u64(&mut self) -> u64 {
        self.inner.next_u64()
    }
    pub fn next_f32(&mut self) -> f32 {
        self.inner.gen::<f32>()
    }
    pub fn next_f64(&mut self) -> f64 {
        self.inner.gen::<f64>()
    }

    pub fn next_u32_range(&mut self, min: u32, max: u32) -> u32 {
        (self.next_f32() * (max - min) as f32) as u32 + min
    }
    pub fn next_u64_range(&mut self, min: u64, max: u64) -> u64 {
        (self.next_f64() * (max - min) as f64) as u64 + min
    }
    pub fn next_f32_range(&mut self, min: f32, max: f32) -> f32 {
        self.next_f32() * (max - min) + min
    }
    pub fn next_f64_range(&mut self, min: f64, max: f64) -> f64 {
        self.next_f64() * (max - min) + min
    }
}

impl FromWorld for Rng {
    fn from_world(world: &mut World) -> Self {
        let inner = match world.get_resource::<RootRng>() {
            Some(rng) => Xoshiro256StarStar::from_rng(rng.rng.clone())
                .expect("failed to create rng"),
            None => Xoshiro256StarStar::from_entropy(),
        };

        Self { inner }
    }
}
