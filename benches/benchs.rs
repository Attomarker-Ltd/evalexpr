#![feature(test)]

extern crate test;
extern crate rand;
extern crate rand_pcg;

use test::Bencher;
use rand_pcg::Pcg32;
use rand::{Rng, SeedableRng};
use rand::distributions::Uniform;
use rand::seq::SliceRandom;
use evalexpr::build_operator_tree;
use std::hint::black_box;

const BENCHMARK_LEN: usize = 100_000;

fn generate_expression<Gen: Rng>(len: usize, gen: &mut Gen) -> String {
    let int_distribution = Uniform::new_inclusive(-100, 100);
    let whitespaces = vec![" ", "", "", "  ", " \n", "       "];
    let operators = vec!["+", "-", "*", "/", "%", "^"];
    let mut result = String::new();
    result.push_str(&format!("{}", gen.sample(int_distribution)));

    while result.len() < len {
        result.push_str(whitespaces.choose(gen).unwrap());
        result.push_str(operators.choose(gen).unwrap());
        result.push_str(whitespaces.choose(gen).unwrap());
        result.push_str(&format!("{}", gen.sample(int_distribution)));
    }

    result
}

fn generate_expression_chain<Gen: Rng>(len: usize, gen: &mut Gen) -> String {
    let mut chain = generate_expression(10, gen);
    while chain.len() < len {
        chain.push_str("; ");
        chain.push_str(&generate_expression(10, gen));
    }
    chain
}

fn generate_small_expressions<Gen: Rng>(len: usize, gen: &mut Gen) -> Vec<String> {
    let mut result = Vec::new();
    let mut result_len = 0;
    while result_len < len {
        let expression = generate_expression(10, gen);
        result_len += expression.len();
        result.push(expression);
    }
    result
}

#[bench]
fn bench_parse_long_expression_chains(bencher: &mut Bencher) {
    let mut gen = Pcg32::seed_from_u64(0);
    let long_expression_chain = generate_expression_chain(BENCHMARK_LEN, &mut gen);

    bencher.iter(|| {
        build_operator_tree(&long_expression_chain).unwrap()
    });
}

#[bench]
fn bench_parse_deep_expression_trees(bencher: &mut Bencher) {
    let mut gen = Pcg32::seed_from_u64(15);
    let deep_expression_tree = generate_expression(BENCHMARK_LEN, &mut gen);

    bencher.iter(|| {
        build_operator_tree(&deep_expression_tree).unwrap()
    });
}

#[bench]
fn bench_parse_many_small_expressions(bencher: &mut Bencher) {
    let mut gen = Pcg32::seed_from_u64(33);
    let small_expressions = generate_small_expressions(BENCHMARK_LEN, &mut gen);

    bencher.iter(|| {
        for expression in &small_expressions {
            black_box(build_operator_tree(&expression).unwrap());
        }
    });
}