use tree::alv_tree::avl::avl_set::AvlTree;
fn main() {
    let mut tree = AvlTree::new();

    for i in 1..15 {
        tree.add(i);
    }
    let tree_iter = tree.iter();

    for i in tree_iter {
        println!("Got {}", i);
    }

    /* tree.show_in_level(); */
    /* println!("Balance Factor = {} ", tree.root.unwrap().balance_fac); */
    /* tree.show_in_order(&tree.root); */
}
