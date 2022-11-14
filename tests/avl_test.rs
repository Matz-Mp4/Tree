#[cfg(test)]
mod tests {

    mod avl {
        use tree::avl::avlset::AvlTree;
        #[test]
        fn add_verification() {
            let mut tree: AvlTree<i32> = AvlTree::new();

            for i in 1..=100 {
                tree.add(i);
            }

            let iter = tree.node_iter();
            let bal_iter = iter.map(|node| node.balance_fac < 2 && node.balance_fac > -2);

            for checker in bal_iter {
                assert_eq!(true, checker);
            }
        }

        #[test]
        fn remove_verification() {
            let mut tree: AvlTree<i32> = AvlTree::new();

            for i in 1..=10000 {
                tree.add(i);
            }

            while !tree.is_empty() {
                let value_in_root = tree.root.as_ref().unwrap().data;

                let iter = tree.node_iter();
                let bal_iter = iter.map(|node| node.balance_fac < 2 && node.balance_fac > -2);

                for checker in bal_iter {
                    assert_eq!(true, checker);
                }
                tree.remove(&value_in_root);
            }
        }
    }

    mod node {
        use std::cmp::Ordering;
        use tree::avl::{avlset::AvlTree, node::Node};

        fn add_no_rotation(tree: &mut AvlTree<i32>, value: i32) {
            let mut current_tree = &mut tree.root;

            while let Some(current_node) = current_tree {
                match current_node.data.cmp(&value) {
                    Ordering::Less => current_tree = &mut current_node.right,
                    Ordering::Equal => {
                        return;
                    }
                    Ordering::Greater => current_tree = &mut current_node.left,
                }
            }
            *current_tree = Some(Box::new(Node {
                data: value,
                left: None,
                right: None,
                balance_fac: 0,
            }));
        }

        #[test]
        fn rotation_right() {
            let mut tree: AvlTree<i32> = AvlTree::new();
            add_no_rotation(&mut tree, 2);
            add_no_rotation(&mut tree, 1);
            add_no_rotation(&mut tree, 0);
            let root_node = &mut tree.root.unwrap();
            root_node.rotation_right();
            assert_eq!(1, root_node.data);
            assert_eq!(0, root_node.left.as_mut().unwrap().data);
            assert_eq!(2, root_node.right.as_mut().unwrap().data);
        }

        #[test]
        fn rotation_left() {
            let mut tree: AvlTree<i32> = AvlTree::new();
            add_no_rotation(&mut tree, 0);
            add_no_rotation(&mut tree, 1);
            add_no_rotation(&mut tree, 2);
            let root_node = &mut tree.root.unwrap();
            root_node.rotation_left();

            assert_eq!(1, root_node.data);
            assert_eq!(0, root_node.left.as_mut().unwrap().data);
            assert_eq!(2, root_node.right.as_mut().unwrap().data);
        }
    }
}
