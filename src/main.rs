use tree::bst::binary_search_tree::bst::BinaryTree;
fn main() {
     let mut tree = BinaryTree::new(); 
    tree.add(5);
    tree.add(3);
    tree.add(7);
    tree.add(6);
    tree.add(4);
    tree.add(1);
    tree.add(2);

    tree.show_in_order(&tree.root); 

}
