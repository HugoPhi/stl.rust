use crate::LinkedList;
use crate::LinkedListError;

#[derive(Debug)]
struct Stack<T> {
    list: LinkedList<T>,
}

impl<T: Clone + std::cmp::PartialOrd> Stack<T> {
    fn new() -> Stack<T> {
        Stack {
            list: LinkedList::new(),
        }
    }

    fn push(&mut self, item: T) {
        self.list.push_head(item);
    }

    fn pop(&mut self) -> Result<T, LinkedListError> {
        self.list.pop_head()
    }

    fn is_empty(&self) -> bool {
        self.list.is_empty()
    }
}
