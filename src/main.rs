use tree::avl::avlset::AvlTree;

fn main() {
    let mut tree = AvlTree::new();

    for i in 1..=10 {
        tree.add(i);
     } 

    tree.remove(&4);
    tree.remove(&5);
    tree.remove(&6);
    tree.remove(&7);
    tree.remove(&8);
    tree.remove(&9);
    tree.remove(&1);

    let iter = tree.node_iter(); 

    for i in iter{
        println!("data = {}, balance_fac = {}", i.data, i.balance_fac);
    }

    tree.show_in_level();

    println!("balance factor: {}" ,tree.root.unwrap().balance_fac);

}
