use fraction::{Fraction};
use std::io;

fn main() {

    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let d: usize = input.trim().parse().unwrap();

    let mut equations = vec![vec![0.into(); d+1]; d];
    for equation in &mut equations {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        *equation = input.trim().split(' ').map(|x| x.parse::<i64>().unwrap().into()).collect();
    }

    match solve(equations) {
        GaussianEliminationSolution::Singular(answer) => {
            println!("J");
            for val in answer {
                print!("{} ", val);
            }
            println!();
        }
        GaussianEliminationSolution::Partial(answer) => {
            println!("P");
            for i in 0..answer[0].len() {
                for j in 0..answer.len() {
                    print!("{} ", answer[j][i])
                }
                println!();
            }
        }
        _ => {
            println!("N");
        }
    }
}

#[derive(Debug)]
pub enum GaussianEliminationSolution {
    Singular(Vec<Fraction>),
    Partial(Vec<Vec<Fraction>>),
    Impossible,
}

pub fn solve(mut eqs: Vec<Vec<Fraction>>) -> GaussianEliminationSolution {
    let d = eqs.len();
    let mut used_eqs = vec![];

    for i in 0..d {
        let Some(pos) = eqs.iter().position(|eq| eq[i] != 0.into()) else {
            continue;
        };
        let sub_eq = eqs.remove(pos);
        for eq in &mut eqs {
            let mp = eq[i] / sub_eq[i];
            for j in i..(d + 1) {
                eq[j] -= sub_eq[j] * mp;
            }
        }
        used_eqs.push(sub_eq);
    }

    if eqs.is_empty() {
        let mut answer = vec![0.into(); d];
        for (i, eq) in used_eqs.iter().enumerate().rev() {
            let mut sum = eq[d];
            for j in i + 1..d {
                sum -= answer[j] * eq[j];
            }
            answer[i] = sum / eq[i];
        }
        GaussianEliminationSolution::Singular(answer)
    } else if eqs.iter().all(|eq| eq[d] == 0.into()) {
        let mut answer: Vec<Vec<Fraction>> = vec![vec![0.into(); d + 1]; d];
        for i in 0..eqs.len() {
            let j = (d) - i;
            answer[j - 1][j] = 1.into();
        }
        for (i, eq) in used_eqs.iter().enumerate().rev() {
            let mut sum = vec![0.into(); d + 1];
            sum[0] = eq[d];
            for j in i + 1..d {
                for k in 0..d + 1 {
                    sum[k] -= answer[j][k] * eq[j];
                }
            }
            answer[i] = sum.iter().map(|&x| x / eq[i]).collect();
        }
        GaussianEliminationSolution::Partial(answer)
    } else {
        GaussianEliminationSolution::Impossible
    }
}
