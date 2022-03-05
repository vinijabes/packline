#[allow(dead_code)]
pub struct Queue<T> {
    head: Link<T>,
    tail: *mut Node<T>,

    len: usize,
}

#[allow(dead_code)]
type Link<T> = Option<Box<Node<T>>>;

#[allow(dead_code)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

#[allow(dead_code)]
impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            head: None,
            tail: std::ptr::null_mut(),
            len: 0,
        }
    }

    pub fn push(&mut self, elem: T) {
        let mut new_tail = Box::new(Node { elem, next: None });
        let raw_tail = &mut *new_tail as *mut Node<T>;

        self.len += 1;

        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        } else {
            self.head = Some(new_tail);
        }

        self.tail = raw_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;

            self.len -= 1;

            if self.head.is_none() {
                self.tail = std::ptr::null_mut();
            }

            head.elem
        })
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operations() {
        let mut queue: Queue<i32> = Queue::new();

        assert_eq!(queue.pop(), None);
        assert_eq!(queue.len(), 0);

        queue.push(3);
        queue.push(3);
        queue.push(0);
        queue.push(1);

        assert_eq!(queue.len(), 4);
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), Some(0));

        queue.push(42);

        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), Some(42));
        assert_eq!(queue.pop(), None);
        assert_eq!(queue.len(), 0);

        queue.push(1);
        queue.push(2);
        queue.push(3);

        assert_eq!(queue.len(), 3);
        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), None);
        assert_eq!(queue.len(), 0);
    }
}
