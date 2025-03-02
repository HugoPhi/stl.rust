use std::fmt;
use std::marker::PhantomData;
use std::ptr::NonNull;

/// `LinkedListNode` represents a single node in a linked list containing a value and a reference to the next node.
#[derive(Debug)]
pub struct LinkedListNode<T> {
    value: T,                                 // The value stored in the node.
    next: Option<NonNull<LinkedListNode<T>>>, // A reference to the next node in the list, if any.
}

impl<T> LinkedListNode<T> {
    /// Creates a new `LinkedListNode` with the given value and next node.
    ///
    /// # Arguments
    ///
    /// * `val` - The value to be stored in the node.
    ///
    /// # Returns
    ///
    /// A new `LinkedListNode` with the provided value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hym::nonnull_linked_list::LinkedListNode;
    ///
    /// let node = LinkedListNode::new(1);
    /// ```
    pub fn new(val: T) -> LinkedListNode<T> {
        LinkedListNode {
            value: val,
            next: None,
        }
    }

    /// Converts a raw pointer to a mutable reference of the node (unsafe operation).
    ///
    /// # Safety
    ///
    /// The caller must ensure the pointer is valid and not null.
    unsafe fn from_raw(ptr: NonNull<Self>) -> &'static mut Self {
        &mut *ptr.as_ptr()
    }
}

/// Error type for LinkedList.
///
/// # Errors
///
/// - RemoveWhileNextIsNone: The next node is `None`.
/// - InsertOutOfRange: An insert operation is out of range.
/// - RemoveOutOfRange: A remove operation is out of range.
/// - PopFromEmptyList: Trying to pop from an empty list.
/// - RemoveFromEmptyList: Trying to remove from an empty list.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LinkedListError {
    RemoveWhileNextIsNone,
    InsertOutOfRange,
    RemoveOutOfRange,
    PopFromEmptyList,
    RemoveFromEmptyList,
}

/// A linked list that supports common operations such as adding and removing elements by NonNull ptr.
///
/// # Attributes
///
/// * `len` - The length of the list.
/// * `head` - A reference to the first node in the list.
/// * `tail` - A reference to the last node in the list, used to optimize tail operations.
///
/// # Explanation
///
/// The `LinkedList` struct represents a linked list data structure. It contains the length of the list,
/// a reference to the first node in the list, and a reference to the last node in the list.
#[derive(Debug)]
pub struct LinkedList<T> {
    len: usize,
    head: Option<NonNull<LinkedListNode<T>>>,
    tail: Option<NonNull<LinkedListNode<T>>>,
    _marker: PhantomData<T>, // Used to handle covariance and drop check.
}

impl<T> LinkedList<T> {
    /// Creates a new empty linked list.
    ///
    /// # Returns
    ///
    /// * `Self` - An empty linked list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hym::nonnull_linked_list::LinkedList;
    ///
    /// let list = LinkedList::<u32>::new();
    /// assert_eq!(list.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            len: 0,
            head: None,
            tail: None,
            _marker: PhantomData,
        }
    }

    /// Inserts a new node with the given value at the beginning of the list.
    ///
    /// # Arguments
    ///
    /// * `val` - The value to be added to the beginning of the list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hym::nonnull_linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// list.push_head(1);
    /// list.push_head(2);
    /// assert_eq!(list.len(), 2);
    /// assert_eq!(format!("{}", list), "(2 -> 1)");
    /// ```
    ///
    /// # Complexity
    ///
    /// | Time Complexity | Space Complexity |
    /// |-----------------|------------------|
    /// | O(1)            | O(1)             |
    pub fn push_head(&mut self, val: T) {
        let mut node = Box::new(LinkedListNode::new(val));
        node.next = self.head;
        let node_ptr = NonNull::new(Box::into_raw(node));

        if let Some(old_head) = self.head {
            unsafe {
                (*node_ptr.unwrap().as_ptr()).next = Some(old_head);
            }
        } else {
            self.tail = node_ptr;
        }

        self.head = node_ptr;
        self.len += 1;
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
    /// use hym::nonnull_linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// list.push_back(1);
    /// list.push_back(2);
    /// assert_eq!(list.len(), 2);
    /// assert_eq!(format!("{}", list), "(1 -> 2)");
    /// ```
    ///
    /// # Complexity
    ///
    /// | Time Complexity | Space Complexity |
    /// |-----------------|------------------|
    /// | O(1)            | O(1)             |
    pub fn push_back(&mut self, val: T) {
        let node = Box::new(LinkedListNode::new(val));
        let node_ptr = NonNull::new(Box::into_raw(node));

        unsafe {
            if let Some(tail) = self.tail {
                (*tail.as_ptr()).next = node_ptr;
            } else {
                self.head = node_ptr;
            }
        }

        self.tail = node_ptr;
        self.len += 1;
    }

    /// Removes and returns the value from the beginning (head) of the list.
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
    /// use hym::nonnull_linked_list::LinkedList;
    ///
    /// let mut list: LinkedList<i32> = LinkedList::new();
    /// assert_eq!(list.pop_head(), Err(hym::LinkedListError::PopFromEmptyList));
    /// ```
    ///
    /// ```rust
    /// use hym::nonnull_linked_list::LinkedList;
    ///
    /// let mut list: LinkedList<i32> = LinkedList::new();
    /// list.push_head(1);
    /// list.push_head(2);
    /// list.push_head(3);
    /// assert_eq!(format!("{}", list), "(3 -> 2 -> 1)");
    /// assert_eq!(list.pop_head(), Ok(3));
    /// assert_eq!(format!("{}", list), "(2 -> 1)");
    /// ```
    ///
    /// # Complexity
    ///
    /// | Time Complexity | Space Complexity |
    /// |-----------------|------------------|
    /// | O(1)            | O(1)             |
    pub fn pop_head(&mut self) -> Result<T, LinkedListError> {
        match self.head {
            Some(head_ptr) => unsafe {
                let head = Box::from_raw(head_ptr.as_ptr());
                self.head = head.next;

                // If the list becomes empty, update the tail.
                if self.head.is_none() {
                    self.tail = None;
                }

                self.len -= 1;
                Ok(head.value)
            },
            None => Err(LinkedListError::PopFromEmptyList),
        }
    }

    /// Removes and returns the value from the end (tail) of the list.
    ///
    /// # Returns
    ///
    /// * `Ok(T)` - The value from the removed tail node.
    /// * `Err(LinkedListError)` - An error if the list is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hym::nonnull_linked_list::LinkedList;
    ///
    /// let mut list: LinkedList<i32> = LinkedList::new();
    /// list.push_back(1);
    /// list.push_back(2);
    /// assert_eq!(list.pop_back(), Ok(2));
    /// assert_eq!(list.pop_back(), Ok(1));
    /// assert_eq!(list.pop_back(), Err(LinkedListError::PopFromEmptyList));
    /// ```
    ///
    /// # Complexity
    ///
    /// | Time Complexity | Space Complexity |
    /// |-----------------|------------------|
    /// | O(n)            | O(1)             |
    pub fn pop_back(&mut self) -> Result<T, LinkedListError> {
        if self.len == 0 {
            return Err(LinkedListError::PopFromEmptyList);
        }

        if self.len == 1 {
            // If there's only one node, pop it and reset head and tail.
            let head_ptr = self.head.take().unwrap();
            self.tail = None;
            self.len = 0;
            unsafe {
                let head = Box::from_raw(head_ptr.as_ptr());
                Ok(head.value)
            }
        } else {
            // Traverse to the second-to-last node.
            let mut current = self.head;
            for _ in 0..self.len - 2 {
                unsafe {
                    current = current.unwrap().as_ref().next;
                }
            }

            unsafe {
                let tail_ptr = current.unwrap().as_mut().next.take().unwrap();
                self.tail = current;
                self.len -= 1;
                let tail = Box::from_raw(tail_ptr.as_ptr());
                Ok(tail.value)
            }
        }
    }

    /// Inserts a value at a specific index.
    ///
    /// # Arguments
    ///
    /// * `val` - The value to be inserted.
    /// * `at` - The index at which to insert the value.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the insertion is successful.
    /// * `Err(LinkedListError)` - If the index is out of range.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hym::nonnull_linked_list::LinkedList;
    ///
    /// let mut list: LinkedList<i32> = LinkedList::new();
    /// list.push_back(1);
    /// list.push_back(2);
    /// list.push_back(3);
    /// assert_eq!(list.insert(4, 1), Ok(()));
    /// assert_eq!(format!("{}", list), "(1 -> 4 -> 2 -> 3)");
    /// ```
    ///
    /// # Complexity
    ///
    /// | Time Complexity | Space Complexity |
    /// |-----------------|------------------|
    /// | O(n)            | O(1)             |
    pub fn insert(&mut self, val: T, at: usize) -> Result<(), LinkedListError> {
        if at > self.len {
            return Err(LinkedListError::InsertOutOfRange);
        }

        if at == 0 {
            self.push_head(val);
        } else if at == self.len {
            self.push_back(val);
        } else {
            let mut current = self.head;
            for _ in 0..at - 1 {
                unsafe {
                    current = current.unwrap().as_ref().next;
                }
            }

            unsafe {
                let node = Box::new(LinkedListNode::new(val));
                let node_ptr = NonNull::new(Box::into_raw(node));
                node_ptr.unwrap().as_mut().next = current.unwrap().as_ref().next;
                current.unwrap().as_mut().next = node_ptr;
            }

            self.len += 1;
        }

        Ok(())
    }

    /// Removes and returns the value at a specific index.
    ///
    /// # Arguments
    ///
    /// * `at` - The index of the value to remove.
    ///
    /// # Returns
    ///
    /// * `Ok(T)` - The value at the specified index.
    /// * `Err(LinkedListError)` - If the index is out of range.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hym::nonnull_linked_list::LinkedList;
    ///
    /// let mut list: LinkedList<i32> = LinkedList::new();
    /// list.push_back(1);
    /// list.push_back(2);
    /// list.push_back(3);
    /// assert_eq!(list.remove(1), Ok(2));
    /// assert_eq!(format!("{}", list), "(1 -> 3)");
    /// ```
    ///
    /// # Complexity
    ///
    /// | Time Complexity | Space Complexity |
    /// |-----------------|------------------|
    /// | O(n)            | O(1)             |
    pub fn remove(&mut self, at: usize) -> Result<T, LinkedListError> {
        if at >= self.len {
            return Err(LinkedListError::RemoveOutOfRange);
        }

        if at == 0 {
            self.pop_head()
        } else {
            let mut current = self.head;
            for _ in 0..at - 1 {
                unsafe {
                    current = current.unwrap().as_ref().next;
                }
            }

            unsafe {
                let node_to_remove = current.unwrap().as_mut().next.take().unwrap();
                current.unwrap().as_mut().next = node_to_remove.as_ref().next;

                if node_to_remove.as_ref().next.is_none() {
                    self.tail = current;
                }

                self.len -= 1;
                Ok(Box::from_raw(node_to_remove.as_ptr()).value)
            }
        }
    }

    /// Finds all indices of a given value in the list.
    ///
    /// # Arguments
    ///
    /// * `val` - The value to search for.
    ///
    /// # Returns
    ///
    /// * `Vec<usize>` - A vector of indices where the value is found.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hym::nonnull_linked_list::LinkedList;
    ///
    /// let mut list: LinkedList<i32> = LinkedList::new();
    /// list.push_back(1);
    /// list.push_back(2);
    /// list.push_back(1);
    /// assert_eq!(list.val2ix(&1), vec![0, 2]);
    /// ```
    ///
    /// # Complexity
    ///
    /// | Time Complexity | Space Complexity |
    /// |-----------------|------------------|
    /// | O(n)            | O(k)             | (k is the number of matches)
    pub fn val2ix(&self, val: &T) -> Vec<usize>
    where
        T: PartialEq,
    {
        let mut indices = Vec::new();
        let mut current = self.head;
        let mut index = 0;

        while let Some(node) = current {
            unsafe {
                if node.as_ref().value == *val {
                    indices.push(index);
                }
                current = node.as_ref().next;
                index += 1;
            }
        }

        indices
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
    /// use hym::nonnull_linked_list::LinkedList;
    ///
    /// let mut list: LinkedList<i32> = LinkedList::new();
    /// list.push_back(1);
    /// list.push_back(2);
    /// list.push_back(3);
    /// assert_eq!(list.ix2val(1), Some(2));
    /// assert_eq!(list.ix2val(3), None);
    /// ```
    ///
    /// # Complexity
    ///
    /// | Time Complexity | Space Complexity |
    /// |-----------------|------------------|
    /// | O(n)            | O(1)             |
    pub fn ix2val(&self, ix: usize) -> Option<T>
    where
        T: Clone,
    {
        if ix >= self.len {
            return None;
        }

        let mut current = self.head;
        for _ in 0..ix {
            unsafe {
                current = current.unwrap().as_ref().next;
            }
        }

        unsafe { Some(current.unwrap().as_ref().value.clone()) }
    }

    /// Returns the number of elements in the list.
    ///
    /// # Returns
    ///
    /// * `usize` - The number of elements in the list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hym::nonnull_linked_list::LinkedList;
    ///
    /// let mut list: LinkedList<i32> = LinkedList::new();
    /// list.push_back(1);
    /// list.push_back(2);
    /// assert_eq!(list.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.len
    }

    /// Checks if the list is empty.
    ///
    /// # Returns
    ///
    /// * `true` - If the list is empty.
    /// * `false` - If the list is not empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hym::nonnull_linked_list::LinkedList;
    ///
    /// let mut list: LinkedList<i32> = LinkedList::new();
    /// assert!(list.is_empty());
    /// list.push_back(1);
    /// assert!(!list.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Clears the list by removing all nodes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hym::nonnull_linked_list::LinkedList;
    ///
    /// let mut list: LinkedList<i32> = LinkedList::new();
    /// list.push_back(1);
    /// list.push_back(2);
    /// list.clean();
    /// assert!(list.is_empty());
    /// ```
    pub fn clean(&mut self) {
        while self.pop_head().is_ok() {}
    }

    /// Returns an iterator over the values in the list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hym::nonnull_linked_list::LinkedList;
    ///
    /// let list: LinkedList<i32> = LinkedList::from_iter(vec![1, 2, 3]);
    /// let mut iter = list.iter();
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> LinkedListBorrowIterator<T> {
        LinkedListBorrowIterator::new(self.head)
    }

    /// Returns a mutable iterator over the values in the list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hym::nonnull_linked_list::LinkedList;
    ///
    /// let mut list: LinkedList<i32> = LinkedList::from_iter(vec![1, 2, 3, 4, 5]);
    /// let mut iter = list.iter_mut(); // Create a mutable borrowed iterator for the linked list.
    ///
    /// assert_eq!(iter.next(), Some(&mut 1));
    /// assert_eq!(iter.next(), Some(&mut 2));
    /// assert_eq!(iter.next(), Some(&mut 3));
    /// assert_eq!(iter.next(), Some(&mut 4));
    /// assert_eq!(iter.next(), Some(&mut 5));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_mut(&mut self) -> LinkedListBorrowMutIterator<T> {
        LinkedListBorrowMutIterator::new(self.head)
    }

    pub fn get(&self, ix: usize) -> Option<T>
    where
        T: Clone,
    {
        if ix >= self.len {
            return None;
        }

        let mut current = self.head;
        for _ in 0..ix {
            unsafe {
                current = current.unwrap().as_ref().next;
            }
        }

        unsafe { Some(current.unwrap().as_ref().value.clone()) }
    }

    /// Creates a `LinkedList` from an iterator.
    ///
    /// # Arguments
    ///
    /// * `iter` - An iterator over values of type `T`.
    ///
    /// # Returns
    ///
    /// * `Self` - A new `LinkedList` containing the values from the iterator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hym::nonnull_linked_list::LinkedList;
    ///
    /// let list: LinkedList<i32> = LinkedList::from_iter(vec![1, 2, 3]);
    /// assert_eq!(list.len(), 3);
    /// assert_eq!(format!("{}", list), "(1 -> 2 -> 3)");
    /// ```
    pub fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut list = LinkedList::new();
        for item in iter {
            list.push_back(item);
        }
        list
    }
}

impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        let mut new_list = LinkedList::new();
        for item in self.iter() {
            new_list.push_back(item.clone());
        }
        new_list
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head;
        while let Some(node) = current {
            unsafe {
                let next = node.as_ref().next;
                let _ = Box::from_raw(node.as_ptr());
                current = next;
            }
        }
    }
}

impl<T: fmt::Display> fmt::Display for LinkedList<T> {
    /// Formats the list as a string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        let mut current = self.head;
        let mut first = true;

        while let Some(node_ptr) = current {
            unsafe {
                if !first {
                    write!(f, " -> ")?;
                }
                write!(f, "{}", node_ptr.as_ref().value)?;
                first = false;
                current = node_ptr.as_ref().next;
            }
        }

        write!(f, ")")
    }
}

/// Iterator for `LinkedList<T>`.
pub struct LinkedListIterator<T> {
    current: Option<NonNull<LinkedListNode<T>>>,
    _marker: PhantomData<T>, // Ensures the iterator is tied to the list's lifetime.
}

impl<T> LinkedListIterator<T> {
    /// Creates a new `LinkedListIterator` starting at the given node.
    fn new(head: Option<NonNull<LinkedListNode<T>>>) -> Self {
        Self {
            current: head,
            _marker: PhantomData,
        }
    }
}

impl<T: Clone> Iterator for LinkedListIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| unsafe {
            let node_ref = node.as_ref();
            self.current = node_ref.next;
            node_ref.value.clone()
        })
    }
}

impl<T: Clone> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = LinkedListIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        LinkedListIterator::new(self.head)
    }
}

/// Borrowed iterator for `LinkedList<T>`.
pub struct LinkedListBorrowIterator<'a, T> {
    current: Option<NonNull<LinkedListNode<T>>>,
    _marker: PhantomData<&'a T>, // Ensures the iterator is tied to the list's lifetime.
}

impl<'a, T> LinkedListBorrowIterator<'a, T> {
    /// Creates a new `LinkedListBorrowIterator` starting at the given node.
    fn new(head: Option<NonNull<LinkedListNode<T>>>) -> Self {
        Self {
            current: head,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for LinkedListBorrowIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| unsafe {
            let node_ref = node.as_ref();
            self.current = node_ref.next;
            &node_ref.value
        })
    }
}

/// Mutable borrowed iterator for `LinkedList<T>`.
pub struct LinkedListBorrowMutIterator<'a, T> {
    current: Option<NonNull<LinkedListNode<T>>>,
    _marker: PhantomData<&'a mut T>, // Ensures the iterator is tied to the list's lifetime.
}

impl<'a, T> LinkedListBorrowMutIterator<'a, T> {
    /// Creates a new `LinkedListBorrowMutIterator` starting at the given node.
    fn new(head: Option<NonNull<LinkedListNode<T>>>) -> Self {
        Self {
            current: head,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for LinkedListBorrowMutIterator<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|mut node| unsafe {
            let node_ref = node.as_mut();
            self.current = node_ref.next;
            &mut node_ref.value
        })
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
        list.push_head(1); // Add 1 to the head
        assert_eq!(list.len(), 1); // List should contain 1 element
        assert_eq!(list.get(0), Some(1)); // First element should be 1

        list.push_head(2); // Add 2 to the head
        assert_eq!(list.len(), 2); // List should now contain 2 elements
        assert_eq!(list.get(0), Some(2)); // First element should be 2
        assert_eq!(list.get(1), Some(1)); // Second element should be 1
    }

    #[test]
    fn test_push_back() {
        // Test adding elements to the back of the list
        let mut list = LinkedList::new();
        list.push_back(1); // Add 1 to the back
        assert_eq!(list.len(), 1); // List should contain 1 element
        assert_eq!(list.get(0), Some(1)); // First element should be 1

        list.push_back(2); // Add 2 to the back
        assert_eq!(list.len(), 2); // List should contain 2 elements
        assert_eq!(list.get(1), Some(2)); // Second element should be 2
    }

    #[test]
    fn test_pop_head() {
        // Test removing elements from the head of the list
        let mut list = LinkedList::new();
        assert_eq!(list.pop_head(), Err(LinkedListError::PopFromEmptyList)); // Pop on an empty list should return an error

        list.push_head(1); // Add 1 to the head
        list.push_head(2); // Add 2 to the head
        assert_eq!(list.pop_head(), Ok(2)); // Pop should return 2 (head element)
        assert_eq!(list.len(), 1); // List should now contain 1 element
        assert_eq!(list.pop_head(), Ok(1)); // Pop should return 1
        assert_eq!(list.len(), 0); // List should be empty
        assert_eq!(list.pop_head(), Err(LinkedListError::PopFromEmptyList)); // Pop on an empty list should return an error
    }

    #[test]
    fn test_pop_back() {
        // Test removing elements from the back of the list
        let mut list = LinkedList::new();
        assert_eq!(list.pop_back(), Err(LinkedListError::PopFromEmptyList)); // Pop on an empty list should return an error

        list.push_back(1); // Add 1 to the back
        list.push_back(2); // Add 2 to the back
        list.push_back(3); // Add 3 to the back
        assert_eq!(list.pop_back(), Ok(3)); // Pop should return 3 (last element)
        assert_eq!(list.len(), 2); // List should now contain 2 elements
        assert_eq!(list.pop_back(), Ok(2)); // Pop should return 2
        assert_eq!(list.len(), 1); // List should now contain 1 element
        assert_eq!(list.pop_back(), Ok(1)); // Pop should return 1
        assert_eq!(list.len(), 0); // List should be empty
        assert_eq!(list.pop_back(), Err(LinkedListError::PopFromEmptyList)); // Pop on an empty list should return an error
    }

    #[test]
    fn test_insert() {
        // Test inserting elements at a specific position
        let mut list = LinkedList::new();
        assert_eq!(list.insert(1, 1), Err(LinkedListError::InsertOutOfRange)); // Inserting out of range

        list.push_back(1); // Add 1 to the back
        list.push_back(3); // Add 3 to the back
        assert_eq!(list.insert(2, 1), Ok(())); // Insert 2 at position 1
        assert_eq!(list.len(), 3); // List should contain 3 elements
        assert_eq!(list.get(1), Some(2)); // Element at position 1 should be 2

        assert_eq!(list.insert(4, 3), Ok(())); // Insert 4 at position 3
        assert_eq!(list.len(), 4); // List should contain 4 elements
        assert_eq!(list.get(3), Some(4)); // Element at position 3 should be 4

        assert_eq!(list.insert(0, 0), Ok(())); // Insert 0 at position 0
        assert_eq!(list.len(), 5); // List should contain 5 elements
        assert_eq!(list.get(0), Some(0)); // Element at position 0 should be 0

        // Attempt to insert out of range
        assert_eq!(list.insert(5, 6), Err(LinkedListError::InsertOutOfRange)); // Inserting out of range should return an error
    }

    #[test]
    fn test_remove() {
        // Test removing elements at a specific position
        let mut list = LinkedList::new();
        assert_eq!(list.remove(0), Err(LinkedListError::RemoveFromEmptyList)); // Remove from an empty list should return an error

        list.push_back(1); // Add 1 to the back
        list.push_back(2); // Add 2 to the back
        list.push_back(3); // Add 3 to the back
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

        list.push_back(1); // Add 1 to the back
        list.push_back(2); // Add 2 to the back
        list.push_back(3); // Add 3 to the back
        list.push_back(2); // Add another 2 to the back

        assert_eq!(list.val2ix(&1), vec![0]); // 1 is at index 0
        assert_eq!(list.val2ix(&2), vec![1, 3]); // 2 is at indices 1 and 3
        assert_eq!(list.val2ix(&3), vec![2]); // 3 is at index 2
        assert_eq!(list.val2ix(&4), vec![]); // No 4 in the list
    }

    #[test]
    fn test_ix2val() {
        // Test accessing value by index
        let mut list = LinkedList::new();
        list.push_back(10); // Add 10 to the back
        list.push_back(20); // Add 20 to the back
        list.push_back(30); // Add 30 to the back

        assert_eq!(list.ix2val(0), Some(10)); // Element at index 0 should be 10
        assert_eq!(list.ix2val(1), Some(20)); // Element at index 1 should be 20
        assert_eq!(list.ix2val(2), Some(30)); // Element at index 2 should be 30
        assert_eq!(list.ix2val(3), None); // No element at index 3
    }

    #[test]
    fn test_get() {
        // Test retrieving element at a specific index
        let mut list = LinkedList::new();
        list.push_back(100); // Add 100 to the back
        list.push_back(200); // Add 200 to the back

        assert_eq!(list.get(0), Some(100)); // Element at index 0 should be 100
        assert_eq!(list.get(1), Some(200)); // Element at index 1 should be 200
        assert_eq!(list.get(2), None); // No element at index 2
    }

    #[test]
    fn test_len() {
        // Test the length of the list
        let mut list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.len(), 0); // Empty list

        list.push_head(1); // Add 1 to the head
        assert_eq!(list.len(), 1); // List should contain 1 element

        list.push_back(2); // Add 2 to the back
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

        list.push_back(1); // Add 1 to the back
        assert_eq!(format!("{}", list), "(1)");

        list.push_back(2); // Add 2 to the back
        list.push_back(3); // Add 3 to the back
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
        list.push_back(1); // Add 1 to the back
        list.push_back(2); // Add 2 to the back
        list.push_back(3); // Add 3 to the back

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
        list.push_back(1); // List: 1
        list.push_back(3); // List: 1 -> 3
        list.insert(2, 1).unwrap(); // List: 1 -> 2 -> 3
        list.insert(4, 3).unwrap(); // List: 1 -> 2 -> 3 -> 4
        list.insert(0, 0).unwrap(); // List: 0 -> 1 -> 2 -> 3 -> 4

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
        list.push_back(1); // Add 1 to the back
        list.push_back(2); // Add 2 to the back
        list.push_back(3); // Add 3 to the back
        assert_eq!(list.len(), 3);
        assert_eq!(format!("{}", list), "(1 -> 2 -> 3)");

        // Call clean and ensure the list is empty
        list.clean();
        assert_eq!(list.len(), 0);
        assert_eq!(format!("{}", list), "()");
    }

    #[test]
    fn test_from_iter() {
        // Test creating a list from a vector
        let list: LinkedList<i32> = LinkedList::from_iter(vec![]);
        assert_eq!(list.len(), 0); // Empty list
        assert_eq!(format!("{}", list), "()");

        let list = LinkedList::from_iter(vec![1, 2, 3]);
        assert_eq!(list.len(), 3); // List should contain 3 elements
        assert_eq!(format!("{}", list), "(1 -> 2 -> 3)");

        let list = LinkedList::from_iter(vec![1, 1, 1, 1]);
        assert_eq!(list.len(), 4); // List should contain 4 elements
        assert_eq!(format!("{}", list), "(1 -> 1 -> 1 -> 1)");

        let list = LinkedList::from_iter(vec![1, 1, 1, 1].into_iter());
        assert_eq!(list.len(), 4); // List should contain 4 elements
        assert_eq!(format!("{}", list), "(1 -> 1 -> 1 -> 1)");
    }

    #[test]
    fn test_into_iter() {
        let list: LinkedList<i32> = LinkedList::from_iter(vec![1, 2, 3, 4, 5, 6]);

        let it = list.into_iter(); // list is moved

        let vec = it.collect::<Vec<i32>>();

        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_is_empty() {
        let mut list = LinkedList::new();
        assert!(list.is_empty());
        list.push_back(1);
        assert!(!list.is_empty());
    }

    #[test]
    fn test_iter() {
        let list: LinkedList<i32> = LinkedList::from_iter(vec![1, 2, 3, 4, 5]);
        let mut iter = list.iter(); // create an borrowed iterator for linked list

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_mut() {
        let mut list: LinkedList<i32> = LinkedList::from_iter(vec![1, 2, 3, 4, 5]);
        let mut iter = list.iter_mut(); // create a mutable borrowed iterator for linked list

        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 4));
        assert_eq!(iter.next(), Some(&mut 5));
        assert_eq!(iter.next(), None);

        for val in list.iter_mut() {
            *val *= *val;
        }

        assert_eq!(format!("{}", list), "(1 -> 4 -> 9 -> 16 -> 25)");
    }
}
