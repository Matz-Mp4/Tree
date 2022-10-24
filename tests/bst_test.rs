#[cfg(test)]
mod tests {

    use tree::bst::binary_search_tree::bst::BinaryTree;

    #[test]
    fn amount_verification() {
        let mut tree: BinaryTree<usize> = BinaryTree::new();
        assert_eq!(0, tree.get_amount_nodes());

        for i in (1..4 as usize).rev() {
            tree.add(i);
        }

        assert_eq!(3, tree.get_amount_nodes());
    }

    #[test]
    fn get_data_verification() {
        let mut tree: BinaryTree<i32> = BinaryTree::new();

        for i in (1..4).rev() {
            tree.add(i);
        }

        for i in (1..4).rev() {
            assert_eq!(&i, tree.get(&i).unwrap());
        }

        assert_eq!(None, tree.get(&5));
    }

    #[test]
    fn is_empty_verification() {
        let mut tree: BinaryTree<i32> = BinaryTree::new();

        assert_eq!(true, tree.is_empty());
        tree.add(1);
        tree.add(2);

        assert_eq!(false, tree.is_empty());
    }

    #[test]
    fn remove_node_zero_child() {
        let mut tree: BinaryTree<i32> = BinaryTree::new();
        tree.add(2);
        tree.remove(&2);

        assert_eq!(true, tree.is_empty());

        tree.add(2);
        tree.add(1);
        tree.add(3);

        tree.remove(&1);

        assert_eq!(None, tree.get(&1));
    }

    #[test]
    fn remove_node_one_child() {
        let mut tree: BinaryTree<i32> = BinaryTree::new();
        tree.add(2);
        tree.add(3);
        tree.remove(&3);

        assert_eq!(None, tree.get(&3));

        tree.add(1);
        tree.remove(&1);

        assert_eq!(None, tree.get(&3));

        tree.add(1);
        tree.add(3);
        tree.add(4);
        tree.remove(&4);
        assert_eq!(None, tree.get(&4));
    }

    #[test]
    fn remove_node_two_children() {
        let mut tree: BinaryTree<i32> = BinaryTree::new();
        tree.add(2);
        tree.add(3);
        tree.add(1);
        tree.remove(&2);

        assert_eq!(None, tree.get(&2));
        assert_eq!(&1, tree.get(&1).unwrap());
        assert_eq!(&3, tree.get(&3).unwrap());

        tree.add(-1);
        tree.add(0);
        tree.add(-2);
        tree.remove(&-1);
        assert_eq!(None, tree.get(&-1));
        assert_eq!(&0, tree.get(&0).unwrap());
        assert_eq!(&-2, tree.get(&-2).unwrap());
    }

    #[test]
    fn clear_verification() {
        let mut tree: BinaryTree<i32> = BinaryTree::new();
        tree.add(2);
        tree.add(3);
        tree.add(1);

        tree.clear();

        assert_eq!(None, tree.get(&3));
        assert_eq!(None, tree.get(&2));
        assert_eq!(None, tree.get(&1));
    }
}
