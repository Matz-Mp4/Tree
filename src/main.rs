use tree::avl::avlset::AvlTree;

fn main() {
    let mut tree = AvlTree::new();

    for i in 1..=20 {
        tree.add(i);
    }



}
