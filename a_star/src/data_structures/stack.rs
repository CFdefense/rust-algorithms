/*
    Stack
        storage representation can be your choice of contiguous array or linked nodes

    Note
        Each class must have a static function that performs a unit test of the class by instantiating and and calling the methods of the class.
*/

#[derive(PartialEq)]
struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    /// new()
    ///
    /// Creates a new instance of a Node.
    ///
    /// Returns an instance of the Node.
    ///
    pub fn new(value: T, next: Option<Box<Node<T>>>) -> Self {
        Node { value, next }
    }
}

pub struct Stack<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> Stack<T> {
    /// new()
    ///
    /// Create a new instance of a Stack.
    ///
    /// Returns an instance of the Stack.
    ///
    pub fn new() -> Self {
        Stack { head: None }
    }

    /// pop()
    ///
    /// Update the head to the heads next node.
    /// Return the original head.
    ///
    /// Returns an optional top element.
    ///
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|boxed_node| {
            let node = *boxed_node; // unbox Node<T>
            self.head = node.next; // update head to next
            node.value // return old value
        })
    }

    /// push()
    ///
    /// Add a new node to the head.
    /// Set the previous node as the next.
    ///
    /// Returns None.
    ///
    pub fn push(&mut self, value: T) {
        // Take the previous head
        let old = self.head.take();

        // Create a new node
        let new = Box::new(Node::new(value, old));

        // Update the head
        self.head = Some(new);
    }

    /// len()
    ///
    /// Counts the total elements in the Stack.
    /// Does so by traversing the linked-list of nodes and counting them.
    ///
    /// Returns the count of elements.
    ///
    pub fn len(&self) -> u32 {
        let mut count = 0;
        let mut cur = self.head.as_deref();

        // While the next node is not none
        while let Some(node) = cur {
            cur = node.next.as_deref();
            count += 1;
        }

        count
    }

    /// peek()
    ///
    /// Peek at the top (head) element of the Stack.
    ///
    /// Returns an optional element from the top of the Stack.
    ///
    pub fn peek(&self) -> Option<&T> {
        self.head.as_deref().map(|node| &node.value)
    }

    /// is_empty()
    ///
    /// Check if the Stack is empty.
    ///
    /// Returns true if empty, false otherwise.
    ///
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        // Create a new stack
        let mut stack = Stack::<u32>::new();

        // Initially, stack should be empty and have length 0
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.is_empty(), true);

        // Push values onto the stack
        stack.push(1);
        stack.push(2);
        stack.push(3);

        // Pop values and check LIFO behavior
        assert_eq!(stack.pop(), Some(3)); // Last in, first out
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));

        // Stack is now empty, popping should return None
        assert_eq!(stack.pop(), None);

        // Confirm stack length and empty state
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.is_empty(), true);

        // Push another value to test reusability
        stack.push(1);

        // Length should reflect new element, stack is no longer empty
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.is_empty(), false);

        // Pop the last element
        assert_eq!(stack.pop(), Some(1));
    }
}
