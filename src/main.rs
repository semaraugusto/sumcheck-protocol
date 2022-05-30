pub mod poly;
pub mod prover;
pub mod verifier;
use ark_poly::polynomial::multivariate::{SparseTerm, Term};
use ark_poly::MVPolynomial;
use poly::{MultiLinearPolynomial as MLP, PolyEvaluation};
use prover::Prover;
use verifier::{Status, Verifier};

fn main() {
    let g_0: MLP = MLP::from_coefficients_vec(
        3,
        vec![
            (2u32.into(), SparseTerm::new(vec![(0, 3)])),
            (1u32.into(), SparseTerm::new(vec![(0, 1), (2, 1)])),
            (1u32.into(), SparseTerm::new(vec![(1, 1), (2, 1)])),
        ],
    );
    let g0_sum = g_0.slow_sum_poly();
    let mut prover = Prover::new(&g_0.clone());
    let s1 = prover.first_round();
    let mut verifier = Verifier::new(g_0.clone(), &s1, g0_sum);
    let mut status = Status::Verifying;
    while status == Status::Verifying {
        let r = verifier.gen_r();
        let gi = prover.gen_uni_polynomial(r);
        status = verifier.execute_round(&gi);
    }
    assert_eq!(status, Status::Verified);
}
