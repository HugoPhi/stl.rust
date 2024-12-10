mod linked_list;

pub use linked_list::LinkedList;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_pop() {
        let mut list = LinkedList::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_len() {
        let mut list = LinkedList::new();
        assert_eq!(list.len(), 0);

        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.len(), 3);

        list.pop();
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_is_empty() {
        let mut list = LinkedList::new();
        assert!(list.is_empty());

        list.push(1);
        assert!(!list.is_empty());

        list.pop();
        assert!(list.is_empty());
    }
}
