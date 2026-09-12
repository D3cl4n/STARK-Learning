use plonky2_field::goldilocks_field::GoldilocksField;
use plonky2_field::types::Field;
mod merkle; 
mod polynomial;
mod stark;


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
    let large_omega_raw: u64 = polynomial::find_nth_root_of_unity(n_large as u128, p);
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
    let lagrange_coeffs: Vec<GoldilocksField> = polynomial::lagrange_interpolate(&points);
    println!("[*] Recovered polynomial coefficient vector: {:?} with degree: {}", lagrange_coeffs, lagrange_coeffs.len() - 1);

    // evaluate the recovered low-degree polynomial on n_large points
    let total_points: Vec<(GoldilocksField, GoldilocksField)> = stark::generate_blow_up_points(n_large, large_omega, &lagrange_coeffs);

    // sanity check that interpolation is preserved in the blow-up dataset
    for i in 0..(n_small as usize) {
        let idx = i * (ratio as usize); // ratio = 16
        assert_eq!(total_points[idx], points[i]);
    }

    // make Merkle leafs out of the total points after blow-up (x-coordinate public and not committed to)
    let y_points: Vec<GoldilocksField> = total_points.iter().map(|(_, y)| *y).collect();
}