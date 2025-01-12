use std::{
    cell::RefCell,
    fmt::{self},
    rc::Rc,
};

#[derive(Debug, Clone)]
pub struct LinkedListNode<T> {
    value: T,
    next: Option<Rc<RefCell<LinkedListNode<T>>>>,
}

#[derive(Debug, Clone)]
pub struct LinkedList<T> {
    len: usize,
    head: Option<Rc<RefCell<LinkedListNode<T>>>>,
    tail: Option<Rc<RefCell<LinkedListNode<T>>>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkedListError {
    EmptyList,
    InsertOutOfRange,
    RemoveOutOfRange,
    RemoveFromEmptyList,
    NextIsNone,
}

impl<T> LinkedListNode<T>
where
    T: Clone,
{
    pub fn new(val: T, next_node: Option<Rc<RefCell<LinkedListNode<T>>>>) -> Self {
        LinkedListNode {
            value: val,
            next: next_node,
        }
    }

    pub fn next(&self) -> Option<Rc<RefCell<LinkedListNode<T>>>> {
        // get a counting Pointer to next node
        self.next.clone()
    }

    pub fn insert(&mut self, val: &T) {
        // note: you can not get the ownership of self, so it is not possible to create a Pointer to self
        let node = LinkedListNode::new(val.clone(), self.next());
        self.next = Some(Rc::new(RefCell::new(node)));
    }

    pub fn remove(&mut self) -> Result<T, LinkedListError> {
        if let Some(node) = self.next.as_ref() {
            let next_ptr = node.borrow().next();
            let val = node.borrow().value.clone();
            self.next = next_ptr;
            Ok(val)
        } else {
            Err(LinkedListError::NextIsNone)
        }
    }
}

impl<T> LinkedList<T>
where
    T: Clone + std::cmp::PartialEq,
{
    pub fn new() -> Self {
        LinkedList {
            len: 0,
            head: None,
            tail: None,
        }
    }

    pub fn push_head(&mut self, val: &T) {
        if self.head.is_none() {
            let node = Rc::new(RefCell::new(LinkedListNode {
                value: val.clone(),
                next: None,
            }));

            self.head = Some(node.clone());
            self.tail = Some(node.clone());
            self.len += 1;
        } else {
            let node = Rc::new(RefCell::new(LinkedListNode {
                value: val.clone(),
                next: self.head.clone(), // get MutRc Pointer to where head pointer to
            }));

            self.head = Some(node.clone());
            self.len += 1;
        }
    }

    pub fn push_back(&mut self, val: &T) {
        if self.tail.is_none() {
            self.push_head(val);
        } else {
            self.tail.as_ref().unwrap().borrow_mut().insert(val);
            let tail_next_ptr = self.tail.as_ref().unwrap().borrow().next();
            self.tail = tail_next_ptr;

            self.len += 1;
        }
    }

    pub fn pop_head(&mut self) -> Result<T, LinkedListError> {
        if self.len == 0 {
            Err(LinkedListError::EmptyList)
        } else if self.len == 1 {
            let val = self.head.as_ref().unwrap().borrow().value.clone();
            self.head = None;
            self.tail = None;
            self.len = 0;
            Ok(val)
        } else {
            let node = self.head.as_ref().unwrap().borrow().next();
            let val = self.head.as_ref().unwrap().borrow().value.clone();
            self.head = node;
            self.len -= 1;
            Ok(val)
        }
    }

    pub fn pop_back(&mut self) -> Result<T, LinkedListError> {
        if self.len == 0 {
            Err(LinkedListError::EmptyList)
        } else if self.len == 1 {
            self.pop_head()
        } else {
            let mut curr = self.head.as_ref().unwrap().clone();
            for _ in 0..self.len - 2 {
                let node = curr.borrow_mut().next.as_ref().unwrap().clone();
                curr = node;
            }
            let val = curr.borrow_mut().remove().unwrap();
            self.tail = Some(curr);
            self.len -= 1;
            Ok(val)
        }
    }

    pub fn insert(&mut self, val: &T, at: usize) -> Result<(), LinkedListError> {
        if at == 0 {
            self.push_head(val);
            Ok(())
        } else if (0 < at) && (at < self.len + 1) {
            let mut curr = self.head.as_ref().unwrap().clone();
            for _ in 0..at - 1 {
                let node = curr.borrow().next.as_ref().unwrap().clone();
                curr = node;
            }

            curr.borrow_mut().insert(val);
            self.len += 1;

            Ok(())
        } else {
            Err(LinkedListError::InsertOutOfRange)
        }
    }

    pub fn remove(&mut self, at: usize) -> Result<T, LinkedListError> {
        if at == 0 {
            if self.len == 0 {
                return Err(LinkedListError::RemoveFromEmptyList);
            }
            self.pop_head()
        } else if (0 < at) && (at < self.len) {
            let mut curr = self.head.as_ref().unwrap().clone();
            for _ in 0..at - 1 {
                let node = curr.borrow().next.as_ref().unwrap().clone();
                curr = node;
            }
            let val = curr.borrow_mut().remove().unwrap();
            self.len -= 1;
            Ok(val)
        } else {
            Err(LinkedListError::RemoveOutOfRange)
        }
    }

    pub fn val2ix(&self, val: &T) -> Vec<usize> {
        if self.len() == 0 {
            return vec![];
        }

        let mut curr = self.head.as_ref().unwrap().clone();
        let mut res = vec![];
        println!("len is: {}", self.len);
        for ix in 0..self.len - 1 {
            if curr.borrow().value == *val {
                res.push(ix);
            }
            let node = curr.borrow().next.as_ref().unwrap().clone();
            curr = node;
        }

        if curr.borrow().value == *val {
            res.push(self.len - 1);
        }

        res
    }

    pub fn ix2val(&self, ix: usize) -> Option<T> {
        if ix >= self.len {
            return None;
        }

        let mut curr = self.head.as_ref().unwrap().clone();
        for _ in 0..ix {
            let node = curr.borrow().next.as_ref().unwrap().clone();
            curr = node;
        }
        let x = curr.borrow().value.clone();
        Some(x)
    }

    pub fn get(&self, ix: usize) -> Option<T> {
        self.ix2val(ix)
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T: Default> Default for LinkedListNode<T> {
    fn default() -> Self {
        LinkedListNode {
            value: T::default(),
            next: None,
        }
    }
}

impl<T: fmt::Display> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;

        let mut curr = self.head.clone();
        let mut first = true;

        while let Some(node) = curr {
            let node_ref = node.borrow();
            if !first {
                write!(f, " -> ")?;
            }
            write!(f, "{}", node_ref.value)?;
            first = false;
            curr = node_ref.next.clone();
        }

        write!(f, ")")?;
        Ok(())
    }
}

// Unit Test for LinkedList
#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_push_head() {
        let mut list = LinkedList::new();
        list.push_head(&1);
        assert_eq!(list.len(), 1);
        assert_eq!(list.get(0), Some(1));

        list.push_head(&2);
        assert_eq!(list.len(), 2);
        assert_eq!(list.get(0), Some(2));
        assert_eq!(list.get(1), Some(1));
    }

    #[test]
    fn test_push_back() {
        let mut list = LinkedList::new();
        list.push_back(&1);
        assert_eq!(list.len(), 1);
        assert_eq!(list.get(0), Some(1));

        list.push_back(&2);
        assert_eq!(list.len(), 2);
        assert_eq!(list.get(1), Some(2));
    }

    #[test]
    fn test_pop_head() {
        let mut list = LinkedList::new();
        assert_eq!(list.pop_head(), Err(LinkedListError::EmptyList));

        list.push_head(&1);
        list.push_head(&2);
        assert_eq!(list.pop_head(), Ok(2));
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_head(), Ok(1));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_head(), Err(LinkedListError::EmptyList));
    }

    #[test]
    fn test_pop_back() {
        let mut list = LinkedList::new();
        assert_eq!(list.pop_back(), Err(LinkedListError::EmptyList));

        list.push_back(&1);
        list.push_back(&2);
        list.push_back(&3);
        assert_eq!(list.pop_back(), Ok(3));
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_back(), Ok(2));
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_back(), Ok(1));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_back(), Err(LinkedListError::EmptyList));
    }

    #[test]
    fn test_insert() {
        let mut list = LinkedList::new();
        assert_eq!(list.insert(&1, 1), Err(LinkedListError::InsertOutOfRange));

        list.push_back(&1);
        list.push_back(&3);
        assert_eq!(list.insert(&2, 1), Ok(()));
        assert_eq!(list.len(), 3);
        assert_eq!(list.get(1), Some(2));

        assert_eq!(list.insert(&4, 3), Ok(()));
        assert_eq!(list.len(), 4);
        assert_eq!(list.get(3), Some(4));

        assert_eq!(list.insert(&0, 0), Ok(()));
        assert_eq!(list.len(), 5);
        assert_eq!(list.get(0), Some(0));

        // Attempt to insert out of range
        assert_eq!(list.insert(&5, 6), Err(LinkedListError::InsertOutOfRange));
    }

    #[test]
    fn test_remove() {
        let mut list = LinkedList::new();
        assert_eq!(list.remove(0), Err(LinkedListError::RemoveFromEmptyList));

        list.push_back(&1);
        list.push_back(&2);
        list.push_back(&3);
        assert_eq!(list.remove(1), Ok(2));
        assert_eq!(list.len(), 2);
        assert_eq!(list.get(1), Some(3));

        assert_eq!(list.remove(0), Ok(1));
        assert_eq!(list.len(), 1);
        assert_eq!(list.get(0), Some(3));

        assert_eq!(list.remove(0), Ok(3));
        assert_eq!(list.len(), 0);
        assert_eq!(list.remove(0), Err(LinkedListError::RemoveFromEmptyList));
    }

    #[test]
    fn test_val2ix() {
        let mut list = LinkedList::new();
        assert_eq!(list.val2ix(&1), vec![]);

        list.push_back(&1);
        list.push_back(&2);
        list.push_back(&3);
        list.push_back(&2);

        assert_eq!(list.val2ix(&1), vec![0]);
        assert_eq!(list.val2ix(&2), vec![1, 3]);
        assert_eq!(list.val2ix(&3), vec![2]);
        assert_eq!(list.val2ix(&4), vec![]);
    }

    #[test]
    fn test_ix2val() {
        let mut list = LinkedList::new();
        list.push_back(&10);
        list.push_back(&20);
        list.push_back(&30);

        assert_eq!(list.ix2val(0), Some(10));
        assert_eq!(list.ix2val(1), Some(20));
        assert_eq!(list.ix2val(2), Some(30));
        assert_eq!(list.ix2val(3), None);
    }

    #[test]
    fn test_get() {
        let mut list = LinkedList::new();
        list.push_back(&100);
        list.push_back(&200);

        assert_eq!(list.get(0), Some(100));
        assert_eq!(list.get(1), Some(200));
        assert_eq!(list.get(2), None);
    }

    #[test]
    fn test_len() {
        let mut list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.len(), 0);

        list.push_head(&1);
        assert_eq!(list.len(), 1);

        list.push_back(&2);
        assert_eq!(list.len(), 2);

        list.pop_head().unwrap();
        assert_eq!(list.len(), 1);

        list.pop_back().unwrap();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_display() {
        let mut list = LinkedList::new();
        assert_eq!(format!("{}", list), "()");

        list.push_back(&1);
        assert_eq!(format!("{}", list), "(1)");

        list.push_back(&2);
        list.push_back(&3);
        assert_eq!(format!("{}", list), "(1 -> 2 -> 3)");

        list.pop_head().unwrap();
        assert_eq!(format!("{}", list), "(2 -> 3)");

        list.pop_back().unwrap();
        assert_eq!(format!("{}", list), "(2)");
    }

    #[test]
    fn test_clone() {
        let mut list = LinkedList::new();
        list.push_back(&1);
        list.push_back(&2);
        list.push_back(&3);

        let cloned_list = list.clone();
        assert_eq!(cloned_list.len(), 3);
        assert_eq!(cloned_list.get(0), Some(1));
        assert_eq!(cloned_list.get(1), Some(2));
        assert_eq!(cloned_list.get(2), Some(3));

        // Ensure that modifying the original list does not affect the cloned list
        list.pop_back().unwrap();
        assert_eq!(list.len(), 2);
        assert_eq!(cloned_list.len(), 3);
    }

    #[test]
    fn test_insert_remove_multiple() {
        let mut list = LinkedList::new();
        // Insert elements at various positions
        list.push_back(&1); // List: 1
        list.push_back(&3); // List: 1 -> 3
        list.insert(&2, 1).unwrap(); // List: 1 -> 2 -> 3
        list.insert(&4, 3).unwrap(); // List: 1 -> 2 -> 3 -> 4
        list.insert(&0, 0).unwrap(); // List: 0 -> 1 -> 2 -> 3 -> 4

        assert_eq!(list.len(), 5);
        assert_eq!(format!("{}", list), "(0 -> 1 -> 2 -> 3 -> 4)");

        // Remove elements from various positions
        assert_eq!(list.remove(2), Ok(2)); // List: 0 -> 1 -> 3 -> 4
        assert_eq!(list.remove(0), Ok(0)); // List: 1 -> 3 -> 4
        assert_eq!(list.remove(2), Ok(4)); // List: 1 -> 3

        assert_eq!(list.len(), 2);
        assert_eq!(format!("{}", list), "(1 -> 3)");
    }
}
