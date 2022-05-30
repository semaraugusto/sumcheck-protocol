pub mod poly;
pub use poly::*;

pub mod prover;
pub use prover::*;

pub mod verifier;
pub use verifier::*;

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;

    // use super::*;
    use crate::poly::{MultiLinearPolynomial, SumEvaluation};
    use crate::prover::Prover;
    use crate::verifier::{Status, Verifier};
    use ark_bls12_381::Fr as ScalarField;
    use ark_poly::polynomial::multivariate::{SparsePolynomial, SparseTerm, Term};
    use ark_poly::polynomial::univariate::DensePolynomial as UPoly;
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
    }

    #[rstest]
    #[case(&G_0, &G_0_SUM1)]
    #[case(&G_1, &G_1_SUM1)]
    fn test_first_round(#[case] p: &MultiLinearPolynomial, #[case] c1: &ScalarField) {
        let p = p.clone();
        let mut prover = Prover::new(&p.clone());
        let s1 = prover.first_round();
        let _ = Verifier::new(p.clone(), &s1, *c1);
    }
    #[rstest]
    #[case(&G_0, &G_0_SUM1)]
    #[case(&G_1, &G_1_SUM1)]
    #[should_panic]
    fn should_fail_first_round(#[case] p: &MultiLinearPolynomial, #[case] c1: &ScalarField) {
        let p = p.clone();
        let mut prover = Prover::new(&p.clone());
        let mut rand = thread_rng();
        let r: ScalarField = rand.gen();
        let _ = prover.first_round();
        let s2 = prover.gen_uni_polynomial(r);
        let _ = Verifier::new(p.clone(), &s2, *c1);
    }
    #[rstest]
    #[case(&G_0, &G_0_SUM1)]
    #[case(&G_1, &G_1_SUM1)]
    fn test_second_round(#[case] p: &MultiLinearPolynomial, #[case] c1: &ScalarField) {
        let p = p.clone();
        let mut prover = Prover::new(&p.clone());
        let s1 = prover.first_round();
        // let s1 = prover.gen_uni_polynomial();
        let mut verifier = Verifier::new(p.clone(), &s1, *c1);
        let r = verifier.gen_r();
        let gi = prover.gen_uni_polynomial(r);
        let status = verifier.execute_round(&gi);
        assert_eq!(status, Status::Verifying);
    }

    #[rstest]
    #[case(&G_0, &G_0_SUM1)]
    fn test_third_round(#[case] p: &MultiLinearPolynomial, #[case] c1: &ScalarField) {
        let p = p.clone();
        let mut prover = Prover::new(&p.clone());
        let s1 = prover.first_round();
        // let s1 = prover.gen_uni_polynomial();
        let mut verifier = Verifier::new(p.clone(), &s1, *c1);
        let r = verifier.gen_r();
        let gi = prover.gen_uni_polynomial(r);
        let status = verifier.execute_round(&gi);
        assert_eq!(status, Status::Verifying);
        let r = verifier.gen_r();
        let gi = prover.gen_uni_polynomial(r);
        let status = verifier.execute_round(&gi);
        assert_eq!(status, Status::Verified);
    }

    #[rstest]
    #[case(&G_1, &G_1_SUM1)]
    fn test_fourth_round(#[case] p: &MultiLinearPolynomial, #[case] c1: &ScalarField) {
        let p = p.clone();
        let mut prover = Prover::new(&p.clone());
        let s1 = prover.first_round();
        // let s1 = prover.gen_uni_polynomial();
        let mut verifier = Verifier::new(p.clone(), &s1, *c1);
        let r = verifier.gen_r();
        let gi = prover.gen_uni_polynomial(r);
        let status = verifier.execute_round(&gi);
        assert_eq!(status, Status::Verifying);
        let r = verifier.gen_r();
        let gi = prover.gen_uni_polynomial(r);
        let status = verifier.execute_round(&gi);
        assert_eq!(status, Status::Verifying);
        let r = verifier.gen_r();
        let gi = prover.gen_uni_polynomial(r);
        let status = verifier.execute_round(&gi);
        assert_eq!(status, Status::Verified);
    }
}
