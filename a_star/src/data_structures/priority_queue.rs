/*
   PriorityQueue
        based on a min-binary heap

    Min-binary Heap
        A min-binary heap is a complete binary tree where each parent is ≤ its children.
        parent(i)     = (i - 1) / 2
        left(i)       = 2*i + 1
        right(i)      = 2*i + 2

    Note
        Each class must have a static function that performs a unit test of the class by instantiating and and calling the methods of the class.
*/

pub struct PriorityQueue<T> {
    heap: MinHeap<T>,
}

impl<T: Default + PartialOrd> PriorityQueue<T> {
    /// new()
    ///
    /// Creates a new instance of a Priority Queue.
    ///
    /// Returns an instance of the Priority Queue.
    ///
    pub fn new() -> Self {
        PriorityQueue {
            heap: MinHeap { data: Vec::new() },
        }
    }

    /// len()
    ///
    /// Returns the length of the priority queue.
    ///
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    /// is_empty()
    ///
    /// Returns True if the priority queue is empty, False otherwise.
    ///
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// push()
    ///
    /// Pushes a new value to the Queue.
    ///
    pub fn push(&mut self, value: T) {
        self.heap.push(value)
    }

    /// pop()
    ///
    /// Pop a value from the Queue.
    ///
    /// Returns an optional element from the back of the Queue.
    ///
    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop()
    }

    /// peek()
    ///
    /// Peek at the back element of the Queue.
    ///
    /// Returns an optional element from the back of Queue.
    ///
    pub fn peek(&self) -> Option<&T> {
        self.heap.peek()
    }
}

struct MinHeap<T> {
    data: Vec<T>,
}

impl<T: Default + PartialOrd> MinHeap<T> {
    /// new()
    ///
    /// Creates a new instance of a Min-Heap.
    ///
    /// Returns an instance of the Min-Heap.
    ///
    pub fn new() -> Self {
        MinHeap { data: Vec::new() }
    }

    /// len()
    ///
    /// Calculate the length of the Min-Heap.
    ///
    /// Returns the size of the Min-Heap vector.
    ///
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// is_empty()
    ///
    /// Returns True if the Min-Heap is empty, False otherwise.
    ///
    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    /// peek()
    ///
    /// Peek at the Front element of the Min-Heap.
    ///
    /// Returns an optional element from the Front of the Min-Heap.
    ///
    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    /// push()
    ///
    /// When pushing to a min-binary heap we want to simply put the element at the back.
    /// Then we will fix up the heap to maintain the min-heap property.
    ///
    /// Returns None.
    ///
    pub fn push(&mut self, value: T) {
        self.data.push(value);
        self.bubble_up();
    }

    /// pop()
    ///
    /// When popping to a min-binary heap we want to swap the back element with front.
    /// Pop the back element.
    /// Then Bubble down to fix up the array.
    ///
    /// Returns the min element from the Min-Heap.
    ///
    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }

        // Get the last
        let last = self.data.len() - 1;

        // Swap the front and the back
        self.data.swap(0, last);

        // Remove the back element
        let min = self.data.pop();

        // Bubble down
        if !self.data.is_empty() {
            self.bubble_down();
        }

        min
    }

    /// bubble_up()
    ///
    /// Fix-up operation to move a newly inserted element to its proper location.
    /// Will compare against the parent elements to find its proper place.
    ///
    /// Stop when the Min-Heap Property is restored.
    ///
    /// Returns None.
    ///
    fn bubble_up(&mut self) {
        let mut i = self.data.len() - 1;
        while i > 0 {
            let parent = (i - 1) / 2;

            // Compare the element to the current parent
            if self.data[i] < self.data[parent] {
                self.data.swap(i, parent);
                i = parent;
            } else {
                break; // Min-heap property restored
            }
        }
    }

    /// bubble_down()
    ///
    /// Fix-up operation that occurs when the root of the tree is deleted.
    /// Move the last element to the front and swap it down with the smaller of the children.
    ///
    /// Returns None.
    ///
    fn bubble_down(&mut self) {
        let (mut i, n) = (0, self.data.len());
        loop {
            let left = 2 * i + 1;
            let right = 2 * i + 2;

            // find smallest child
            let mut smallest = i;

            if left < n && self.data[left] < self.data[smallest] {
                smallest = left;
            }

            if right < n && self.data[right] < self.data[smallest] {
                smallest = right;
            }

            // If no swap needed, heap property is restored
            if smallest == i {
                break;
            }

            // Swap and continue
            self.data.swap(i, smallest);
            i = smallest;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_queue() {
        // Create a new Priority Queue
        let mut priority_queue = PriorityQueue::<u32>::new();

        // Push multiple values into the priority queue
        priority_queue.push(1);
        priority_queue.push(2);
        priority_queue.push(3);
        priority_queue.push(4);
        priority_queue.push(10);
        priority_queue.push(9);
        priority_queue.push(6);
        priority_queue.push(5);

        // Pop elements one by one and check that they come out in ascending order
        assert_eq!(priority_queue.pop(), Some(1)); // smallest element
        assert_eq!(priority_queue.pop(), Some(2));
        assert_eq!(priority_queue.pop(), Some(3));
        assert_eq!(priority_queue.pop(), Some(4));

        // Check length after some pops
        assert_eq!(priority_queue.len(), 4);

        // Continue popping
        assert_eq!(priority_queue.pop(), Some(5));
        assert_eq!(priority_queue.pop(), Some(6));

        // Length should now be 2
        assert_eq!(priority_queue.len(), 2);

        // Pop remaining elements
        assert_eq!(priority_queue.pop(), Some(9));
        assert_eq!(priority_queue.pop(), Some(10));

        // The queue should now be empty
        assert_eq!(priority_queue.is_empty(), true);
    }

    #[test]
    fn test_min_heap() {
        // Create a new Min-Heap
        let mut min_heap = MinHeap::<u32>::new();

        // Push multiple values into the min-heap
        min_heap.push(1);
        min_heap.push(2);
        min_heap.push(3);
        min_heap.push(4);
        min_heap.push(10);
        min_heap.push(9);
        min_heap.push(6);
        min_heap.push(5);

        // Pop elements one by one and check that they come out in ascending order
        assert_eq!(min_heap.pop(), Some(1));
        assert_eq!(min_heap.pop(), Some(2));
        assert_eq!(min_heap.pop(), Some(3));
        assert_eq!(min_heap.pop(), Some(4));

        // Check length after some pops
        assert_eq!(min_heap.len(), 4);

        // Continue popping
        assert_eq!(min_heap.pop(), Some(5));
        assert_eq!(min_heap.pop(), Some(6));

        // Length should now be 2
        assert_eq!(min_heap.len(), 2);

        // Pop remaining elements
        assert_eq!(min_heap.pop(), Some(9));
        assert_eq!(min_heap.pop(), Some(10));

        // The heap should now be empty
        assert_eq!(min_heap.is_empty(), true);
    }
}
