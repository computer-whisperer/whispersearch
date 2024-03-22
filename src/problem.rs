

pub trait CandidateSolution  {
    fn mutate_candidate_solution(&mut self, rng: &mut impl nanorand::Rng<8>);
}

pub trait Problem {
    type CandidateSolution: CandidateSolution + Clone;
    fn evaluate(&self, data: &Self::CandidateSolution) -> f32;
}


pub struct HelloWorldProblem {
    pub key: String
}

impl Problem for HelloWorldProblem {
    type CandidateSolution = UTF8ArrayCandidateSolution;
    fn evaluate(&self, data: &Self::CandidateSolution) -> f32 {
        let mut score = 0f32;
        for i in 0..self.key.as_bytes().len() {
            let key_val = self.key.as_bytes()[i] as i32;
            let data_val = data.data[i] as i32;
            let local_score = 1f32 - (i32::abs(key_val - data_val) as f32);

            if local_score > 0f32 {
                score += local_score;
            }
        }
        score
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct UTF8ArrayCandidateSolution{
    data: [u8; 20]
}

impl CandidateSolution for UTF8ArrayCandidateSolution {
    fn mutate_candidate_solution(&mut self, rng: &mut impl nanorand::Rng<8>) {
        // Replace a random byte with a random byte
        self.data[rng.generate_range(0..self.data.len())] = rng.generate_range(0..=255);
    }
}
