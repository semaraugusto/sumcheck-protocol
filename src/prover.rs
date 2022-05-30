#![allow(dead_code)]
use ark_bls12_381::Fr as ScalarField;
// use ark_poly::polynomial::multivariate::{SparsePolynomial, SparseTerm};
// use sumcheck::oracle;
use crate::oracle::{Oracle, UPoly};
// use ark_poly::Polynomial;

#[derive(Debug, Clone)]
pub struct Prover {
    pub oracle: Oracle,
}

impl Prover {
    pub fn new(oracle: Oracle) -> Self {
        Self { oracle }
    }
    pub fn first_round(&self) -> UPoly<ScalarField> {
        self.oracle.gen_uni_polynomial()
    }

    pub fn gen_uni_polynomial(&mut self, r: ScalarField) -> UPoly<ScalarField> {
        self.oracle.push_r_vec(Some(r));
        self.oracle.gen_uni_polynomial()
    }

    pub fn compute_round(&mut self, r: ScalarField) -> UPoly<ScalarField> {
        self.gen_uni_polynomial(r)
    }

    pub fn compute_final_round(&mut self, r: ScalarField) {
        self.oracle.full_eval(r);
    }
}
#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;

    use super::*;
    use crate::poly::{MultiLinearPolynomial, SumEvaluation};
    use ark_bls12_381::Fr as ScalarField;
    use ark_poly::polynomial::multivariate::{SparsePolynomial, SparseTerm, Term};
    use ark_poly::polynomial::MVPolynomial;
    use ark_poly::Polynomial;
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
    fn test_first_round_prover(
        #[case] p: &MultiLinearPolynomial,
        #[case] c1: &ScalarField,
        #[case] c2: &ScalarField,
    ) {
        assert_eq!(c1, c2);
        let p = p.clone();
        let oracle = Oracle::new(p);
        let prover = Prover::new(oracle);
        let s1 = prover.first_round();
        let expected_c = s1.evaluate(&0u32.into()) + s1.evaluate(&1u32.into());
        // println!("expected_c: {:?}", expected_c);
        // println!("g0_sum: {:?}", c1);
        assert_eq!(expected_c, *c1);
    }
}
