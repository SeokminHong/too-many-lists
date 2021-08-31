use std::mem;
use std::ptr;

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: *mut Node<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Node {
            elem,
            next: None,
            prev: ptr::null_mut(),
        }
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: ptr::null_mut(),
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let mut new_head = Box::new(Node::new(elem));
        let raw_head = &mut *new_head;

        match self.head.take() {
            Some(mut old_head) => {
                old_head.prev = raw_head;
                new_head.next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = raw_head;
                self.head = Some(new_head);
            }
        }
    }

    pub fn push_back(&mut self, elem: T) {
        let mut new_tail = Box::new(Node::new(elem));
        let raw_tail: *mut _ = &mut *new_tail;

        if self.tail.is_null() {
            self.head = Some(new_tail);
        } else {
            new_tail.prev = self.tail;
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        }

        self.tail = raw_tail;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|mut old_head| {
            match old_head.next.take() {
                Some(mut new_head) => {
                    new_head.prev = ptr::null_mut();
                    self.head = Some(new_head);
                }
                None => {
                    if !self.tail.is_null() {
                        self.tail = ptr::null_mut();
                    }
                }
            };
            old_head.elem
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.tail.is_null() {
            None
        } else {
            let prev = unsafe { (*self.tail).prev };
            // If the list has only one element.
            if prev.is_null() {
                self.tail = ptr::null_mut();
                Some(self.head.take().unwrap().elem)
            } else {
                let new_tail = unsafe { &mut *prev };
                let old_tail = mem::replace(&mut new_tail.next, None);
                self.tail = new_tail;

                Some(old_tail.unwrap().elem)
            }
        }
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn front() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    pub fn back() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_back(), None);

        // Populate list
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_back(4);
        list.push_back(5);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }
}
