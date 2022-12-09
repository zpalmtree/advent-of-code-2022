#[derive(Debug)]
pub struct Node<T> {
    pub index: usize,

    pub data: T,

    pub parent: Option<usize>,

    pub children: Vec<usize>,
}

impl<T> Node<T> {
    pub fn new(index: usize, data: T, parent: Option<usize>) -> Self {
        Self {
            index,
            data,
            parent,
            children: Vec::new(),
        }
    }
}
