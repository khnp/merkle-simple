use utils::{Hashable, Hasher};

#[derive(Debug)]
/// Common type for leafs and internal nodes.
/// In order to store custom struct it should implement the Hashable trait.
pub enum TreeElement<T>
where
    T: Hashable,
{
    /// Leafs contain data.
    Leaf {
        /// Value to store withing the tree
        value: T,
        /// Hash of the stored value
        hash: Vec<u8>,
    },
    /// Represents the internal node that can have children
    Node {
        /// Reference to the left subtree
        left: Box<TreeElement<T>>,
        /// Reference to the right subtree
        right: Box<TreeElement<T>>,
        /// Combined hash of child subtrees
        hash: Vec<u8>,
    },
}

impl<T> TreeElement<T>
where
    T: Hashable,
{
    /// Returns hash according to a node type
    pub fn hash(&self) -> &[u8] {
        match *self {
            TreeElement::Leaf { ref hash, .. } => hash,
            TreeElement::Node { ref hash, .. } => hash,
        }
    }

    /// Produces new leaf node from data
    pub fn new_leaf(value: T) -> TreeElement<T> {
        TreeElement::Leaf {
            hash: Hasher::hash_leaf_data(value.get_bytes()),
            value: value,
        }
    }

    /// Produces new internal node from children elements
    pub fn new_node(left: TreeElement<T>, right: TreeElement<T>) -> TreeElement<T> {
        TreeElement::Node {
            hash: Hasher::hash_node_data(left.hash(), right.hash()),
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}
