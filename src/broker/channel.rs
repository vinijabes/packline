use std::collections::LinkedList;

pub trait Queue<T> {
    fn enqueue(&mut self, element: T);
    fn dequeue(&mut self) -> Option<T>;
}

pub struct Channel {
    queue: LinkedList<u32>
}

impl Queue<u32> for Channel {
    fn enqueue(&mut self, element: u32) {
        self.queue.push_back(element);
    }
    
    fn dequeue(&mut self) -> Option<u32> {
        return self.queue.pop_front();
    }
}

impl Channel {
    pub fn new() -> Channel {
        return Channel{
            queue: LinkedList::new()
        };
    }
}