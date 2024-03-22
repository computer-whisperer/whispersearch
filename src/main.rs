use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Instant;
use crate::problem::{CandidateSolution, Problem};

mod problem_travelling_salesman;
mod problem;


trait AcceptNeighborWhen {
    fn accept_neighbor(current_score: f32, neighbor_score: f32, progress: f32, rng: &mut impl nanorand::Rng<8>) -> bool;
}

struct SimulatedAnnealingAcceptNeighborWhen {}

impl AcceptNeighborWhen for SimulatedAnnealingAcceptNeighborWhen {
    fn accept_neighbor(current_score: f32, neighbor_score: f32, progress: f32, rng: &mut impl nanorand::Rng<8>) -> bool {
        if neighbor_score > current_score
        {
            true
        }
        else
        {
            let p = ((neighbor_score - current_score)*4000f32*progress).exp();
            rng.generate_range(0..1000) < (1000f32 * p) as i32
        }
    }
}

struct GreedyAcceptNeighborWhen {}

impl AcceptNeighborWhen for GreedyAcceptNeighborWhen {
    fn accept_neighbor(current_score: f32, neighbor_score: f32, _progress: f32, _rng: &mut impl nanorand::Rng<8>) -> bool {
        neighbor_score > current_score
    }
}

fn simulated_annealing<AcceptNeigborWhenT: AcceptNeighborWhen, ProblemT: problem::Problem>(
    problem: &ProblemT,
    initial_solution: ProblemT::CandidateSolution,
    iters: usize,
    rng: &mut impl nanorand::Rng<8>,
    mut debug_out: Option<&mut impl Write>
) -> ProblemT::CandidateSolution {
    let mut best_solution_so_far = initial_solution;
    let mut best_solution_so_far_score = problem.evaluate(&best_solution_so_far);

    for i in 0..iters {
        let mut new_solution = best_solution_so_far.clone();
        new_solution.mutate_candidate_solution(rng);
        let new_score = problem.evaluate(&new_solution);

        if AcceptNeigborWhenT::accept_neighbor(best_solution_so_far_score, new_score, (i as f32)/(iters as f32), rng) {
            best_solution_so_far = new_solution;
            best_solution_so_far_score = new_score;
        }

        if let Some(ref mut d) = debug_out {
            writeln!(d, "{best_solution_so_far_score}, {new_score}").unwrap();
        }
    }

    best_solution_so_far
}

fn main() {
    let problem = problem::HelloWorldProblem{key: String::from("Mystery")};
    //let problem = problem_travelling_salesman::TravellingSalesmanProblem::<10>::new_randomized(&mut nanorand::WyRand::new_seed(4819));
    let initial_solution = Default::default();

    println!("{:?}\t{}", initial_solution, problem.evaluate(&initial_solution));
    let mut rng = nanorand::WyRand::new_seed(4819);
    let start = Instant::now();
    let path = Path::new("debug_out.csv");
    let mut debug_out = File::create(&path).expect("Could not create file");
    writeln!(debug_out, "best_value, new_value").unwrap();
    let best_solution = simulated_annealing::<SimulatedAnnealingAcceptNeighborWhen, _>(&problem, initial_solution, 1000000, &mut rng, Some(&mut debug_out));
    println!("Time: {:?}", start.elapsed());
    println!("{:?}\t{}", best_solution, problem.evaluate(&best_solution));
}
