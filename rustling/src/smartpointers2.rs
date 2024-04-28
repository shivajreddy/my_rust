#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

struct ListNode<'a> {
    val: i32,
    next: Option<Box<&'a ListNode<'a>>>,
    // next: Option<ListNode>,
}

pub fn main() {
    let mut n1 = ListNode {
        val: 10,
        next: None,
    };
    let mut n2 = ListNode {
        val: 20,
        next: None,
    };
    let mut n3 = ListNode {
        val: 30,
        next: None,
    };

    let n1 = Box::new(ListNode {
        val: 10,
        next: None,
    });
    let n2 = Box::new(ListNode {
        val: 20,
        next: None,
    });
    let n3 = Box::new(ListNode {
        val: 30,
        next: None,
    });

    n1.next = Some(&n2);
    // n1.next = Some(Box::new(n2));
    n2.next = Some(Box::new(n3));

    print_linked_list(&n1);
}

pub fn print_linked_list(root: &ListNode) {
    println!();
}
