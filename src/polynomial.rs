use plonky2_field::goldilocks_field::GoldilocksField;
use plonky2_field::types::Field;


// multiply an existing polynomial by a linear factor (x-xj); building a polynomial by its roots
pub fn poly_mul_linear(
    polynomial: &[GoldilocksField], 
    root: GoldilocksField
) -> Vec<GoldilocksField> {
    let d: usize = polynomial.len(); // number of terms in the polynomial
    let mut result: Vec<GoldilocksField> = vec![GoldilocksField::ZERO; d + 1]; // 
    
    for k in 0..=d {
        let mut val: GoldilocksField = GoldilocksField::ZERO;
        if k >= 1 {
            val += polynomial[k - 1];
        }
        if k < d {
            val -= polynomial[k] * root;
        }
        result[k] = val;
    }

    result
}


// multiply a polynomial by a scalar (multiply each coefficient by the scalar)
pub fn poly_scale(polynomial: &[GoldilocksField], scalar: GoldilocksField) -> Vec<GoldilocksField> {
    polynomial.iter().map(|&c| c * scalar).collect()
}


// add two polynomials a and b together by adding coefficients on same degree terms (both univariate over x)
pub fn poly_add(a: &[GoldilocksField], b: &[GoldilocksField]) -> Vec<GoldilocksField> {
    let length: usize = a.len().max(b.len()); 
    let mut result: Vec<GoldilocksField> = vec![GoldilocksField::ZERO; length];
    
    for (i, &c) in a.iter().enumerate() {
        result[i] += c;
    }

    for (i, &c) in b.iter().enumerate() {
        result[i] += c;
    }

    result
}


// find the unique Lagrange Interpolation Polynomial that passes through the points
pub fn lagrange_interpolate(points: &[(GoldilocksField, GoldilocksField)]) -> Vec<GoldilocksField> {
    let n = points.len();
    // will store the coefficients of the Lagrange Interpolation Polynomial
    let mut result: Vec<GoldilocksField> = vec![GoldilocksField::ZERO; n]; 

    for i in 0..n {
        let (xi, yi) = points[i];
        // build numerator and demoninator 
        let mut numerator: Vec<GoldilocksField> = vec![GoldilocksField::ONE];
        let mut demoninator: GoldilocksField = GoldilocksField::ONE;

        // this for loop computes the Lagrange basis polynomial, need to loop since the basis polynomial is a product
        for j in 0..n {
            if i == j {
                continue;
            }
            let xj = points[j].0; // access the x-coordinate of the point
            numerator = poly_mul_linear(&numerator, xj); // compute the coefficient vector for the numerator of this Lagrange basis polynomial
            demoninator *= xi - xj; // compute/update the denominator
        }

        let scalar: GoldilocksField = yi * demoninator.inverse();
        let term: Vec<GoldilocksField> = poly_scale(&numerator, scalar);
        result = poly_add(&result, &term);
    }

    result
}


// compute modular exponentiation
pub fn mod_pow(g: u128, p: u128, exp: u128) -> u64 {
    let mut result = 1;
    let mut base = g;
    let mut exponent = exp;

    base %= p;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % p; 
        }
        base = (base * base) % p;
        exponent /= 2;
    }

    result as u64
}


// find an nth degree root of unity in the Goldilocks field where n is the number of values
pub fn find_nth_root_of_unity(n: u128, p: u128) -> u64 {
    let g: u128 = 7u128; // use the smallest generator in Goldilocks
    let exp: u128 = ((p - 1) / n) as u128;

    mod_pow(g, p, exp)
}


// evaluate a polynomial (given by a coefficient vector and basis), evaluate at the point x
pub fn poly_eval(polynomial: &[GoldilocksField], x: GoldilocksField) -> GoldilocksField {
    let mut result: GoldilocksField = GoldilocksField::ZERO;
    for &coeff in polynomial.iter().rev() { // reverse order of coefficients to evaluate using Horner's method
        result = result * x + coeff;
    }

    result
}