use ark_bls12_381::Fr as ScalarField;
use ark_ff::Field;
use ark_poly::polynomial::multivariate::{SparsePolynomial as MPoly, SparseTerm};
use ark_poly::polynomial::univariate::DensePolynomial as UPoly;
use ark_poly::Polynomial;
use ark_poly::UVPolynomial;
use ark_std::{One, Zero};
use itertools::Itertools;

pub type MultiLinearPolynomial = MPoly<ScalarField, SparseTerm>;
pub type UniPoly = UPoly<ScalarField>;

pub trait PolyEvaluation {
    fn slow_sum_poly(&self) -> ScalarField;
    fn partial_eval(&self, vals: &[Option<ScalarField>]) -> UPoly<ScalarField>;
    fn gen_uni_polynomial(&self, inputs: &[Option<ScalarField>]) -> UniPoly;
}

impl PolyEvaluation for MultiLinearPolynomial {
    fn slow_sum_poly(&self) -> ScalarField {
        (0..self.num_vars)
            .map(|_| [ScalarField::zero(), ScalarField::one()])
            .multi_cartesian_product()
            .map(|x| self.evaluate(&x))
            .fold(ScalarField::zero(), |acc, i| acc + i)
    }
    fn partial_eval(&self, vals: &[Option<ScalarField>]) -> UPoly<ScalarField> {
        self.terms
            .iter()
            .map(|(coef, term)| {
                let (coef, degree) = term.iter().fold((*coef, 0), |acc, (var, degree)| match vals
                    [*var]
                {
                    Some(val) => (val.pow([(*degree) as u64]) * acc.0, acc.1),
                    None => (acc.0, *degree),
                });
                let mut vec = vec![ScalarField::zero(); degree + 1];
                vec[degree] = coef;
                UPoly::from_coefficients_slice(&vec)
            })
            .fold(UPoly::zero(), |acc, poly| acc + poly)
    }

    fn gen_uni_polynomial(&self, inputs: &[Option<ScalarField>]) -> UPoly<ScalarField> {
        (0..self.num_vars - inputs.len())
            .map(|_| [ScalarField::zero(), ScalarField::one()])
            .multi_cartesian_product()
            .map(|x| {
                x.iter().fold(inputs.to_vec(), |mut acc, &var| {
                    acc.push(Some(var));
                    acc
                })
            })
            .fold(UPoly::zero(), |acc, vals| acc + self.partial_eval(&vals))
    }
}

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;

    use super::MultiLinearPolynomial as MLP;
    use super::*;
    use ark_bls12_381::Fr as ScalarField;
    use ark_poly::polynomial::multivariate::{SparseTerm, Term};
    use ark_poly::polynomial::MVPolynomial;
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
    }

    #[rstest]
    #[case(&G_0, &G_0_SUM1)]
    #[case(&G_1, &G_1_SUM1)]
    fn test_univariate_poly(#[case] p: &MLP, #[case] c1: &ScalarField) {
        let uni_poly = p.gen_uni_polynomial(&[None]);
        let r0 = uni_poly.evaluate(&ScalarField::zero());
        let r1 = uni_poly.evaluate(&ScalarField::one());
        assert_eq!(r0 + r1, *c1);
    }
}
