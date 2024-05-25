use std::collections::HashMap;

struct Node {
    key: i32,
    value: i32,
    next: Option<Box<Node>>,
    prev: Option<Box<Node>>,
}

struct DoublyLinkedList {
    size: i32,
    head: Node,
    tail: Node,
}

impl DoublyLinkedList {
    fn new() -> Self {
        DoublyLinkedList {
            size: 0,
            head: Node {
                key: -1,
                value: -1,
                next: None,
                prev: None,
            },
            tail: Node {
                key: -1,
                value: -1,
                next: None,
                prev: None,
            },
        }
    }

    fn insert_front(&mut self, mut new_node: Node) {
        // head <==> first <==> ... <==> tail
        // (first points to tail, if list is empty)

        let mut first_node: Option<Box<Node>> = self.head.next;

        // head <=> new_node
        self.head.next = Some(Box::new(new_node));
        new_node.prev = Some(Box::new(self.head));

        // new_node <==> first_node
        new_node.next = first_node;
        first_node.unwrap().next = Some(Box::new(new_node));

        self.size += 1;
    }

    fn remove_last(&mut self) -> Option<Node> {
        // Need atleast 1 node to delete
        if self.size == 0 {
            return None;
        }

        // head <==> ... last_prev <==> last <==> tail
        // take out the last node, replacing it with None
        let mut last = self.tail.prev.take();

        if let Some(mut last_node) = last {
            // last_prev -> last <- tail
            last_node.prev = None;
            last_node.next = None;

            // last_prev <==> tail | last
            if let Some(mut last_prev) = last_node.prev.take() {
                last_prev.next = Some(Box::new(self.tail));
                self.tail.prev = Some(last_prev);
            }
            self.size -= 1;
            return Some(*last_node);
        }
        None
    }

    fn remove_node(&mut self, mut target_node: Node) -> Node {
        // head <==> .. <==> target <==> .. tail

        let mut target_prev = target_node.prev.take();
        let mut target_next = target_node.next.take();

        // target_prev -> target <- target_next
        target_node.prev = None;
        target_node.next = None;

        // target_prev <==> target_next  |  target
        if let Some(mut node) = target_prev {
            node.next = target_next;
        }
        if let Some(mut node) = target_next {
            node.prev = target_prev;
        }
        self.size -= 1;
        target_node
    }
}

struct LRUCache {
    capacity: i32,
    cache: DoublyLinkedList,
    mapper: HashMap<i32, Node>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity,
            cache: DoublyLinkedList::new(),
            mapper: HashMap::new(),
        }
    }

    fn get(&self, key: i32) -> i32 {
        if !self.mapper.contains_key(&key) {
            return -1;
        }

        let target_node = self.mapper.get(&key);

        if let Some(node) = target_node {
            // remove from the list
            self.cache.remove_node(*node);

            // add to front, since recently accessed

            return (*node).value; // deref the box item
        }
        -1
    }

    fn put(&self, key: i32, value: i32) {}
}
