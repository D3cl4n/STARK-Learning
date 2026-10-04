mod prover;
mod verifier;
mod polynomial;
mod merkle;

use plonky2_field::goldilocks_field::GoldilocksField;
use plonky2_field::types::Field;


// main function, for FFT will need to pad the number of points to a mutiple of two
fn main() {
    // define the values which are the y-coordinates of our points
    let values: Vec<u64> = vec![4u64, 5u64, 6u64, 7u64];

    // commit to the y-values of the points that lie on the low-degree Lagrange Polynomial passing through values (after blow-up step)
    let prover_output: prover::ProverOutput = prover::commit(&values);

    // verifier chooses random challenge points and verifies they are on the Lagrange Polynomial and in the Merkle tree
    let challenge_points: Vec<u64> = prover::generate_challenge_points(&prover_output.root, values.len());

    // prover computes authenticaiton path for challenge points
    let mut auth_paths: Vec<Vec<merkle::Hash>> = vec![];
    let mut exposed_points: Vec<GoldilocksField> = vec![];
    for i in 0..challenge_points.len() {
        auth_paths.push(merkle::open(&prover_output.tree, challenge_points[i] as usize));
        exposed_points.push(GoldilocksField::from_canonical_u64(
            prover_output.tree.layers.first().unwrap()[challenge_points[i] as usize][1].into())
        );
    }

    // non-interactive proof
    verifier::verify_challenge_points(&prover_output.root, values.len(), &exposed_points, &auth_paths);
}