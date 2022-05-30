#![allow(dead_code)]
// use crate::poly::MultiLinearPolynomial;
use ark_bls12_381::Fr as ScalarField;
use ark_poly::polynomial::univariate::DensePolynomial as UPoly;
use ark_poly::Polynomial;
// use ark_poly::UVPolynomial;
use ark_std::{One, Zero};
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Verifier {
    pub rand: ThreadRng,
    pub rnd_poly: Vec<Vec<ScalarField>>,
    pub gi: UPoly<ScalarField>,
    pub h: ScalarField,
}

#[derive(PartialEq, Debug)]
pub enum Status {
    Verifying,
    Verified,
}

impl Verifier {
    pub fn new(s1: &UPoly<ScalarField>, h: ScalarField) -> Self {
        let rand = thread_rng();
        // let poly = UPoly::from_coefficients_slice(s1);
        assert!(h == s1.evaluate(&ScalarField::one()) + s1.evaluate(&ScalarField::zero()));
        Self {
            rand,
            rnd_poly: vec![s1.to_vec()],
            gi: s1.clone(),
            h,
        }
    }
    pub fn gen_r(&mut self) -> ScalarField {
        let r: ScalarField = self.rand.gen();
        r
    }

    pub fn execute_round(
        &mut self,
        r: ScalarField,
        gi: UPoly<ScalarField>,
        r_vec: Vec<Option<ScalarField>>,
    ) -> Status {
        // let r = self.gen_r();
        let round = r_vec.len();

        // if self.oracle.g.num_vars - 1 == round {
        //     Status::Verified
        // } else {
        // let expected = self.gi.evaluate(&r);
        let expected = self.gi.evaluate(&r);
        println!("expected: {:?}", expected);
        let value = gi.evaluate(&ScalarField::zero()) + gi.evaluate(&ScalarField::one());
        println!("got: {:?}", value);
        assert_eq!(expected, value);
        self.gi = gi;
        Status::Verifying
        // }
    }
}
