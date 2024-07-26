use std::ptr;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = *mut Node<T>;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    pub fn push(&mut self, elem: T) {
        let new_tail = Box::into_raw(Box::new(Node {
            elem,
            next: ptr::null_mut(),
        }));

        if self.tail.is_null() {
            self.head = new_tail;
        } else {
            unsafe {
                (*self.tail).next = new_tail;
            }
        }

        self.tail = new_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_null() {
            None
        } else {
            let head = unsafe { Box::from_raw(self.head) };
            self.head = head.next;

            if head.next.is_null() {
                self.head = ptr::null_mut();
                self.tail = ptr::null_mut();
            } else {
                self.head = head.next;
            }

            Some(head.elem)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        // Check the exhaustion case fixed the pointer right
        list.push(6);
        list.push(7);

        // Check normal removal
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }
}
