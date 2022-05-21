#![allow(dead_code)]
use crate::poly::MultiLinearPolynomial;
use ark_bls12_381::Fr as ScalarField;
use ark_poly::polynomial::multivariate::{SparsePolynomial, SparseTerm};

#[derive(Debug, Clone)]
pub struct Prover {
    pub poly: MultiLinearPolynomial,
}

impl Prover {
    fn new(poly: SparsePolynomial<ScalarField, SparseTerm>) -> Self {
        Self { poly: poly.clone() }
    }
}
