#[cfg(test)]
mod tests {

    /* use tree::alv_tree::avl::avl_set::AvlTree; */

    mod avl {}

    mod node {
        use std::cmp::Ordering;
        use tree::alv_tree::{avl::avl_set::AvlTree, node::avl_node::Node};

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
