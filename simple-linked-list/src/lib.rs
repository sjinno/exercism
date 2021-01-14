use std::iter::FromIterator;
// use std::mem;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Default for SimpleLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut curr = &self.head;
        while let Some(node) = curr {
            curr = &node.next;
            count += 1;
        }
        count
    }

    pub fn push(&mut self, element: T) {
        self.head = Some(Box::new(Node {
            data: element,
            next: self.head.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(node) = self.head.take() {
            self.head = node.next;
            return Some(node.data);
        }
        None
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {
        // let mut prev = None;
        // while let Some(head) = &mut self.head {
        //     mem::swap(&mut head.next, &mut prev);
        //     if prev.is_none() {
        //         break;
        //     }
        //     mem::swap(&mut prev, &mut self.head);
        // }
        // self
        let mut prev = None;
        while let Some(mut curr) = self.head.take() {
            let next = curr.next.take();
            curr.next = prev;
            prev = Some(curr);
            self.head = next;
        }
        self.head = prev;
        self
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut sll = Self::new();
        for item in iter {
            sll.push(item);
        }
        sll
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(sll: SimpleLinkedList<T>) -> Self {
        let mut v = Vec::new();
        let mut reversed = sll.rev();
        while let Some(node) = reversed.head.take() {
            reversed.head = node.next;
            v.push(node.data);
        }
        v
    }
}

// impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
//     fn into(self) -> Vec<T> {
//         let mut v = Vec::new();
//         let mut reversed = self.rev();
//         while let Some(node) = reversed.head.take() {
//             reversed.head = node.next;
//             v.push(node.data);
//         }
//         v
//     }
// }
