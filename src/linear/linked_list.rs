use std::{
    cell::RefCell,
    fmt::{self},
    rc::Rc,
};

/// `LinkedListNode` represents a single node in a linked list containing a value and a reference to the next node.
#[derive(Debug, Clone)]
pub struct LinkedListNode<T> {
    value: T,                                     // The value stored in the node.
    next: Option<Rc<RefCell<LinkedListNode<T>>>>, // A reference to the next node in the list, if any.
}

impl<T> LinkedListNode<T>
where
    T: Clone,
{
    /// Creates a new `LinkedListNode` with the given value and next node.
    ///
    /// # Arguments
    ///
    /// * `val` - The value to be stored in the node.
    /// * `next_node` - The reference to the next node in the list, if any.
    ///
    /// # Returns
    ///
    /// A new `LinkedListNode` with the provided value and next node.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hym::linked_list::LinkedListNode;
    ///
    /// let node = LinkedListNode::new(1, None);
    /// ```
    pub fn new(val: T, next_node: Option<Rc<RefCell<LinkedListNode<T>>>>) -> Self {
        LinkedListNode {
            value: val,
            next: next_node,
        }
    }

    /// Returns a reference to the next node in the list.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the next node, if any.
    pub fn next(&self) -> Option<Rc<RefCell<LinkedListNode<T>>>> {
        self.next.clone()
    }

    /// Inserts a new node with the given value after the current node.
    ///
    /// # Arguments
    ///
    /// * `val` - The value to be inserted after the current node.
    ///
    /// # Examples
    ///
    /// ```text
    /// call node1.insert(&4) >> node1(1)         node1(1)
    ///                            ^                ^
    ///                            |                |
    ///                          node2(2)   ==>   node4(4)
    ///                            ^                ^
    ///                            |                |
    ///                          node3(3)         node2(2)
    ///                                             ^
    ///                                             |
    ///                                           node3(3)
    ///
    /// ```
    pub fn insert(&mut self, val: &T) {
        let node = LinkedListNode::new(val.clone(), self.next());
        self.next = Some(Rc::new(RefCell::new(node)));
    }

    /// Removes the next node in the list and returns its value.
    ///
    /// # Returns
    ///
    /// * `Ok(T)` - The value of the removed node.
    /// * `Err(LinkedListError)` - An error if there is no next node to remove.
    ///
    /// # Panics
    ///
    /// Panics if the next node is `None`.
    ///
    /// # Examples
    ///
    /// ```text
    /// call node1.remove() >> node1(1)         node1(1): [✓] return Ok(2)
    ///                          ^                ^     
    ///                          |                |     
    ///                        node2(2)   ==>   node3(3)
    ///                          ^
    ///                          |
    ///                        node3(3)
    ///
    /// ```
    ///
    /// ```text
    ///                        node1(1)         node1(1)
    ///                          ^                ^
    ///                          |                |
    ///                        node2(2)   ==>   node2(2)
    ///                          ^                ^
    ///                          |                |
    /// call node1.remove() >> node3(3)         node3(3): [x] nothing behind, return Err(LinkedListError::NextIsNone)
    ///
    /// ```
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

/// A linked list that supports common operations such as adding and removing elements.
///
/// # Attributes
///
/// * `len` - The length of the list.
/// * `head` - A reference to the first node in the list.
/// * `tail` - A reference to the last node in the list.
///
/// # Explanation
///
/// The `LinkedList` struct represents a linked list data structure. It contains the length of the list, a reference to the first node in the list, and a reference to the last node in the list.
///
/// The relationship of `self.len` and other values in the struct is as follows:
///
/// ## Case1 `self.len == 0`
///
/// ```text
/// head -> None
/// tail -> None
/// ```
///
/// ## Case2 `self.len == 1`
///
/// ```text
/// head -> node
///          ^
/// tail ____|
///
/// ```
///
/// ## Case3 `self.len > 1`
///
/// ```text
/// head -> node1
///           ^
///           |
///         node2
///           ^
///           |
///         node3
///           ^
///           |
/// tail -> node4
///
/// ```
#[derive(Debug, Clone)]
pub struct LinkedList<T> {
    len: usize,                                   // The length of the list.
    head: Option<Rc<RefCell<LinkedListNode<T>>>>, // A reference to the first node in the list.
    tail: Option<Rc<RefCell<LinkedListNode<T>>>>, // A reference to the last node in the list.
}

/// Enum for different types of errors that can occur while manipulating the linked list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkedListError {
    EmptyList,           // Error when the list is empty.
    InsertOutOfRange,    // Error when an insert operation is out of range.
    RemoveOutOfRange,    // Error when a remove operation is out of range.
    RemoveFromEmptyList, // Error when trying to remove from an empty list.
    NextIsNone,          // Error when the next node is `None`.
}

impl<T> LinkedList<T>
where
    T: Clone + std::cmp::PartialEq,
{
    /// Creates a new, empty linked list.
    ///
    /// # Returns
    ///
    /// A new, empty `LinkedList`.
    ///
    /// # Examples  
    ///
    /// ```rust
    /// use hym::linked_list::LinkedList;
    ///
    /// let list: LinkedList<i32> = LinkedList::new();
    /// assert_eq!(format!("{}", list), "()");
    /// ```
    pub fn new() -> Self {
        LinkedList {
            len: 0,
            head: None,
            tail: None,
        }
    }

    /// Adds a new node with the given value to the front (head) of the list.
    ///
    /// # Arguments
    ///
    /// * `val` - The value to be added to the front of the list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hym::linked_list::LinkedList;
    ///
    /// let mut list: LinkedList<i32> = LinkedList::new();
    /// list.push_head(&1);
    /// list.push_head(&2);
    /// list.push_head(&3);
    /// assert_eq!(format!("{}", list), "(3 -> 2 -> 1)");
    /// ```
    ///
    /// # Complexity
    ///
    /// | Time | Space |
    /// | ---- | ----- |
    /// | O(1) | O(1) |
    pub fn push_head(&mut self, val: &T) {
        match self.len {
            0 => {
                let node = LinkedListNode::new(val.clone(), None);
                self.head = Some(Rc::new(RefCell::new(node)));
                self.tail = self.head.clone();
                self.len += 1;
            }
            _ => {
                let node = LinkedListNode::new(val.clone(), self.head.clone());
                self.head = Some(Rc::new(RefCell::new(node)));
                self.len += 1;
            }
        }
    }

    /// Adds a new node with the given value to the end (tail) of the list.
    ///
    /// # Arguments
    ///
    /// * `val` - The value to be added to the end of the list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hym::linked_list::LinkedList;
    ///
    /// let mut list: LinkedList<i32> = LinkedList::new();
    /// list.push_back(&1);
    /// list.push_back(&2);
    /// list.push_back(&3);
    /// assert_eq!(format!("{}", list), "(1 -> 2 -> 3)");
    /// ```
    ///
    /// # Complexity
    ///
    /// | Time | Space |
    /// | ---- | ----- |
    /// | O(1) | O(1) |
    pub fn push_back(&mut self, val: &T) {
        match self.len {
            0 => {
                self.push_head(val);
            }
            _ => {
                self.tail.as_ref().unwrap().borrow_mut().insert(val);
                let tail_next_ptr = self.tail.as_ref().unwrap().borrow().next();
                self.tail = tail_next_ptr;
                self.len += 1;
            }
        }
    }

    /// Removes and returns the value from the front (head) of the list.
    ///
    /// # Returns
    ///
    /// * `Ok(T)` - The value from the removed head node.
    /// * `Err(LinkedListError)` - An error if the list is empty.
    ///
    /// # Panics
    ///
    /// This function will panic if the list is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hym::linked_list::LinkedList;
    ///
    /// let mut list: LinkedList<i32> = LinkedList::new();
    /// list.push_head(&1);
    /// list.push_head(&2);
    /// list.push_head(&3);
    /// assert_eq!(format!("{}", list), "(3 -> 2 -> 1)");
    /// assert_eq!(list.pop_head(), Ok(3));
    /// assert_eq!(format!("{}", list), "(2 -> 1)");
    /// ```
    ///
    /// ```rust
    /// use hym::linked_list::LinkedList;
    /// use hym::linked_list::LinkedListError;
    ///
    /// let mut list: LinkedList<i32> = LinkedList::new();
    /// assert_eq!(list.pop_head(), Err(hym::linked_list::LinkedListError::EmptyList));
    /// ```
    ///
    /// # Complexity
    ///
    /// | Time | Space |
    /// | ---- | ----- |
    /// | O(1) | O(1) |
    pub fn pop_head(&mut self) -> Result<T, LinkedListError> {
        match self.len {
            0 => Err(LinkedListError::EmptyList),
            1 => {
                let val = self.head.as_ref().unwrap().borrow().value.clone();
                self.head = None;
                self.tail = None;
                self.len = 0;
                Ok(val)
            }
            _ => {
                let next_node_ptr = self.head.as_ref().unwrap().borrow().next();
                let val = self.head.as_ref().unwrap().borrow().value.clone();
                self.head = next_node_ptr;
                self.len -= 1;
                Ok(val)
            }
        }
    }

    /// Removes and returns the value from the end (tail) of the list.
    ///
    /// # Returns
    ///
    /// * `Ok(T)` - The value from the removed tail node.
    /// * `Err(LinkedListError)` - An error if the list is empty.
    ///
    /// # Panics
    ///
    /// This function will panic if the list is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hym::linked_list::LinkedList;
    ///
    /// let mut list: LinkedList<i32> = LinkedList::new();
    /// list.push_head(&1);
    /// list.push_head(&2);
    /// list.push_head(&3);
    /// assert_eq!(format!("{}", list), "(3 -> 2 -> 1)");
    /// assert_eq!(list.pop_back(), Ok(1));
    /// assert_eq!(format!("{}", list), "(3 -> 2)");
    /// ```
    ///
    /// ```rust
    /// use hym::linked_list::LinkedList;
    /// use hym::linked_list::LinkedListError;
    ///
    /// let mut list: LinkedList<i32> = LinkedList::new();
    /// assert_eq!(list.pop_back(), Err(hym::linked_list::LinkedListError::EmptyList));
    /// ```
    ///
    /// # Complexity
    ///
    /// | Time | Space |
    /// | --- | --- |
    /// | O(n) | O(1) |
    pub fn pop_back(&mut self) -> Result<T, LinkedListError> {
        match self.len {
            0 => Err(LinkedListError::EmptyList),
            1 => self.pop_head(),
            _ => {
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
    }

    /// Inserts a new value at the specified index in the list. If you insert 'val' at 'at', the
    /// place of 'at' will be 'val'. That is you can use list.get(at) to get the value 'val'.
    ///
    /// # Arguments
    ///
    /// * `val` - The value to be inserted.
    /// * `at` - The index at which to insert the value.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the insertion was successful.
    /// * `Err(LinkedListError)` - If the index is out of range.
    ///
    /// # Panics
    ///
    /// This function will panic if the index is out of range(valid: 0 <= at <= len).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hym::linked_list::LinkedList;
    ///
    /// let mut list: LinkedList<i32> = LinkedList::new();
    /// list.push_head(&1);
    /// list.push_head(&2);
    /// list.push_head(&3);
    /// assert_eq!(format!("{}", list), "(3 -> 2 -> 1)");
    /// assert_eq!(list.insert(&4, 2), Ok(()));
    /// assert_eq!(format!("{}", list), "(3 -> 2 -> 4 -> 1)");
    /// ```
    ///
    /// ```rust
    /// use hym::linked_list::LinkedList;
    /// use hym::linked_list::LinkedListError;
    ///
    /// let mut list: LinkedList<i32> = LinkedList::new();
    /// assert_eq!(list.insert(&4, 2), Err(hym::linked_list::LinkedListError::InsertOutOfRange));
    /// ```
    ///
    /// # Complexity
    ///
    /// | Time | Space |
    /// | --- | --- |
    /// | O(n) | O(1) |
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

    /// Removes and returns the value at the specified index in the list.
    ///
    /// # Arguments
    ///
    /// * `at` - The index of the value to be removed.
    ///
    /// # Returns
    ///
    /// * `Ok(T)` - The value from the removed node.
    /// * `Err(LinkedListError)` - An error if the index is out of range or the list is empty.
    ///
    /// # Panics
    ///
    /// * This function will panic if the list is empty.
    /// * This function will panic if the index is out of range(valid: 0 <= at < len).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hym::linked_list::LinkedList;
    ///
    /// let mut list: LinkedList<i32> = LinkedList::new();
    /// list.push_head(&1);
    /// list.push_head(&2);
    /// list.push_head(&3);
    /// assert_eq!(format!("{}", list), "(3 -> 2 -> 1)");
    /// assert_eq!(list.remove(1), Ok(2));
    /// assert_eq!(format!("{}", list), "(3 -> 1)");
    /// ```
    ///
    /// ```rust
    /// use hym::linked_list::LinkedList;
    /// use hym::linked_list::LinkedListError;
    ///
    /// let mut list: LinkedList<i32> = LinkedList::new();
    /// assert_eq!(list.remove(0), Err(hym::linked_list::LinkedListError::RemoveFromEmptyList));
    /// ```
    ///
    /// ```rust
    /// use hym::linked_list::LinkedList;
    /// use hym::linked_list::LinkedListError;
    ///
    /// let mut list: LinkedList<i32> = LinkedList::new();
    /// assert_eq!(list.remove(1), Err(hym::linked_list::LinkedListError::RemoveOutOfRange));
    /// ```
    ///
    /// # Complexity
    ///
    /// | Time | Space |
    /// | --- | --- |
    /// | O(n) | O(1) |
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

    /// Finds all indices of a given value in the list.
    ///
    /// # Arguments
    ///
    /// * `val` - The value to search for in the list.
    ///
    /// # Returns
    ///
    /// A vector of indices where the value is found in the list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hym::linked_list::LinkedList;
    ///
    /// let mut list: LinkedList<i32> = LinkedList::new();
    /// list.push_head(&1);
    /// list.push_head(&2);
    /// list.push_head(&3);
    /// list.push_head(&2);
    /// assert_eq!(format!("{}", list), "(2 -> 3 -> 2 -> 1)");
    /// assert_eq!(list.val2ix(&2), vec![0, 2]);
    ///
    /// ```
    ///
    /// ```rust
    /// use hym::linked_list::LinkedList;
    ///
    /// let mut list: LinkedList<i32> = LinkedList::new();
    /// assert_eq!(list.val2ix(&2), vec![]);
    /// ```
    ///
    /// # Complexity
    ///
    /// | Time | Space |
    /// | --- | --- |
    /// | O(n) | O(1) |
    pub fn val2ix(&self, val: &T) -> Vec<usize> {
        if self.len() == 0 {
            return vec![];
        }
        let mut curr = self.head.as_ref().unwrap().clone();
        let mut res = vec![];
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

    /// Returns the value at the specified index.
    ///
    /// # Arguments
    ///
    /// * `ix` - The index of the value to retrieve.
    ///
    /// # Returns
    ///
    /// * `Some(T)` - The value at the specified index.
    /// * `None` - If the index is out of range.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hym::linked_list::LinkedList;
    ///
    /// let mut list: LinkedList<i32> = LinkedList::new();
    /// list.push_head(&1);
    /// list.push_head(&2);
    /// list.push_head(&3);
    /// assert_eq!(format!("{}", list), "(3 -> 2 -> 1)");
    /// assert_eq!(list.ix2val(1), Some(2));
    /// ```
    ///
    /// ```rust
    /// use hym::linked_list::LinkedList;
    ///
    /// let mut list: LinkedList<i32> = LinkedList::new();
    /// assert_eq!(list.ix2val(0), None);
    /// ```
    pub fn ix2val(&self, ix: usize) -> Option<T> {
        if ix >= self.len {
            return None;
        }
        let mut curr = self.head.as_ref().unwrap().clone();
        for _ in 0..ix {
            let node = curr.borrow().next.as_ref().unwrap().clone();
            curr = node;
        }
        let x = Some(curr.borrow().value.clone());
        x
    }

    /// Retrieves the value at the specified index.
    ///
    /// # Arguments
    ///
    /// * `ix` - The index of the value to retrieve.
    ///
    /// # Returns
    ///
    /// * `Some(T)` - The value at the specified index.
    /// * `None` - If the index is out of range.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hym::linked_list::LinkedList;
    ///
    /// let mut list: LinkedList<i32> = LinkedList::new();
    /// list.push_head(&1);
    /// list.push_head(&2);
    /// list.push_head(&3);
    /// assert_eq!(format!("{}", list), "(3 -> 2 -> 1)");
    /// assert_eq!(list.get(1), Some(2));
    /// ```
    pub fn get(&self, ix: usize) -> Option<T> {
        self.ix2val(ix)
    }

    /// Returns the current length of the linked list.
    ///
    /// # Returns
    ///
    /// The number of elements in the list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hym::linked_list::LinkedList;
    ///
    /// let mut list: LinkedList<i32> = LinkedList::new();
    /// list.push_head(&1);
    /// list.push_head(&2);
    /// list.push_head(&3);
    /// assert_eq!(format!("{}", list), "(3 -> 2 -> 1)");
    /// assert_eq!(list.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.len
    }

    /// Clears the list by removing all nodes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hym::linked_list::LinkedList;
    ///
    /// let mut list: LinkedList<i32> = LinkedList::new();
    /// list.push_head(&1);
    /// list.push_head(&2);
    /// list.push_head(&3);
    /// assert_eq!(format!("{}", list), "(3 -> 2 -> 1)");
    /// list.clean();
    /// assert_eq!(format!("{}", list), "()");
    /// ```
    pub fn clean(&mut self) {
        self.tail = None;
        self.head = None;
        self.len = 0;
    }

    /// Creates a new linked list from a vector of values.
    ///
    /// # Arguments
    ///
    /// * `vals` - A vector of values to initialize the linked list.
    ///
    /// # Returns
    ///
    /// A new `LinkedList` containing the values from the vector.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hym::linked_list::LinkedList;
    ///
    /// let list: LinkedList<i32> = LinkedList::from_vec(vec![1, 2, 3]);
    /// assert_eq!(format!("{}", list), "(1 -> 2 -> 3)");
    /// ```
    pub fn from_vec(vals: Vec<T>) -> Self {
        let mut list = LinkedList::new();
        for val in vals {
            list.push_back(&val);
        }
        list
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
        // Test adding elements to the head of the list
        let mut list = LinkedList::new();
        list.push_head(&1); // Add 1 to the head
        assert_eq!(list.len(), 1); // List should contain 1 element
        assert_eq!(list.get(0), Some(1)); // First element should be 1

        list.push_head(&2); // Add 2 to the head
        assert_eq!(list.len(), 2); // List should now contain 2 elements
        assert_eq!(list.get(0), Some(2)); // First element should be 2
        assert_eq!(list.get(1), Some(1)); // Second element should be 1
    }

    #[test]
    fn test_push_back() {
        // Test adding elements to the back of the list
        let mut list = LinkedList::new();
        list.push_back(&1); // Add 1 to the back
        assert_eq!(list.len(), 1); // List should contain 1 element
        assert_eq!(list.get(0), Some(1)); // First element should be 1

        list.push_back(&2); // Add 2 to the back
        assert_eq!(list.len(), 2); // List should contain 2 elements
        assert_eq!(list.get(1), Some(2)); // Second element should be 2
    }

    #[test]
    fn test_pop_head() {
        // Test removing elements from the head of the list
        let mut list = LinkedList::new();
        assert_eq!(list.pop_head(), Err(LinkedListError::EmptyList)); // Pop on an empty list should return an error

        list.push_head(&1); // Add 1 to the head
        list.push_head(&2); // Add 2 to the head
        assert_eq!(list.pop_head(), Ok(2)); // Pop should return 2 (head element)
        assert_eq!(list.len(), 1); // List should now contain 1 element
        assert_eq!(list.pop_head(), Ok(1)); // Pop should return 1
        assert_eq!(list.len(), 0); // List should be empty
        assert_eq!(list.pop_head(), Err(LinkedListError::EmptyList)); // Pop on an empty list should return an error
    }

    #[test]
    fn test_pop_back() {
        // Test removing elements from the back of the list
        let mut list = LinkedList::new();
        assert_eq!(list.pop_back(), Err(LinkedListError::EmptyList)); // Pop on an empty list should return an error

        list.push_back(&1); // Add 1 to the back
        list.push_back(&2); // Add 2 to the back
        list.push_back(&3); // Add 3 to the back
        assert_eq!(list.pop_back(), Ok(3)); // Pop should return 3 (last element)
        assert_eq!(list.len(), 2); // List should now contain 2 elements
        assert_eq!(list.pop_back(), Ok(2)); // Pop should return 2
        assert_eq!(list.len(), 1); // List should now contain 1 element
        assert_eq!(list.pop_back(), Ok(1)); // Pop should return 1
        assert_eq!(list.len(), 0); // List should be empty
        assert_eq!(list.pop_back(), Err(LinkedListError::EmptyList)); // Pop on an empty list should return an error
    }

    #[test]
    fn test_insert() {
        // Test inserting elements at a specific position
        let mut list = LinkedList::new();
        assert_eq!(list.insert(&1, 1), Err(LinkedListError::InsertOutOfRange)); // Inserting out of range

        list.push_back(&1); // Add 1 to the back
        list.push_back(&3); // Add 3 to the back
        assert_eq!(list.insert(&2, 1), Ok(())); // Insert 2 at position 1
        assert_eq!(list.len(), 3); // List should contain 3 elements
        assert_eq!(list.get(1), Some(2)); // Element at position 1 should be 2

        assert_eq!(list.insert(&4, 3), Ok(())); // Insert 4 at position 3
        assert_eq!(list.len(), 4); // List should contain 4 elements
        assert_eq!(list.get(3), Some(4)); // Element at position 3 should be 4

        assert_eq!(list.insert(&0, 0), Ok(())); // Insert 0 at position 0
        assert_eq!(list.len(), 5); // List should contain 5 elements
        assert_eq!(list.get(0), Some(0)); // Element at position 0 should be 0

        // Attempt to insert out of range
        assert_eq!(list.insert(&5, 6), Err(LinkedListError::InsertOutOfRange)); // Inserting out of range should return an error
    }

    #[test]
    fn test_remove() {
        // Test removing elements at a specific position
        let mut list = LinkedList::new();
        assert_eq!(list.remove(0), Err(LinkedListError::RemoveFromEmptyList)); // Remove from an empty list should return an error

        list.push_back(&1); // Add 1 to the back
        list.push_back(&2); // Add 2 to the back
        list.push_back(&3); // Add 3 to the back
        assert_eq!(list.remove(1), Ok(2)); // Remove element at position 1 (value 2)
        assert_eq!(list.len(), 2); // List should now contain 2 elements
        assert_eq!(list.get(1), Some(3)); // Element at position 1 should be 3

        assert_eq!(list.remove(0), Ok(1)); // Remove element at position 0 (value 1)
        assert_eq!(list.len(), 1); // List should now contain 1 element
        assert_eq!(list.get(0), Some(3)); // Element at position 0 should be 3

        assert_eq!(list.remove(0), Ok(3)); // Remove last element (value 3)
        assert_eq!(list.len(), 0); // List should be empty
        assert_eq!(list.remove(0), Err(LinkedListError::RemoveFromEmptyList)); // Remove from an empty list should return an error
    }

    #[test]
    fn test_val2ix() {
        // Test finding indices of a specific value
        let mut list = LinkedList::new();
        assert_eq!(list.val2ix(&1), vec![]); // No elements in the list

        list.push_back(&1); // Add 1 to the back
        list.push_back(&2); // Add 2 to the back
        list.push_back(&3); // Add 3 to the back
        list.push_back(&2); // Add another 2 to the back

        assert_eq!(list.val2ix(&1), vec![0]); // 1 is at index 0
        assert_eq!(list.val2ix(&2), vec![1, 3]); // 2 is at indices 1 and 3
        assert_eq!(list.val2ix(&3), vec![2]); // 3 is at index 2
        assert_eq!(list.val2ix(&4), vec![]); // No 4 in the list
    }

    #[test]
    fn test_ix2val() {
        // Test accessing value by index
        let mut list = LinkedList::new();
        list.push_back(&10); // Add 10 to the back
        list.push_back(&20); // Add 20 to the back
        list.push_back(&30); // Add 30 to the back

        assert_eq!(list.ix2val(0), Some(10)); // Element at index 0 should be 10
        assert_eq!(list.ix2val(1), Some(20)); // Element at index 1 should be 20
        assert_eq!(list.ix2val(2), Some(30)); // Element at index 2 should be 30
        assert_eq!(list.ix2val(3), None); // No element at index 3
    }

    #[test]
    fn test_get() {
        // Test retrieving element at a specific index
        let mut list = LinkedList::new();
        list.push_back(&100); // Add 100 to the back
        list.push_back(&200); // Add 200 to the back

        assert_eq!(list.get(0), Some(100)); // Element at index 0 should be 100
        assert_eq!(list.get(1), Some(200)); // Element at index 1 should be 200
        assert_eq!(list.get(2), None); // No element at index 2
    }

    #[test]
    fn test_len() {
        // Test the length of the list
        let mut list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.len(), 0); // Empty list

        list.push_head(&1); // Add 1 to the head
        assert_eq!(list.len(), 1); // List should contain 1 element

        list.push_back(&2); // Add 2 to the back
        assert_eq!(list.len(), 2); // List should contain 2 elements

        list.pop_head().unwrap(); // Remove from head
        assert_eq!(list.len(), 1); // List should contain 1 element

        list.pop_back().unwrap(); // Remove from back
        assert_eq!(list.len(), 0); // List should be empty
    }

    #[test]
    fn test_display() {
        // Test the display of the list
        let mut list = LinkedList::new();
        assert_eq!(format!("{}", list), "()"); // Empty list

        list.push_back(&1); // Add 1 to the back
        assert_eq!(format!("{}", list), "(1)");

        list.push_back(&2); // Add 2 to the back
        list.push_back(&3); // Add 3 to the back
        assert_eq!(format!("{}", list), "(1 -> 2 -> 3)");

        list.pop_head().unwrap(); // Remove from head
        assert_eq!(format!("{}", list), "(2 -> 3)");

        list.pop_back().unwrap(); // Remove from back
        assert_eq!(format!("{}", list), "(2)");
    }

    #[test]
    fn test_clone() {
        // Test cloning the list
        let mut list = LinkedList::new();
        list.push_back(&1); // Add 1 to the back
        list.push_back(&2); // Add 2 to the back
        list.push_back(&3); // Add 3 to the back

        let cloned_list = list.clone(); // Clone the list
        assert_eq!(cloned_list.len(), 3); // Cloned list should contain 3 elements
        assert_eq!(cloned_list.get(0), Some(1)); // First element should be 1
        assert_eq!(cloned_list.get(1), Some(2)); // Second element should be 2
        assert_eq!(cloned_list.get(2), Some(3)); // Third element should be 3

        // Ensure modifying original list does not affect cloned list
        list.pop_back().unwrap(); // Modify original list
        assert_eq!(list.len(), 2); // Original list should have 2 elements
        assert_eq!(cloned_list.len(), 3); // Cloned list should still have 3 elements
    }

    #[test]
    fn test_insert_remove_multiple() {
        // Test inserting and removing multiple elements
        let mut list = LinkedList::new();
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

    #[test]
    fn test_clean() {
        // Test cleaning the list
        let mut list = LinkedList::new();

        // Test clean on an empty list
        list.clean();
        assert_eq!(list.len(), 0);
        assert_eq!(format!("{}", list), "()");

        // Test clean on a list with elements
        list.push_back(&1); // Add 1 to the back
        list.push_back(&2); // Add 2 to the back
        list.push_back(&3); // Add 3 to the back
        assert_eq!(list.len(), 3);
        assert_eq!(format!("{}", list), "(1 -> 2 -> 3)");

        // Call clean and ensure the list is empty
        list.clean();
        assert_eq!(list.len(), 0);
        assert_eq!(format!("{}", list), "()");
    }

    #[test]
    fn test_from_vec() {
        // Test creating a list from a vector
        let list: LinkedList<i32> = LinkedList::from_vec(vec![]);
        assert_eq!(list.len(), 0); // Empty list
        assert_eq!(format!("{}", list), "()");

        let list = LinkedList::from_vec(vec![1, 2, 3]);
        assert_eq!(list.len(), 3); // List should contain 3 elements
        assert_eq!(format!("{}", list), "(1 -> 2 -> 3)");

        let list = LinkedList::from_vec(vec![1, 1, 1, 1]);
        assert_eq!(list.len(), 4); // List should contain 4 elements
        assert_eq!(format!("{}", list), "(1 -> 1 -> 1 -> 1)");
    }
}
