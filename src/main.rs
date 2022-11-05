use tree::bst::binary_search_tree::bst::BinaryTree;
fn main() {
     let mut tree = BinaryTree::new(); 
    tree.add(7);
    tree.add(8);
    tree.add(2);
    tree.add(3);
    tree.add(5);
    tree.add(4);
    tree.remove(&7);
    tree.show_in_order(&tree.root); 

}
