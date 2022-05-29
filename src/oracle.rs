#![allow(dead_code)]
use ark_bls12_381::Fr as ScalarField;
// use ark_ff::Field;
// use ark_poly::polynomial::multivariate::{SparsePolynomial, SparseTerm, Term};
// use ark_poly::polynomial::univariate::DensePolynomial as UPoly;
// use ark_poly::polynomial::{MVPolynomial, Polynomial};
// use ark_std::{cfg_into_iter, One, Zero};
// use itertools::Itertools;

// use ark_bls12_381::Fr;
use ark_ff::Zero;
use ark_ff::{Field, One};
use ark_poly::polynomial::multivariate::SparsePolynomial as MPoly;
use ark_poly::polynomial::multivariate::SparseTerm;
pub use ark_poly::polynomial::univariate::DensePolynomial as UPoly;
// use ark_poly::MVPolynomial;
use ark_poly::Polynomial;
use ark_poly::UVPolynomial;
use itertools::Itertools;

use crate::poly::{MultiLinearPolynomial, SumEvaluation};
// use poly;

// type MultiPoly = SparsePolynomial<ScalarField, SparseTerm>;
#[derive(Debug, Clone)]
pub struct Oracle {
    pub g: MultiLinearPolynomial,
    pub r_vec: Vec<Option<ScalarField>>,
}

impl Oracle {
    pub fn new(g: MultiLinearPolynomial) -> Self {
        let r_vec = vec![None];
        Self { g, r_vec }
    }

    pub fn slow_sum_g(&self) -> ScalarField {
        self.g.slow_sum_poly()
    }

    pub fn gen_uni_polynomial(&self) -> UPoly<ScalarField> {
        partial_hypercube_eval(&self.g, &self.r_vec)
    }

    pub fn push_r_vec(&mut self, r: Option<ScalarField>) {
        self.r_vec.push(r);
    }
    pub fn full_eval(&mut self, r: ScalarField) -> ScalarField {
        self.push_r_vec(Some(r));
        let r_vec: Vec<ScalarField> = self.r_vec.iter().map(|r| r.unwrap()).collect();
        self.g.evaluate(&r_vec)
    }
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

// Sum Polynomial for all 0,1 combinations
fn hypercube_eval(g: &MPoly<ScalarField, SparseTerm>) -> ScalarField {
    (0..g.num_vars)
        .map(|_| [ScalarField::zero(), ScalarField::one()])
        .multi_cartesian_product()
        .map(|x| g.evaluate(&x))
        .fold(ScalarField::zero(), |acc, i| acc + i)
}

// Take variables and use 0,1 combination for the vars not provided
fn partial_hypercube_eval(
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
    use ark_bls12_381::Fr as ScalarField;
    use ark_poly::polynomial::multivariate::{SparsePolynomial, SparseTerm, Term};
    use ark_poly::polynomial::MVPolynomial;
    use rstest::rstest;
    // use thaler::sumcheck;

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
    #[case(&G_0_SUM1, &G_0_SUM2)]
    #[case(&G_1_SUM1, &G_1_SUM2)]
    fn test_poly_sum(#[case] c1: &ScalarField, #[case] c2: &ScalarField) {
        println!("c1: {:?}", c1);
        println!("c2: {:?}", c2);
        assert_eq!(c1, c2);
    }
    #[rstest]
    #[case(&G_0, &G_0_SUM1, &G_0_SUM2)]
    #[case(&G_1, &G_1_SUM1, &G_1_SUM2)]
    fn test_univariate_poly(
        #[case] p: &MultiLinearPolynomial,
        #[case] c1: &ScalarField,
        #[case] c2: &ScalarField,
    ) {
        assert_eq!(c1, c2);
        let p = p.clone();
        let uni_polyother = partial_hypercube_eval(&p, &[None]);
        let oracle = Oracle::new(p);
        let uni_poly = oracle.gen_uni_polynomial();
        println!("uni_poly: {:?}", uni_poly);
        println!("uni_polyother: {:?}", uni_polyother);
        let r0 = uni_poly.evaluate(&0u32.into());
        let r1 = uni_poly.evaluate(&1u32.into());
        assert_eq!(r0 + r1, *c1);
    }

    #[rstest]
    #[case(&G_0, &G_0_SUM1, &G_0_SUM2)]
    #[case(&G_1, &G_1_SUM1, &G_1_SUM2)]
    fn test_univariate_poly_new(
        #[case] p: &MultiLinearPolynomial,
        #[case] c1: &ScalarField,
        #[case] c2: &ScalarField,
    ) {
        assert_eq!(c1, c2);
        let p = p.clone();
        let uni_poly = partial_hypercube_eval(&p, &[None]);
        // let uni_poly_old = p.gen_uni_polynomial();
        println!("uni_poly: {:?}", uni_poly);
        // println!("uni_poly_old: {:?}", uni_poly_old);
        let r0 = uni_poly.evaluate(&0u32.into());
        let r1 = uni_poly.evaluate(&1u32.into());
        assert_eq!(r0 + r1, *c1);
    }
}
