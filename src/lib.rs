pub mod poly;
pub mod prover;
pub mod verifier;

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;

    use crate::poly::{MultiLinearPolynomial as MLP, PolyEvaluation};
    use crate::prover::Prover;
    use crate::verifier::{Status, Verifier};
    use ark_bls12_381::Fr as ScalarField;
    use ark_poly::polynomial::multivariate::{SparseTerm, Term};
    use ark_poly::polynomial::MVPolynomial;
    use rand::{thread_rng, Rng};
    use rstest::rstest;

    lazy_static! {
        // g = 2(x_1)^3 + (x_1)(x_3) + (x_2)(x_3)
        static ref G_0: MLP = MLP::from_coefficients_vec(
            3,
            vec![
                (2u32.into(), SparseTerm::new(vec![(0, 3)])),
                (1u32.into(), SparseTerm::new(vec![(0, 1), (2, 1)])),
                (1u32.into(), SparseTerm::new(vec![(1, 1), (2, 1)])),
            ],
        );
        static ref G_0_SUM1: ScalarField = G_0.slow_sum_poly();
        // Test with a larger g
        static ref G_1: MLP = MLP::from_coefficients_vec(
            4,
            vec![
                (2u32.into(), SparseTerm::new(vec![(0, 3)])),
                (1u32.into(), SparseTerm::new(vec![(0, 1), (2, 1)])),
                (1u32.into(), SparseTerm::new(vec![(1, 1), (2, 1)])),
                (1u32.into(), SparseTerm::new(vec![(3, 1), (2, 1)])),
            ],
        );
        static ref G_1_SUM1: ScalarField = G_1.slow_sum_poly();
    }

    #[rstest]
    #[case(&G_0, &G_0_SUM1)]
    #[case(&G_1, &G_1_SUM1)]
    fn test_first_round(#[case] p: &MLP, #[case] c1: &ScalarField) {
        let mut prover = Prover::new(&p.clone());
        let s1 = prover.first_round();
        let _ = Verifier::new(p, &s1, *c1);
    }
    #[rstest]
    #[case(&G_0, &G_0_SUM1)]
    #[case(&G_1, &G_1_SUM1)]
    #[should_panic]
    fn should_fail_first_round(#[case] p: &MLP, #[case] c1: &ScalarField) {
        let mut prover = Prover::new(&p.clone());
        let mut rand = thread_rng();
        let r: ScalarField = rand.gen();
        let _ = prover.first_round();
        let s2 = prover.gen_round_polynomial(r);
        let _ = Verifier::new(p, &s2, *c1);
    }
    #[rstest]
    #[case(&G_0, &G_0_SUM1)]
    #[case(&G_1, &G_1_SUM1)]
    fn test_second_round(#[case] p: &MLP, #[case] c1: &ScalarField) {
        let mut prover = Prover::new(&p.clone());
        let s1 = prover.first_round();
        let mut verifier = Verifier::new(p, &s1, *c1);
        let r = verifier.gen_r();
        let gi = prover.gen_round_polynomial(r);
        let status = verifier.execute_round(&gi);
        assert_eq!(status, Status::Verifying);
    }

    #[rstest]
    #[case(&G_0, &G_0_SUM1)]
    #[case(&G_1, &G_1_SUM1)]
    fn test_protocol(#[case] p: &MLP, #[case] c1: &ScalarField) {
        let mut prover = Prover::new(&p.clone());
        let s1 = prover.first_round();
        let mut verifier = Verifier::new(p, &s1, *c1);
        let mut status = Status::Verifying;
        while status == Status::Verifying {
            let r = verifier.gen_r();
            let gi = prover.gen_round_polynomial(r);
            status = verifier.execute_round(&gi);
        }
        assert_eq!(status, Status::Verified);
    }
}
