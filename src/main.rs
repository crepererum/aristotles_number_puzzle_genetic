use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::collections::HashSet;

///     00  01  02
///   03  04  05  06
/// 07  08  09  10  11
///   12  13  14  15
///     16  17  18

const INDICES: &[&[usize]] = &[
    &[0, 1, 2],
    &[3, 4, 5, 6],
    &[7, 8, 9, 10, 11],
    &[12, 13, 14, 15],
    &[16, 17, 18],
    &[0, 3, 7],
    &[1, 4, 8, 12],
    &[2, 5, 9, 13, 16],
    &[6, 10, 14, 17],
    &[11, 15, 18],
    &[2, 6, 11],
    &[1, 5, 10, 15],
    &[0, 4, 9, 14, 18],
    &[3, 8, 13, 17],
    &[7, 12, 16],
];
const POPULATION_SIZE: usize = 10000;

type Candidate = Vec<u8>;

fn eval_single(candidate: &[u8]) -> i64 {
    let mut field_scores: Vec<i64> = (0..candidate.len()).map(|_| 0).collect();
    let mut overall_score = 0i64;
    for row in INDICES {
        let row_score = 38i64 - row.iter().map(|i| candidate[*i] as i64).sum::<i64>();
        for i in row.iter() {
            field_scores[*i] += row_score;
        }
        overall_score += row_score.abs();
    }
    overall_score + field_scores.iter().map(|x| x.abs()).sum::<i64>()
}

fn eval_all(population: &mut HashSet<Candidate>) -> Vec<(i64, Candidate)> {
    let mut result: Vec<(i64, Candidate)> = population
        .drain()
        .map(|candidate| (eval_single(&candidate), candidate))
        .collect();
    result.sort_by_key(|x| x.0);
    result
}

fn generate_new_candidate<R>(mut rng: &mut R) -> Candidate
where
    R: Rng,
{
    let mut candidate: Candidate = (1..20).collect();
    candidate.shuffle(&mut rng);
    candidate
}

fn generate_population<R>(mut rng: &mut R) -> HashSet<Candidate>
where
    R: Rng,
{
    let mut population: HashSet<Candidate> = HashSet::new();
    while population.len() < POPULATION_SIZE {
        population.insert(generate_new_candidate(&mut rng));
    }
    population
}

fn mutate_single<R>(candidate: &[u8], rng: &mut R) -> Candidate
where
    R: Rng,
{
    let mut candidate2 = candidate.to_vec();
    let n_mutations = rng.gen_range(1, candidate2.len());
    for _ in 0..n_mutations {
        let i1 = rng.gen_range(0, candidate2.len());
        let i2 = rng.gen_range(0, candidate2.len());

        // std::mem::swap does not work due to the borrow checker
        let tmp = candidate2[i1];
        candidate2[i1] = candidate2[i2];
        candidate2[i2] = tmp;
    }

    candidate2
}

fn mutate_population<R>(population: &mut HashSet<Candidate>, rng: &mut R)
where
    R: Rng,
{
    let mut vector: Vec<Candidate> = population.iter().cloned().collect();
    let len_at_start = vector.len();
    while population.len() < len_at_start * 2 {
        let i = rng.gen_range(0, vector.len());
        let new = mutate_single(&vector[i], rng);
        vector.push(new.clone());
        population.insert(new);
    }
}

fn select(mut population: HashSet<Candidate>) -> HashSet<Candidate> {
    let mut v = eval_all(&mut population);
    v.drain(..).take(POPULATION_SIZE).map(|x| x.1).collect()
}

fn report(population: &HashSet<Candidate>) {
    let ratings = eval_all(&mut population.clone());
    println!("min: {}", ratings.iter().map(|x| x.0).min().unwrap());
    println!("max: {}", ratings.iter().map(|x| x.0).max().unwrap());
    println!(
        "avg: {}",
        ratings.iter().map(|x| x.0).sum::<i64>() as f64 / ratings.len() as f64
    );
    println!("best: {:?}", ratings.iter().next().map(|x| &x.1).unwrap());
}

fn main() {
    println!("Hello, world!");
    let mut rng = thread_rng();

    let mut population = generate_population(&mut rng);
    loop {
        for _ in 0..100 {
            mutate_population(&mut population, &mut rng);
            population = select(population);
        }
        report(&population);
    }
}
