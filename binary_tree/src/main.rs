use std::collections::BinaryHeap;

// An ordered collection of `T`s
#[derive(Debug)]
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

impl<T: Ord> BinaryTree<T> {
    fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                }))
            }
            BinaryTree::NonEmpty(ref mut node) => {
                // The left tree is for the smaller or equal value
                if value <= node.element {
                    node.left.add(value);
                // The right tree is for the greater value
                } else {
                    node.right.add(value);
                }
            }
        }
    }
}

// A part of a BinaryTree
#[derive(Debug)]
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

use self::BinaryTree::*;

fn main() {
    // Adding values with a binary tree
    let mut tree = NonEmpty(Box::new(TreeNode {
        element: 57,
        left: Empty,
        right: Empty,
    }));
    dbg!(&tree);

    tree.add(58);
    tree.add(60);
    dbg!(&tree);

    tree.add(59);
    dbg!(&tree);

    tree.add(17);
    tree.add(9);
    dbg!(&tree);

    // Now do the same numbers in a BinaryHeap
    let mut heap = BinaryHeap::new();
    heap.push(57);
    heap.push(58);
    heap.push(59);
    heap.push(9);
    heap.push(17);
    dbg!(&heap);

    while heap.len() > 0 {
        let value = heap.pop();
        dbg!(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        assert!(false);
    }
}
