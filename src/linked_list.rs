#[derive(Debug, Clone)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

#[derive(Debug, Clone)]
pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    pub fn len(&self) -> usize {
        let mut current = &self.head;
        let mut length = 0;
        while let Some(node) = current {
            length += 1;
            current = &node.next;
        }
        length
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}
