# Box Based Linked List

有了[上一节](./rc_linked_list.md)的铺垫，本节我们需要完成的事情相对而言就少得多。上一节中提到`Rc`链表虽然带来了$O(1)$级别的尾部插入操作，但是也让一些比较频繁的正常操作变得繁重与缓慢，其原因主要在于`Rc`的引用计数机制导致它产生了巨大的额外开销。为了避免这种开销，我们选择了另一种比较轻量智能指针`Box`指针来制作链表，其节点数据结构如下：  

```rust
pub struct LinkedListNode<T> {
    value: T,
    next: Option<Box<LinkedListNode<T>>>,
}
```

## 2. 如何建立LinkedList的数据结构


