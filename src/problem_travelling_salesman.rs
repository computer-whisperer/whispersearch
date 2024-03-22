use nanorand::Rng;
use crate::problem;

#[derive(Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Copy, Clone, Debug)]
pub struct TravellingSalesmanProblemCandidateSolution <const LENGTH: usize> {
    city_order: [u32; LENGTH]
}

impl <const LENGTH: usize> Default for TravellingSalesmanProblemCandidateSolution<LENGTH> {
    fn default() -> Self {
        let mut order = [0u32; LENGTH];
        for i in 0..LENGTH {
            order[i] = i as u32;
        }
        Self {
            city_order: order
        }
    }
}

impl <const LENGTH: usize> problem::CandidateSolution for TravellingSalesmanProblemCandidateSolution <LENGTH> {
    fn mutate_candidate_solution(&mut self, rng: &mut impl nanorand::Rng<8>) {
        // Swap two cities at random in order
        let a = rng.generate_range(0..LENGTH);
        let b = rng.generate_range(0..LENGTH);
        self.city_order.swap(a, b);
    }
}

pub(crate) struct TravellingSalesmanProblem <const LENGTH: usize> {
    cities: [Point; LENGTH],

}

impl<const LENGTH: usize> TravellingSalesmanProblem<LENGTH> {
    pub(crate) fn new_randomized(rng: &mut impl Rng<8>) -> Self {
        let mut cities = [Point { x: 0, y: 0 }; LENGTH];
        for i in 0..LENGTH {
            cities[i].x = rng.generate_range(0..1000);
            cities[i].y = rng.generate_range(0..1000);
        }
        Self {
            cities
        }
    }
}

impl<const LENGTH: usize> problem::Problem for TravellingSalesmanProblem<LENGTH> {
    type CandidateSolution = TravellingSalesmanProblemCandidateSolution<LENGTH>;

    fn evaluate(&self, data: &Self::CandidateSolution) -> f32 {
        let mut score = 0f32;

        for i in 1..LENGTH {
            let prev_city = &self.cities[data.city_order[i-1] as usize];
            let city = &self.cities[data.city_order[i] as usize];
            let dist = (prev_city.x - city.x).pow(2) + (prev_city.y - city.y).pow(2);
            score -= (dist as f32).sqrt();
        }

        score
    }
}





