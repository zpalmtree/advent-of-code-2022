pub mod node;

use tree::node::Node;

#[derive(Debug)]
pub struct Tree<T> {
    nodes: Vec<Node<T>>,
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Tree { 
            nodes: Vec::new()
        }
    }

    pub fn add(&mut self, data: T) -> usize {
        let new_index = self.size();

        let node = Node::new(
            new_index,
            data,
            None,
        );

        self.nodes.push(node);

        return new_index;
    }

    pub fn add_child(&mut self, parent: usize, data: T) -> usize {
        let new_index = self.nodes.len();

        let child = Node::new(
            new_index,
            data,
            Some(parent),
        );

        self.nodes.push(child);

        let parent_node = self.get_mut(parent)
            .expect("Parent does not exist!");

        parent_node.children.push(new_index);

        return new_index;
    }

    pub fn get(&self, index: usize) -> Option<&Node<T>> {
        return self.nodes.get(index);
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Node<T>> {
        return self.nodes.get_mut(index);
    }

    pub fn size(&self) -> usize {
        return self.nodes.len();
    }
}
