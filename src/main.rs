use tree::alv_tree::avl::avl_set::AvlTree;
fn main() {
    let mut tree = AvlTree::new();

    tree.add(1);
    tree.add(2);
    tree.add(3);
    tree.add(4);
    tree.add(5);
    tree.add(6);
    tree.add(7);
    tree.add(8);
    tree.add(9);
    tree.add(10);
    tree.add(11);
    tree.add(12);
    tree.add(13);
    tree.add(14);

    tree.show_in_level();
    println!("Balance Factor = {} ", tree.root.unwrap().balance_fac);
    /* tree.show_in_order(&tree.root); */
}
