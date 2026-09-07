use plonky2_field::goldilocks_field::GoldilocksField;
use plonky2_field::types::Field;
use sha2::{Sha256, Digest};


// structure for the merkle tree nodes
pub struct MerkleNode {
    // sha256 hash of the data, left child, right child (can be None)
    pub hash: Digest,
    pub left_child: MerkleNode,
    pub right_child: MerkleNode
}


// construct a node given the data to be hashed for storage in the node
pub fn construct_merkle_node(data: GoldilocksField) -> MerkleNode {
    let mut node: MerkleNode = 
}