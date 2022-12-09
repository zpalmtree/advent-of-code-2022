use tree::Tree;

pub fn solution(data: Vec<String>) -> String {
    println!("Successfully loaded {} lines of data.", data.len());

    let mut tree = Tree::new();

    let a = tree.add('a');
    let b = tree.add('b');
    let c = tree.add_child(a, 'c');

    let c_node = tree.get(c).unwrap();
    let a_node = tree.get(c_node.parent.unwrap());

    println!("A: {:?}", a_node);

    println!("Tree: {:?}", tree);

    return "".to_string();
}
