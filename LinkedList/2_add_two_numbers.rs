// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut dummy_node = Some(Box::new(ListNode::new(0)));
        let mut temp = &mut dummy_node;
        let mut carry = 0;

        let (mut l1, mut l2) = (l1, l2);

        while l1.is_some() || l2.is_some() || carry != 0 {
            let l1_val = l1.as_ref().map_or(0, |node| node.val);
            let l2_val = l2.as_ref().map_or(0, |node| node.val);
            let sum_values = l1_val + l2_val + carry;

            carry = sum_values / 10;
            let new_node = Some(Box::new(ListNode::new(sum_values % 10)));
            temp.as_mut().unwrap().next = new_node;
            temp = &mut temp.as_mut().unwrap().next;

            l1 = l1.and_then(|node| node.next);
            l2 = l2.and_then(|node| node.next);
        }

        dummy_node.unwrap().next
        
    }
}

