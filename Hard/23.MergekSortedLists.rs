use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq)]
struct HeapNode {
    val: i32,
    node: Box<ListNode>,
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering to make BinaryHeap work as a min-heap
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HeapNode {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl Solution {
    pub fn merge_k_lists(
        lists: Vec<Option<Box<ListNode>>>,
    ) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();

        for list in lists {
            if let Some(node) = list {
                heap.push(HeapNode {
                    val: node.val,
                    node,
                });
            }
        }

        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        while let Some(mut current) = heap.pop() {
            if let Some(next) = current.node.next.take() {
                heap.push(HeapNode {
                    val: next.val,
                    node: next,
                });
            }

            tail.next = Some(current.node);
            tail = tail.next.as_mut().unwrap();
        }

        dummy.next
    }
}