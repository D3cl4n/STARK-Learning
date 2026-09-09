use plonky2_field::goldilocks_field::GoldilocksField;
use plonky2_field::types::PrimeField64;
use sha2::{Sha256, Digest};


// hash type for the Merkle tree
pub type Hash = [u8; 32];


// compute the hash of a leaf node
pub fn hash_leaf_node(value: GoldilocksField) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(value.to_canonical_u64().to_le_bytes());
    hasher.finalize().into() // .into() converts to [u8; 32]
}


// compute the hash of a node with 2 children
pub fn hash_parent_node(left: &Hash, right: &Hash) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(left);
    hasher.update(right);
    hasher.finalize().into()
}