use plonky2_field::goldilocks_field::GoldilocksField;
use plonky2_field::types::Field;


// multiply an existing polynomial by a linear factor (x-xj); building a polynomial by its roots
fn poly_mul_linear(
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
fn poly_scale(polynomial: &[GoldilocksField], scalar: GoldilocksField) -> Vec<GoldilocksField> {
    polynomial.iter().map(|&c| c * scalar).collect()
}


// add two polynomials a and b together by adding coefficients on same degree terms (both univariate over x)
fn poly_add(a: &[GoldilocksField], b: &[GoldilocksField]) -> Vec<GoldilocksField> {
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
fn lagrange_interpolate(points: &[(GoldilocksField, GoldilocksField)]) -> Vec<GoldilocksField> {
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
/*
* The multiplicative group of a finite field with prime modulus P has P-1 elements since 0 is excluded
* The powers of a primitive element or generator e.g. g=7 from g^{0} to g^{P-2} give all elements of the group
* This function computes g^{P-1} / n which gives an nth primitive root of unity modulo p
* This works because an nth root of unity x needs x^{n} = 1 (mod p) 
* We set x = g^{P-1} / n and then x^{n} = (g^{P-1} / n)^{n} = 1 (mod p) as required
*/
fn mod_pow(g: u128, p: u128, exp: u128) -> u64 {
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
fn find_nth_root_of_unity(n: u128, p: u128) -> u64 {
    let g: u128 = 7u128; // use the smallest generator in Goldilocks
    let exp: u128 = ((p - 1) / n) as u128;

    mod_pow(g, p, exp)
}


// evaluate a polynomial (given by a coefficient vector and basis), evaluate at the point x
fn poly_eval(polynomial: &[GoldilocksField], x: GoldilocksField) -> GoldilocksField {
    let mut result: GoldilocksField = GoldilocksField::ZERO;
    for &coeff in polynomial {
        result = result * x + coeff;
    }

    result
}


// generate n_large points to expand the evaluation domain once the low-degree polynomial is calculated
fn generate_blow_up_points(
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


// main function, for FFT will need to pad the number of points to a mutiple of two
fn main() {
    let p: u128 = 0xFFFFFFFF00000001; // the prime modulus for Goldilocks
    println!("[*] Running over the Goldilocks field with modulus 0x{:X}", p);
    // define the values which are the y-coordinates of our points
    let values: Vec<u64> = vec![4u64, 5u64, 6u64, 7u64];
    let n_small: u128 = values.len() as u128;
    let n_large: u128 = 64u128;

    println!("[*] Recovering low-degree polynomial that passes through n_small = {} points", n_small);
    println!("[*] Low-degree polynomial will be evaluated on n_large = {} points", n_large);

    // find a primitive nth root of unity where n = len(values)
    let large_omega_raw: u64 = find_nth_root_of_unity(n_large as u128, p);
    let large_omega: GoldilocksField = GoldilocksField::from_canonical_u64(large_omega_raw);
    let ratio: u64 = (n_large / n_small) as u64; // 64 / 4 = 16 
    let omega: GoldilocksField = large_omega.exp_u64(ratio); // omega = large_omega^{16}

    println!("[*] Recovered a {}th root of unity 0x{:X}", n_large, large_omega.0);
    assert_eq!(large_omega.exp_u64(n_large as u64), GoldilocksField::ONE); // sanity checks
    assert_eq!(omega.exp_u64(n_small as u64), GoldilocksField::ONE);

    // define a set of points to interpolate: (omega^{0}, 4), (omega^{1}, 5), (omega^{2}, 6), omega^{3}, 7)
    println!("[*] Constructed n={} points using {}th root of unity powers as x-coordinates and values as y-coordinates", n_small, n_small);
    let mut points: Vec<(GoldilocksField, GoldilocksField)> = vec![];
    let mut omega_pow: GoldilocksField = GoldilocksField::ONE; // start at omega^{0} = 1
    for i in 0..(n_small as usize) {
        points.push((omega_pow, GoldilocksField::from_canonical_u64(values[i])));
        omega_pow *= omega; // reduces mod p automatically
    }

    // get the unique Lagrange Interpolation polynomial that passes through the n points
    let lagrange_coeffs: Vec<GoldilocksField> = lagrange_interpolate(&points);
    println!("[*] Recovered polynomial coefficient vector: {:?} with degree: {}", lagrange_coeffs, lagrange_coeffs.len() - 1);

    // evaluate the recovered low-degree polynomial on n_large points
    let total_points: Vec<(GoldilocksField, GoldilocksField)> = generate_blow_up_points(n_large, large_omega, &lagrange_coeffs);
}
