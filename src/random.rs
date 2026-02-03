use std::time;

use bevy::prelude::*;
use rand::Rng;
use rand::SeedableRng;
use rand::distr::Distribution;
use rand::distr::StandardUniform;
use rand::distr::uniform::SampleRange;
use rand::distr::uniform::SampleUniform;
use rand_chacha::ChaCha8Rng;

pub fn plugin(app: &mut App) {
    app.insert_resource(RandomSource::default());
}

// RandomSource is a resource that provides random number generation capabilities.
#[derive(Resource)]
pub struct RandomSource(ChaCha8Rng);

impl Default for RandomSource {
    // Creates a new RandomSource with a seed based on the current system time in nanoseconds.
    fn default() -> Self {
        Self::new(
            time::SystemTime::now()
                .duration_since(time::UNIX_EPOCH)
                .unwrap_or(time::Duration::from_secs(0))
                .as_nanos() as u64,
        )
    }
}

impl RandomSource {
    // Creates a new RandomSource with a specific seed.
    pub fn new(seed: u64) -> Self {
        Self(ChaCha8Rng::seed_from_u64(seed))
    }

    // Returns a random number in the passed range.
    pub fn range<T, R>(&mut self, range: R) -> T
    where
        T: SampleUniform,
        R: SampleRange<T>,
    {
        self.0.random_range(range)
    }

    // Returns a boolean with a n/d chance of being true.
    // Example usage: `ratio(1, 3)` has a 1/3 chance of returning true.
    pub fn ratio(&mut self, n: u32, d: u32) -> bool {
        self.0.random_ratio(n, d)
    }

    // Returns a random value from the passed vector.
    pub fn pick<T: Clone>(&mut self, vec: &Vec<T>) -> T {
        if vec.is_empty() {
            panic!("Cannot pick from an empty vector");
        }
        let index = self.0.random_range(0..vec.len());
        vec[index].clone()
    }

    pub fn color(&mut self) -> Color {
        Color::hsv(self.0.random_range(0.0..255.0), 1.0, 1.0)
    }

    pub fn random<T>(&mut self) -> T
    where
        StandardUniform: Distribution<T>,
    {
        self.0.random()
    }
}

#[cfg(test)]
mod tests {
    use crate::random::RandomSource;

    #[test]
    fn test_range() {
        let mut rng = RandomSource::new(12345);
        let mut min = 10;
        let mut max = 1;
        for _ in 0..100 {
            let result = rng.range(1..10);
            min = result.min(min);
            max = result.max(max);
            assert!(
                result >= 1 && result < 10,
                "Result {} is out of range",
                result
            );
        }
        assert!(
            min == 1 && max == 9,
            "Range should be between 1 and 9, got {} and {}",
            min,
            max
        );
    }

    #[test]
    fn test_ratio() {
        let mut rng = RandomSource::new(12345);
        let mut true_count = 0;
        let mut false_count = 0;
        for _ in 0..100 {
            let result = rng.ratio(1, 3);
            if result {
                true_count += 1;
            } else {
                false_count += 1;
            }
        }
        assert!(
            true_count < false_count,
            "Expected more true results, got {} true and {} false",
            true_count,
            false_count
        );
    }

    #[test]
    fn test_pick() {
        let mut rng = RandomSource::new(12345);
        let sample = vec![0, 1, 2, 3, 4];
        let mut results = vec![0; sample.len()];
        for _ in 0..100 {
            let result = rng.pick(&sample);
            results[result] += 1;
            assert!(
                sample.contains(&result),
                "Picked value {} not in {:?}",
                result,
                sample
            );
        }
        assert!(
            results.iter().all(|&count| count > 0),
            "Not all values were picked, results: {:?}",
            results
        );
    }
}
