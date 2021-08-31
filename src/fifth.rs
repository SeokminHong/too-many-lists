use std::mem;

pub struct List<'a, T> {
    head: Link<T>,
    tail: Option<&'a mut Node<T>>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<'a, T> List<'a, T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push(&'a mut self, elem: T) {
        let new_tail = Box::new(Node { elem, next: None });

        self.tail = match self.tail.take() {
            Some(old_tail) => {
                old_tail.next = Some(new_tail);
                old_tail.next.as_deref_mut()
            }
            None => {
                self.head = Some(new_tail);
                self.head.as_deref_mut()
            }
        };
    }
}

impl<'a, T> Default for List<'a, T> {
    fn default() -> Self {
        List::new()
    }
}
