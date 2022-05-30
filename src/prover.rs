#![allow(dead_code)]
use ark_bls12_381::Fr as ScalarField;
use ark_ff::Field;
use ark_poly::polynomial::multivariate::{SparsePolynomial as MPoly, SparseTerm};
use ark_poly::polynomial::univariate::DensePolynomial as UPoly;
// use ark_poly::Polynomial;
use ark_poly::UVPolynomial;
use ark_std::{One, Zero};
use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Prover {
    pub g: MPoly<ScalarField, SparseTerm>,
    pub r_vec: Vec<ScalarField>,
}

impl Prover {
    pub fn new(g: MPoly<ScalarField, SparseTerm>) -> Self {
        Self { g, r_vec: vec![] }
    }
    pub fn first_round(&mut self) -> UPoly<ScalarField> {
        // self.r_vec.push();
        gen_uni_polynomial(&self.g, &[None])
    }

    pub fn gen_uni_polynomial(&mut self, r: ScalarField) -> UPoly<ScalarField> {
        self.r_vec.push(r);
        let mut inputs: Vec<Option<ScalarField>> = self.r_vec.iter().map(|x| Some(*x)).collect();
        inputs.push(None);
        gen_uni_polynomial(&self.g, &inputs)
    }

    pub fn compute_round(&mut self, r: ScalarField) -> UPoly<ScalarField> {
        self.gen_uni_polynomial(r)
    }

    // pub fn compute_final_round(&mut self, r: ScalarField) {
    //     self.oracle.full_eval(r);
    // }
}
// Take variable values via Some(ScalarField) and solve return a Univariate Polynomial for the None variable
// i.e. [None, Some(2), Some(3)] will return a Poly in respect to x1 with x2 solved for 2 and x3 solved for 3
fn partial_eval(
    g: &MPoly<ScalarField, SparseTerm>,
    vals: &[Option<ScalarField>],
) -> UPoly<ScalarField> {
    g.terms
        .iter()
        .map(|(coef, term)| {
            let (coef, degree) =
                term.iter()
                    .fold((*coef, 0), |acc, (var, degree)| match vals[*var] {
                        Some(val) => (val.pow([(*degree) as u64]) * acc.0, acc.1),
                        None => (acc.0, *degree),
                    });
            let mut vec = vec![ScalarField::zero(); degree + 1];
            vec[degree] = coef;
            UPoly::from_coefficients_slice(&vec)
        })
        .fold(UPoly::zero(), |acc, poly| acc + poly)
}

// Take variables and use 0,1 combination for the vars not provided
pub fn gen_uni_polynomial(
    g: &MPoly<ScalarField, SparseTerm>,
    inputs: &[Option<ScalarField>],
) -> UPoly<ScalarField> {
    (0..g.num_vars - inputs.len())
        .map(|_| [ScalarField::zero(), ScalarField::one()])
        .multi_cartesian_product()
        .map(|x| {
            x.iter().fold(inputs.to_vec(), |mut acc, &var| {
                acc.push(Some(var));
                acc
            })
        })
        .fold(UPoly::zero(), |acc, vals| acc + partial_eval(g, &vals))
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
        let mut prover = Prover::new(p);
        let s1 = prover.first_round();
        let expected_c = s1.evaluate(&0u32.into()) + s1.evaluate(&1u32.into());
        // println!("expected_c: {:?}", expected_c);
        // println!("g0_sum: {:?}", c1);
        assert_eq!(expected_c, *c1);
    }
}
