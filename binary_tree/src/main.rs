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
                if value <= node.element {
                    node.left.add(value);
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
    // let jupiter_tree = NonEmpty(Box::new(TreeNode {
    //     element: "Jupiter",
    //     left: Empty,
    //     right: Empty,
    // }));
    //
    // let mercury_tree = NonEmpty(Box::new(TreeNode {
    //     element: "Mercury",
    //     left: Empty,
    //     right: Empty,
    // }));
    //
    // let mars_tree = NonEmpty(Box::new(TreeNode {
    //     element: "Mars",
    //     left: jupiter_tree,
    //     right: mercury_tree,
    // }));
    //
    // let venus_tree = NonEmpty(Box::new(TreeNode {
    //     element: "Venus",
    //     left: Empty,
    //     right: Empty,
    // }));
    //
    // let uranus_tree = NonEmpty(Box::new(TreeNode {
    //     element: "Uranus",
    //     left: Empty,
    //     right: venus_tree,
    // }));
    //
    // let tree = NonEmpty(Box::new(TreeNode {
    //     element: "Saturn",
    //     left: mars_tree,
    //     right: uranus_tree,
    // }));

    // let tree = NonEmpty(Box::new(TreeNode {
    //     element: "Saturn",
    //     left: mars_tree,
    let mut tree = BinaryTree::Empty;
    println!("tree: {:?}", &tree);

    tree.add("Mercury");
    tree.add("Sun");
    tree.add("Venus");

    println!("tree: {:?}", &tree);
}

mod tests {
    use super::*;

    #[test]
    fn test_it() {
        assert!(false);
    }
}
