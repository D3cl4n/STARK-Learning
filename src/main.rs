mod prover;
mod verifier;
mod polynomial;
mod merkle;


// main function, for FFT will need to pad the number of points to a mutiple of two
fn main() {
    // define the values which are the y-coordinates of our points
    let values: Vec<u64> = vec![4u64, 5u64, 6u64, 7u64];

    // commit to the y-values of the points that lie on the low-degree Lagrange Polynomial passing through values (after blow-up step)
    let merkle_tree: merkle::MerkleTree = prover::commit(&values);

    // verifier chooses random challenge points and verifies they are on the Lagrange Polynomial and in the Merkle tree
    let challenge_points: Vec<u64> = verifier.generate_challenge_points();

    // prover computes authenticaiton path for challenge points
    let mut auth_paths: Vec<merkle::Hash> = vec![];
    for i in 0..challenge_points.len() {
        auth_paths.push(merkle::open(merkle_tree, challenge_points[i]));
    }

    // verifier checks the authentication path given Merkle root, leaf hash, and claimed index
    for i in 0..challenge_points.len() {
        assert_eq!(true, verifier.verify_challenge(&merkle_tree.get_root(), merkle::hash_leaf_node(values[i]), i as usize, auth_path[i]));
    }
}