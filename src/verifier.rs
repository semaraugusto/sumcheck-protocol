use crate::poly::{MultiLinearPolynomial as MLP, UniPoly};
use ark_bls12_381::Fr as ScalarField;
use ark_poly::Polynomial;
use ark_poly::UVPolynomial;
use ark_std::{One, Zero};
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Verifier {
    pub rand: ThreadRng,
    pub rnd_poly: Vec<Vec<ScalarField>>,
    pub poly: MLP,
    pub r_vec: Vec<ScalarField>,
}

#[derive(PartialEq, Debug)]
pub enum Status {
    Verifying,
    Verified,
}

impl Verifier {
    pub fn new(poly: MLP, s1: &UniPoly, claimed_eval: ScalarField) -> Self {
        let rand = thread_rng();
        // let poly = UniPoly::from_coefficients_slice(s1);
        let expected = s1.evaluate(&ScalarField::one()) + s1.evaluate(&ScalarField::zero());
        assert_eq!(claimed_eval, expected);
        Self {
            rand,
            rnd_poly: vec![s1.to_vec()],
            poly: poly.clone(),
            r_vec: vec![],
        }
    }
    pub fn gen_r(&mut self) -> ScalarField {
        let r: ScalarField = self.rand.gen();
        self.r_vec.push(r);
        r
    }
    pub fn execute_round(&mut self, s: &[ScalarField]) -> Status {
        let s_prev = self.rnd_poly.last().unwrap().clone();
        self.rnd_poly.push(s.to_vec());
        let round = self.r_vec.len();

        // Determine if this is the last round
        if round == self.poly.num_vars - 1 {
            let r = self.gen_r();
            let poly = UniPoly::from_coefficients_slice(s);
            let expected = poly.evaluate(&r);

            let full_eval = self.poly.evaluate(&self.r_vec);

            assert_eq!(full_eval, expected);

            Status::Verified
        } else {
            let r_prev = self.r_vec.last().unwrap();
            let eval = UniPoly::from_coefficients_slice(&s_prev).evaluate(r_prev);
            let poly = UniPoly::from_coefficients_slice(s);

            let expected = poly.evaluate(&ScalarField::one()) + poly.evaluate(&ScalarField::zero());

            assert_eq!(eval, expected);

            Status::Verifying
        }
    }
}
