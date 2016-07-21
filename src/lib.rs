#![cfg_attr(test, feature(test))]

/// Implementation of Stochastic universal sampling
/// https://en.wikipedia.org/wiki/Stochastic_universal_sampling
/// Runtime: O(n)
/// Memory Usage: O(n) or O(1) (in place variant)

extern crate rand;

use self::rand::{thread_rng, Rng};

pub struct RandomChoice;


impl RandomChoice {
    /// Chooses n samples by their weights. The greater their weights the more likely they get chosen.
    ///
    /// @invariant sum of weights must not overflow.
    /// @param samples The to be selected samples
    /// @param weights Weights that get chosen by their weight/probability. One weight can be greater 1.
    /// @param n Number of randomly chosen samples by weight.
    /// @return randomly selected samples by their weights
    pub fn random_choice_f64<'a, T>(samples: &'a [T], weights: &[f64], n: usize) -> Vec<&'a T> {
        if weights.len() == 0 || n == 0 {
            return Vec::new();
        }

        let sum: f64 = weights.iter().fold(0.0, |acc, &i| acc + i);
        let spoke_gap: f64 = sum / n as f64;

        // next_f64() ∈ [0.0, 1.0)
        let spin = thread_rng().next_f64() * spoke_gap;

        let mut i: usize = 0;
        let mut accumulated_weights = weights[0];
        let mut choices: Vec<&T> = Vec::with_capacity(n);
        let mut current_spoke: f64 = spin;

        for _ in 0..n {
            while accumulated_weights < current_spoke {
                i += 1;
                accumulated_weights += weights[i];
            }
            choices.push(&samples[i]);
            current_spoke += spoke_gap;
        }

        choices
    }

    /// Chooses n samples by their weights. The greater their weights the more likely they get chosen.
    /// The result gets saved directly in the samples argument.
    /// @invariant sum of weights must not overflow.
    /// @param samples The to be selected samples
    /// @param weights Weights that get chosen by their weight/probability. One weight can be greater 1.
    pub fn random_choice_in_place_f64<T: Clone>(samples: &mut [T], weights: &[f64]) {
        if weights.len() < 2 {
            return;
        }

        let sum: f64 = weights.iter().fold(0.0, |acc, &i| acc + i);
        let n: usize = weights.len();
        let spoke_gap: f64 = sum / n as f64;

        // next_f64() ∈ [0.0, 1.0)
        let spin = thread_rng().next_f64() * spoke_gap;

        let mut j: usize = 0;
        let mut accumulated_weights = weights[0];
        let mut current_spoke: f64 = spin;

        for i in 0..n {
            while accumulated_weights < current_spoke {
                j += 1;
                accumulated_weights += weights[j];
            }
            samples[i] = samples[j].clone();
            current_spoke += spoke_gap;
        }
    }

    pub fn random_choice_f32<'a, T>(samples: &'a [T], weights: &[f32], n: usize) -> Vec<&'a T> {
        if weights.len() == 0 || n == 0 {
            return Vec::new();
        }

        let sum: f32 = weights.iter().fold(0.0, |acc, &i| acc + i);
        let spoke_gap: f32 = sum / n as f32;

        // next_f32() ∈ [0.0, 1.0)
        let spin = thread_rng().next_f32() * spoke_gap;

        let mut i: usize = 0;
        let mut accumulated_weights = weights[0];
        let mut choices: Vec<&T> = Vec::with_capacity(n);
        let mut current_spoke: f32 = spin;

        for _ in 0..n {
            while accumulated_weights < current_spoke {
                i += 1;
                accumulated_weights += weights[i];
            }
            choices.push(&samples[i]);
            current_spoke += spoke_gap;
        }

        choices
    }

    pub fn random_choice_in_place_f32<T: Clone>(samples: &mut [T], weights: &[f32]) {
        if weights.len() < 2 {
            return;
        }

        let sum: f32 = weights.iter().fold(0.0, |acc, &i| acc + i);
        let n: usize = weights.len();
        let spoke_gap: f32 = sum / n as f32;

        // next_f32() ∈ [0.0, 1.0)
        let spin = thread_rng().next_f32() * spoke_gap;

        let mut j: usize = 0;
        let mut accumulated_weights = weights[0];
        let mut current_spoke: f32 = spin;

        for i in 0..n {
            while accumulated_weights < current_spoke {
                j += 1;
                accumulated_weights += weights[j];
            }
            samples[i] = samples[j].clone();
            current_spoke += spoke_gap;
        }
    }
}


#[cfg(test)]
mod benches {
    
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn bench_random_choice_64(b: &mut Bencher) {
        let capacity: usize = 500;
        let mut samples: Vec<f64> = Vec::with_capacity(capacity);
        let mut weights: Vec<f64> = Vec::with_capacity(capacity);

        for i in 0..capacity {
            samples.push((i + 1usize) as f64);
            weights.push((i + 1usize) as f64);
        }
        b.iter(|| {
            super::RandomChoice::random_choice_f64(&samples, &weights, 1200 as usize);
        });
    }

    #[bench]
    fn bench_random_choice_in_place_64(b: &mut Bencher) {
        let capacity: usize = 500;
        let mut samples: Vec<f64> = Vec::with_capacity(capacity);
        let mut weights: Vec<f64> = Vec::with_capacity(capacity);

        for i in 0..capacity {
            samples.push((i + 1usize) as f64);
            weights.push((i + 1usize) as f64);
        }
        b.iter(|| {
            super::RandomChoice::random_choice_in_place_f64(&mut samples, &weights);
        });
    }

    #[bench]
    fn bench_random_choice_32(b: &mut Bencher) {
        let capacity: usize = 500;
        let mut samples: Vec<f32> = Vec::with_capacity(capacity);
        let mut weights: Vec<f32> = Vec::with_capacity(capacity);

        for i in 0..capacity {
            samples.push((i + 1usize) as f32);
            weights.push((i + 1usize) as f32);
        }
        b.iter(|| {
            super::RandomChoice::random_choice_f32(&samples, &weights, 1200 as usize);
        });
    }

    #[bench]
    fn bench_random_choice_in_place_32(b: &mut Bencher) {
        let capacity: usize = 500;
        let mut samples: Vec<f32> = Vec::with_capacity(capacity);
        let mut weights: Vec<f32> = Vec::with_capacity(capacity);

        for i in 0..capacity {
            samples.push((i + 1usize) as f32);
            weights.push((i + 1usize) as f32);
        }
        b.iter(|| {
            super::RandomChoice::random_choice_in_place_f32(&mut samples, &weights);
        });
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_random_choice_64() {
        let capacity: usize = 1000;
        let mut samples: Vec<usize> = Vec::with_capacity(capacity);
        let mut weights: Vec<f64> = Vec::with_capacity(capacity);

        for i in 0..capacity {
            samples.push(i + 1);
            weights.push((i + 1usize) as f64);
        }

        let choices = super::RandomChoice::random_choice_f64(&samples, &weights, 4 as usize);

        let mut weight_counter = HashMap::with_capacity(capacity);

        for choice in choices {
            let counter = weight_counter.entry(choice).or_insert(0);
            *counter += 1;
        }

        for key in weight_counter.keys() {
            println!("{}", key);
        }
    }

    #[test]
    fn test_random_choice_in_place_64() {
        let mut samples = vec!["hi", "this", "is", "a", "test!"];
        let weights: Vec<f64> = vec![1.0, 1.0, 1.0, 1.0, 1.0];

        super::RandomChoice::random_choice_in_place_f64(&mut samples, &weights);

        for sample in samples {
            print!("{}, ", sample);
        }
    }

}