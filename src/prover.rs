use crate::poly::{MultiLinearPolynomial as MLP, PolyEvaluation, UniPoly};
use ark_bls12_381::Fr as ScalarField;

#[derive(Debug, Clone)]
pub struct Prover {
    pub poly: MLP,
    pub r_vec: Vec<ScalarField>,
}

impl Prover {
    pub fn new(poly: &MLP) -> Self {
        Self {
            poly: poly.clone(),
            r_vec: vec![],
        }
    }
    pub fn first_round(&mut self) -> UniPoly {
        self.poly.gen_uni_polynomial(&[None])
    }

    pub fn gen_uni_polynomial(&mut self, r: ScalarField) -> UniPoly {
        self.r_vec.push(r);
        let mut inputs: Vec<Option<ScalarField>> = self.r_vec.iter().map(|&x| Some(x)).collect();
        inputs.push(None);
        if inputs.len() == self.poly.num_vars {
            self.poly.partial_eval(&inputs)
        } else {
            self.poly.gen_uni_polynomial(&inputs)
        }
    }
}

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;

    use super::*;
    use crate::poly::{MultiLinearPolynomial as MLP, PolyEvaluation};
    use ark_bls12_381::Fr as ScalarField;
    use ark_poly::polynomial::multivariate::{SparseTerm, Term};
    use ark_poly::polynomial::MVPolynomial;
    use ark_poly::Polynomial;
    use ark_std::{One, Zero};
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
        static ref G_0_SUM2: ScalarField = G_0.slow_sum_g();
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
        static ref G_1_SUM2: ScalarField = G_1.slow_sum_g();
    }

    #[rstest]
    #[case(&G_0, &G_0_SUM1, &G_0_SUM2)]
    #[case(&G_1, &G_1_SUM1, &G_1_SUM2)]
    fn test_first_round_prover(
        #[case] p: &MLP,
        #[case] c1: &ScalarField,
        #[case] c2: &ScalarField,
    ) {
        assert_eq!(c1, c2);
        let p = p.clone();
        let mut prover = Prover::new(&p.clone());
        let s1 = prover.first_round();
        let expected_c = s1.evaluate(&ScalarField::zero()) + s1.evaluate(&ScalarField::one());
        assert_eq!(expected_c, *c1);
    }
}
