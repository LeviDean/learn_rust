use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
enum Data {
    Int(i32),
    Char(char),
    Float(f32),
    Str(String),
}

#[derive(Debug)]
struct ListNode{
    val: Data,
    next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    fn new(val: Data) -> Rc<RefCell<ListNode>> {
        Rc::new(RefCell::new(ListNode { val, next: None }))
    }

    fn find_and_remove(head: Option<Rc<RefCell<ListNode>>>, val: Data) -> Option<Rc<RefCell<ListNode>>> {
        let mut head_node = head;
        let mut prev_node: Option<Rc<RefCell<ListNode>>> = None;
        let mut curr_node = head.clone();

        while let Some(curr) = curr_node {
        if curr.borrow().val == val {
            if let Some(prev) = prev_node {
                prev.borrow_mut().next = curr.borrow_mut().next.clone();
            } else {
                head_node = curr.borrow_mut().next.clone();
            }
        } else {
            prev_node = curr_node;
            curr_node = curr_node.borrow_mut().next.clone();
        }
        }
        head_node
    }
}

fn main() {
    let n0 = ListNode::new(Data::Int(1));
    let n1 = ListNode::new(Data::Char('A'));
    let n2 = ListNode::new(Data::Float(3.1));
    let n3 = ListNode::new(Data::Str(String::from("Hello")));

    // build the linked list
    n0.borrow_mut().next = Some(n1.clone());
    n1.borrow_mut().next = Some(n2.clone());
    n2.borrow_mut().next = Some(n3.clone());

    // delete the second node

}
