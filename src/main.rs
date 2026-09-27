mod prover;
mod verifier;
mod polynomial;


// main function, for FFT will need to pad the number of points to a mutiple of two
fn main() {
    // define the values which are the y-coordinates of our points
    let values: Vec<u64> = vec![4u64, 5u64, 6u64, 7u64];

    // commit to the y-values of the points that lie on the low-degree Lagrange Polynomial passing through values (after blow-up step)
    let merkle_tree: prover::merkle::MerkleTree = prover::commit(&values);

    // // test verifying that a given leaf is an element of the committed vector at the specified index
    // let authentication_path: Vec<merkle::Hash> = tree.open(5);
    // assert_eq!(true, tree.verify(tree.root(), merkle::hash_leaf_node(y_points[5]), &authentication_path, 5));
}