use tree::avl::avlset::AvlTree;

fn main() {
    let mut tree = AvlTree::new();

    for i in 1..=10 {
        tree.add(i);
    }

}
