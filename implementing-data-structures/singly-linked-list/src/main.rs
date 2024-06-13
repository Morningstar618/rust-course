//-------------------------------------------
//          Link List (Part 1)
//-------------------------------------------

type Pointer = Option<Box<Node>>;

#[derive(Debug)]
struct Node {
    element: i32,
    next: Pointer,
}

#[derive(Debug)]
struct LinkList {
    head: Pointer,
}

impl LinkList {
    fn new() -> LinkList {
        LinkList { head: None }
    }

    fn add(&mut self, element: i32) {
        let previous_head = self.head.take();
        let new_head = Some(Box::new(Node {
            element: element,
            next: previous_head,
        }));
        self.head = new_head;
    }

    fn remove(&mut self) -> Option<i32> {
        match self.head.take() {
            Some(previous_head) => {
                self.head = previous_head.next;
                Some(previous_head.element)
            }
            None => None,
        }
    }

    fn print(&self) {
        let mut list_traversal = &self.head;
        while !list_traversal.is_none() {
            println!("{:?}", list_traversal.as_ref().unwrap().element);
            list_traversal = &list_traversal.as_ref().unwrap().next;
        }
    }
}

fn main() {
    let mut list = LinkList::new();

    list.add(1);
    list.add(2);
    list.add(3);
    list.print();

    println!("Removed: {:?}", list.remove().unwrap());
    list.print();
}
