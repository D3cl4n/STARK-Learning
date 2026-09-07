use plonky2_field::goldilocks_field::GoldilocksField;
use plonky2_field::types::Field;
use crate::polynomial::poly_eval;


// generate n_large points to expand the evaluation domain once the low-degree polynomial is calculated
pub fn generate_blow_up_points(
    n_large: u128, 
    large_omega: GoldilocksField, 
    polynomial: &[GoldilocksField]
) -> Vec<(GoldilocksField, GoldilocksField)> {
    println!("[*] Evaluating recovered polynomial at all powers of the {}th root of unity", n_large);
    let mut result: Vec<(GoldilocksField, GoldilocksField)> = vec![(GoldilocksField::ZERO, GoldilocksField::ZERO); n_large as usize];

    // go over every power when i=0 use n_large as the exponent (will evaluate to 1)
    for i in 0..(n_large as usize) {
        let x_coord: GoldilocksField = large_omega.exp_u64(i as u64); // at i=0 we get one so no need to compute omega^{n_large}
        let y_coord: GoldilocksField = poly_eval(polynomial, x_coord);
        result[i] = (x_coord, y_coord);
    }

    assert_eq!(result.len(), n_large as usize);
    result
}