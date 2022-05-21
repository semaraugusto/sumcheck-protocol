use ark_bls12_381::Fr as ScalarField;
use ark_poly::polynomial::multivariate::{SparsePolynomial, SparseTerm};
use ark_poly::polynomial::{MVPolynomial, Polynomial};
use itertools::Itertools;

pub type MultiLinearPolynomial = SparsePolynomial<ScalarField, SparseTerm>;

pub fn n_to_vec(i: usize, n: usize) -> Vec<ScalarField> {
    let result = format!("{:0>width$}", format!("{:b}", i), width = n)
        .chars()
        .map(|x| if x == '1' { 1.into() } else { 0.into() })
        .collect();
    result
}

pub trait SumEvaluation {
    fn slow_sum_poly(&self) -> ScalarField;
    fn slow_sum_g(&self) -> ScalarField;
}

impl SumEvaluation for MultiLinearPolynomial {
    fn slow_sum_poly(&self) -> ScalarField {
        let v = self.num_vars();
        let n = 2u32.pow(v as u32);
        println!("n: {:?} -> v: {:?}", n, v);
        let test: ScalarField = (0..v)
            .map(|_| [false, true])
            .multi_cartesian_product()
            .fold(0u8.into(), |acc, w| {
                let w: Vec<ScalarField> = w.iter().map(|w_i| (*w_i).into()).collect();
                let result = self.evaluate(&w);
                acc + result
            });

        println!("FINAL_RES: {:?}", test);
        test
        // 1u32.into()
    }
    fn slow_sum_g(&self) -> ScalarField {
        let v = self.num_vars();
        let n = 2u32.pow(v as u32);
        let mut result: ScalarField = 0u8.into();
        let mut results = Vec::new();
        (0..n).for_each(|n| {
            let w = n_to_vec(n as usize, v);
            let res = self.evaluate(&w);
            println!("SLOW res: {:?}", res);
            // println!("w: {:?} -> res: {:?}", w, res);
            results.push(res);
            result += res;
        });

        result
    }
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
    #[case(&G_0, &G_0_SUM1, &G_0_SUM2)]
    #[case(&G_1, &G_1_SUM1, &G_1_SUM2)]
    fn test_poly_sum(
        #[case] p: &MultiLinearPolynomial,
        #[case] c1: &ScalarField,
        #[case] c2: &ScalarField,
    ) {
        println!("P: {:?}", p);
        println!("c1: {:?}", c1);
        println!("c2: {:?}", c2);
        assert_eq!(c1, c2);
    }
}
