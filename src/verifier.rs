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
    pub poly: MLP,
    pub r_vec: Vec<ScalarField>,
    pub prev_gi: Vec<ScalarField>,
    pub curr_gi: Vec<ScalarField>,
}

#[derive(PartialEq, Debug)]
pub enum Status {
    Verifying,
    Verified,
}

impl Verifier {
    pub fn new(poly: &MLP, s1: &UniPoly, claimed_eval: ScalarField) -> Self {
        let rand = thread_rng();
        let expected = s1.evaluate(&ScalarField::one()) + s1.evaluate(&ScalarField::zero());
        assert_eq!(claimed_eval, expected);
        Self {
            rand,
            poly: poly.clone(),
            r_vec: vec![],
            curr_gi: s1.to_vec(),
            prev_gi: s1.to_vec(),
        }
    }
    pub fn gen_r(&mut self) -> ScalarField {
        let r: ScalarField = self.rand.gen();
        self.r_vec.push(r);
        r
    }
    pub fn execute_round(&mut self, s: &[ScalarField]) -> Status {
        let prev_gi = self.curr_gi.clone();

        self.curr_gi = s.to_vec();
        self.prev_gi = prev_gi.clone();

        let round = self.r_vec.len();
        let max_rounds = self.poly.num_vars - 1;
        // Check if this is the last round
        match round {
            _r if _r == max_rounds => {
                let r = self.gen_r();
                let poly = UniPoly::from_coefficients_slice(s);
                let eval = poly.evaluate(&r);

                let full_eval = self.poly.evaluate(&self.r_vec);

                assert_eq!(full_eval, eval);
                Status::Verified
            }
            _ => {
                let r_prev = self.r_vec.last().unwrap();

                let prev_gi = UniPoly::from_coefficients_slice(&prev_gi);
                let eval = prev_gi.evaluate(r_prev);

                let poly = UniPoly::from_coefficients_slice(s);
                let expected =
                    poly.evaluate(&ScalarField::one()) + poly.evaluate(&ScalarField::zero());

                assert_eq!(eval, expected);
                Status::Verifying
            }
        }
    }
}
