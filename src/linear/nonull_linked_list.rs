use std::ptr::NonNull;

pub struct LinkedListNode<T> {
    value: T,
    next: Option<NonNull<LinkedListNode<T>>>,
}
