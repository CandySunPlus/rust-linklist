type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

#[derive(Default)]
pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current_link = self.head.take();
        while let Some(mut boxed_node) = current_link {
            // Here, the `Node` in `next` has been moved out and dropped in advance,
            // so there will be no recursion when `boxed_node` is dropped.
            current_link = boxed_node.next.take();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn basics() {
        let mut list = List::default();
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn long_list() {
        let mut list = List::default();
        for i in 0..100000 {
            list.push(i);
        }
        drop(list);
    }
}
