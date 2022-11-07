use tree::avl::avlset::AvlTree;

fn main() {
    let mut tree = AvlTree::new();

    for i in 1..=100 {
        tree.add(i);
    }

    tree.show_in_level();
}
