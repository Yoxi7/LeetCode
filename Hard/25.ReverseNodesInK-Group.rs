impl Solution {
    pub fn reverse_k_group(
        head: Option<Box<ListNode>>,
        k: i32,
    ) -> Option<Box<ListNode>> {
        let k = k as usize;

        
        let mut p = &head;
        for _ in 0..k {
            match p {
                Some(node) => p = &node.next,
                None => return head,
            }
        }

        let mut curr = head;
        let mut prev = None;

        for _ in 0..k {
            let mut node = curr.unwrap();
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            curr = next;
        }

        let rest = Self::reverse_k_group(curr, k as i32);

        let mut tail = &mut prev;
        while let Some(node) = tail {
            if node.next.is_none() {
                node.next = rest;
                break;
            }
            tail = &mut node.next;
        }

        prev
    }
}