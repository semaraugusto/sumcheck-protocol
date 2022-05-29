#![allow(dead_code)]
use crate::oracle::{Oracle, UPoly};
use crate::poly::MultiLinearPolynomial;
use ark_bls12_381::Fr as ScalarField;
use ark_poly::{Polynomial, UVPolynomial};
use ark_std::{One, Zero};
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};

#[derive(Debug)]
struct Verifier {
    rand: ThreadRng,
    rnd_poly: Vec<Vec<ScalarField>>,
    oracle: Oracle,
    gi: UPoly<ScalarField>,
    h: ScalarField,
}

#[derive(PartialEq)]
pub enum Status {
    Verifying,
    Verified,
}

impl Verifier {
    pub fn new(oracle: Oracle, s1: &UPoly<ScalarField>, h: ScalarField) -> Self {
        let rand = thread_rng();
        // let poly = UPoly::from_coefficients_slice(s1);
        assert!(h == s1.evaluate(&ScalarField::one()) + s1.evaluate(&ScalarField::zero()));
        Self {
            rand,
            oracle,
            rnd_poly: vec![s1.to_vec()],
            gi: s1.clone(),
            h,
        }
    }
    pub fn gen_r(&mut self) -> ScalarField {
        let r: ScalarField = self.rand.gen();
        self.oracle.push_r_vec(Some(r));
        r
    }

    pub fn execute_round(&mut self, gi: UPoly<ScalarField>) -> Status {
        let r = self.gen_r();
        let round = self.oracle.r_vec.len();
        let expected = self.gi.evaluate(&r);
        let value = gi.evaluate(&ScalarField::zero()) + gi.evaluate(&ScalarField::one());
        assert_eq!(expected, value);

        if self.oracle.g.num_vars - 1 == round {
            Status::Verified
        } else {
            Status::Verifying
        }
    }
}
