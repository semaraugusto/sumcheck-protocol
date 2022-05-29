pub mod oracle;
pub mod poly;
pub mod prover;
pub mod verifier;
// use ark_ff::Zero;
// use ark_ff::{Field, One};
// use ark_poly::polynomial::multivariate::SparsePolynomial as MPoly;
use ark_poly::polynomial::multivariate::Term;
use ark_poly::polynomial::multivariate::{SparsePolynomial, SparseTerm};
pub use ark_poly::polynomial::univariate::DensePolynomial as UPoly;
use ark_poly::MVPolynomial;
use ark_poly::Polynomial;
use ark_poly::UVPolynomial;
// use itertools::Itertools;
use oracle::Oracle;
use poly::{MultiLinearPolynomial, SumEvaluation, UniPoly};
use prover::Prover;
// is tn alive
// Checking if tn is working
fn main() {
    let g_0: MultiLinearPolynomial = SparsePolynomial::from_coefficients_vec(
        3,
        vec![
            (2u32.into(), SparseTerm::new(vec![(0, 3)])),
            (1u32.into(), SparseTerm::new(vec![(0, 1), (2, 1)])),
            (1u32.into(), SparseTerm::new(vec![(1, 1), (2, 1)])),
        ],
    );
    let g0_sum = g_0.slow_sum_poly();
    let oracle = Oracle::new(g_0);
    let prover = Prover::new(oracle);
    let s1 = prover.first_round();
    let expected_c = s1.evaluate(&0u32.into()) + s1.evaluate(&1u32.into());
    println!("expected_c: {:?}", expected_c);
    println!("g0_sum: {:?}", g0_sum);
    assert_eq!(expected_c, g0_sum);
}
