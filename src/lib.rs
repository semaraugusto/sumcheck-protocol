pub mod poly;
pub use poly::*;

pub mod oracle;
pub use oracle::*;

pub mod prover;
pub use prover::*;

pub mod verifier;
pub use verifier::*;

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;

    // use super::*;
    use crate::oracle::Oracle;
    use crate::poly::{MultiLinearPolynomial, SumEvaluation};
    use crate::prover::Prover;
    use crate::verifier::Verifier;
    use ark_bls12_381::Fr as ScalarField;
    use ark_poly::polynomial::multivariate::{SparsePolynomial, SparseTerm, Term};
    use ark_poly::polynomial::MVPolynomial;
    // use ark_poly::Polynomial;
    use rand::{thread_rng, Rng};
    use rstest::rstest;

    lazy_static! {
        // g = 2(x_1)^3 + (x_1)(x_3) + (x_2)(x_3)
        static ref G_0: MultiLinearPolynomial = SparsePolynomial::from_coefficients_vec(
            3,
            vec![
                (2u32.into(), SparseTerm::new(vec![(0, 3)])),
                (1u32.into(), SparseTerm::new(vec![(0, 1), (2, 1)])),
                (1u32.into(), SparseTerm::new(vec![(1, 1), (2, 1)])),
            ],
        );
        static ref G_0_SUM1: ScalarField = G_0.slow_sum_poly();
        static ref G_0_SUM2: ScalarField = G_0.slow_sum_g();
        // Test with a larger g
        static ref G_1: MultiLinearPolynomial = SparsePolynomial::from_coefficients_vec(
            4,
            vec![
                (2u32.into(), SparseTerm::new(vec![(0, 3)])),
                (1u32.into(), SparseTerm::new(vec![(0, 1), (2, 1)])),
                (1u32.into(), SparseTerm::new(vec![(1, 1), (2, 1)])),
                (1u32.into(), SparseTerm::new(vec![(3, 1), (2, 1)])),
            ],
        );
        static ref G_1_SUM1: ScalarField = G_1.slow_sum_poly();
        static ref G_1_SUM2: ScalarField = G_1.slow_sum_g();
    }

    #[rstest]
    #[case(&G_0, &G_0_SUM1, &G_0_SUM2)]
    #[case(&G_1, &G_1_SUM1, &G_1_SUM2)]
    fn test_first_round(
        #[case] p: &MultiLinearPolynomial,
        #[case] c1: &ScalarField,
        #[case] c2: &ScalarField,
    ) {
        assert_eq!(c1, c2);
        let p = p.clone();
        let oracle = Oracle::new(p);
        let prover = Prover::new(oracle.clone());
        let s1 = prover.first_round();
        let _ = Verifier::new(oracle, &s1, *c1);
    }
    #[rstest]
    #[case(&G_0, &G_0_SUM1, &G_0_SUM2)]
    #[case(&G_1, &G_1_SUM1, &G_1_SUM2)]
    #[should_panic]
    fn should_fail_first_round(
        #[case] p: &MultiLinearPolynomial,
        #[case] c1: &ScalarField,
        #[case] c2: &ScalarField,
    ) {
        assert_eq!(c1, c2);
        let p = p.clone();
        let oracle = Oracle::new(p);
        let mut prover = Prover::new(oracle.clone());
        let mut rand = thread_rng();
        let r: ScalarField = rand.gen();
        let _ = prover.first_round();
        let s2 = prover.compute_round(r);
        let _ = Verifier::new(oracle, &s2, *c1);
    }
}
