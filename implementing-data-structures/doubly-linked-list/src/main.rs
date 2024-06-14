//-----------------------------------------------------------
//                  Doubly Linked List
//-----------------------------------------------------------

use std::{cell::RefCell, rc::Rc};

type Pointer = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
struct Node {
    element: i32,
    next: Pointer,
    previous: Pointer,
}

#[derive(Debug)]
struct DoublyLinkList {
    head: Pointer,
    tail: Pointer,
}

impl Node {
    fn new(element: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            element: element,
            next: None,
            previous: None,
        }))
    }
}

impl DoublyLinkList {
    fn new() -> Self {
        // Self = DoublyLinkList type (function)     self = instance of DoublyLinkList type (method)
        DoublyLinkList {
            head: None,
            tail: None,
        }
    }

    fn add(&mut self, element: i32) -> () {
        let new_head = Node::new(element);

        match self.head.take() {
            // take function returns the value of the head if any and replaces it with None
            Some(old_head) => {
                old_head.borrow_mut().previous = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head.clone());

                self.head = Some(new_head);
            }

            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    fn remove(&mut self) -> Option<i32> {
        if self.head.is_none() {
            println!("The list is empty. No value to be removed.");
            None
        } else {
            let removed_value = self.head.as_ref().unwrap().borrow().element;

            self.head
                .take()
                .map(|old_head| match old_head.borrow_mut().next.take() {
                    Some(new_head) => {
                        new_head.borrow_mut().previous = None;
                        self.head = Some(new_head);
                        self.head.clone()
                    }
                    None => {
                        self.tail = None;
                        println!("The list is empty after removal");
                        None
                    }
                });

            Some(removed_value)
        }
    }

    fn print(&self) {
        let mut traversal = self.head.clone();
        while !traversal.is_none() {
            println!("{:?}", traversal.as_ref().unwrap().borrow().element);
            traversal = traversal.unwrap().borrow().next.clone();
        }
    }
}

fn main() {
    println!("--------------------------------");

    let mut doubly_link_list = DoublyLinkList::new();
    println!("{:#?}", doubly_link_list);

    println!("--------------------------------");

    doubly_link_list.add(5);
    doubly_link_list.add(14);
    doubly_link_list.add(2);
    doubly_link_list.add(9);

    doubly_link_list.print();

    println!("--------------------------------");

    doubly_link_list.remove();

    doubly_link_list.print();
}
