// use std::iter::FromIterator;

// pub struct SimpleLinkedList<T> {
//     head: Option<Box<Node<T>>>,
// }

// struct Node<T> {
//     data: T,
//     next: Option<Box<Node<T>>>,
// }

// impl<T> SimpleLinkedList<T> {
//     pub fn new() -> Self {
//         SimpleLinkedList { head: None }
//     }

//     // You may be wondering why it's necessary to have is_empty()
//     // when it can easily be determined from len().
//     // It's good custom to have both because len() can be expensive for some types,
//     // whereas is_empty() is almost always cheap.
//     // (Also ask yourself whether len() is expensive for SimpleLinkedList)
//     pub fn is_empty(&self) -> bool {
//         match &self.head {
//             None => true,
//             _ => false,
//         }
//     }

//     // fn add(mut self:&mut Self,value:i32){
//     //     while let Some(ref mut x)=self.next  {
//     //         self=x;
//     //     }
//     //     self.next=Some(Box::new(Self::new(value)));
//     // }

//     pub fn len(&self) -> usize {
//         0
//     }

//     pub fn push(&mut self, _element: T) {
//         unimplemented!()
//     }

//     pub fn pop(&mut self) -> Option<T> {
//         unimplemented!()
//     }

//     pub fn peek(&self) -> Option<&T> {
//         unimplemented!()
//     }

//     pub fn rev(self) -> SimpleLinkedList<T> {
//         unimplemented!()
//     }
// }

// impl<T> FromIterator<T> for SimpleLinkedList<T> {
//     fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
//         unimplemented!()
//     }
// }

// // In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// // instead of implementing an explicit conversion to a vector. This is because, together,
// // FromIterator and IntoIterator enable conversion between arbitrary collections.
// // Given that implementation, converting to a vector is trivial:
// //
// // let vec: Vec<_> = simple_linked_list.into_iter().collect();
// //
// // The reason this exercise's API includes an explicit conversion to Vec<T> instead
// // of IntoIterator is that implementing that interface is fairly complicated, and
// // demands more of the student than we expect at this point in the track.

// impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
//     fn into(self) -> Vec<T> {
//         unimplemented!()
//     }
// }

// Community solution 1
// use std::iter::FromIterator;

// struct Node<T> {
//     data: T,
//     next: Option<Box<Node<T>>>,
// }

// pub struct SimpleLinkedList<T> {
//     head: Option<Box<Node<T>>>,
// }

// impl<T> SimpleLinkedList<T> {
//     pub fn new() -> Self {
//         SimpleLinkedList { head: None }
//     }

//     // You may be wondering why it's necessary to have is_empty()
//     // when it can easily be determined from len().
//     // It's good custom to have both because len() can be expensive for some types,
//     // whereas is_empty() is almost always cheap.
//     // (Also ask yourself whether len() is expensive for SimpleLinkedList)
//     pub fn is_empty(&self) -> bool {
//         self.head.is_none()
//     }

//     pub fn len(&self) -> usize {
//         let mut count = 0;
//         let mut current = &self.head;
//         while let Some(c) = current {
//             current = &c.next;
//             count += 1;
//         }
//         count
//     }

//     pub fn push(&mut self, element: T) {
//         let new_node = Some(Box::new(Node {
//             data: element,
//             next: None,
//         }));
//         match self.head {
//             None => self.head = new_node,
//             _ => {
//                 let mut current = self.head.as_mut().unwrap().as_mut();
//                 while let Some(_) = &current.next {
//                     current = current.next.as_mut().unwrap().as_mut();
//                 }
//                 current.next = new_node;
//             }
//         }
//     }

//     pub fn pop(&mut self) -> Option<T> {
//         if self.head.is_none() {
//             return None;
//         }
//         let mut current = &mut self.head;
//         loop {
//             if current.as_ref().unwrap().next.is_none() {
//                 break;
//             } else {
//                 current = &mut current.as_mut().unwrap().next;
//             }
//         }
//         Some(current.take().unwrap().data)
//     }

//     pub fn peek(&self) -> Option<&T> {
//         if self.head.is_none() {
//             return None;
//         }
//         let mut current = &self.head;
//         loop {
//             if current.as_ref().unwrap().next.is_none() {
//                 break;
//             } else {
//                 current = &current.as_ref().unwrap().next;
//             }
//         }
//         Some(&current.as_ref().unwrap().data)
//     }

//     pub fn rev(mut self) -> SimpleLinkedList<T> {
//         let mut new = SimpleLinkedList::new();
//         loop {
//             match self.pop() {
//                 Some(v) => new.push(v),
//                 None => break,
//             }
//         }
//         new
//     }
// }

// impl<T> FromIterator<T> for SimpleLinkedList<T> {
//     fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
//         let mut sll = SimpleLinkedList::new();
//         for item in iter {
//             sll.push(item);
//         }
//         sll
//     }
// }

// // In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// // instead of implementing an explicit conversion to a vector. This is because, together,
// // FromIterator and IntoIterator enable conversion between arbitrary collections.
// // Given that implementation, converting to a vector is trivial:
// //
// // let vec: Vec<_> = simple_linked_list.into_iter().collect();
// //
// // The reason this exercise's API includes an explicit conversion to Vec<T> instead
// // of IntoIterator is that implementing that interface is fairly complicated, and
// // demands more of the student than we expect at this point in the track.

// impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
//     fn into(self) -> Vec<T> {
//         let mut new = Vec::new();
//         let mut reversed = self.rev();
//         loop {
//             match reversed.pop() {
//                 Some(v) => new.push(v),
//                 None => break,
//             }
//         }
//         new
//     }
// }

// // Community solution 2
// use std::iter::FromIterator;
// use std::mem;

// struct Node<T> {
//     data: T,
//     next: Option<Box<Node<T>>>,
// }

// pub struct SimpleLinkedList<T> {
//     head: Option<Box<Node<T>>>,
// }

// impl<T> SimpleLinkedList<T> {
//     pub fn new() -> Self {
//         Self { head: None }
//     }

//     // You may be wondering why it's necessary to have is_empty()
//     // when it can easily be determined from len().
//     // It's good custom to have both because len() can be expensive for some types,
//     // whereas is_empty() is almost always cheap.
//     // (Also ask yourself whether len() is expensive for SimpleLinkedList)
//     pub fn is_empty(&self) -> bool {
//         self.head.is_none()
//     }

//     pub fn len(&self) -> usize {
//         match &self.head {
//             None => 0,
//             Some(head) => {
//                 let mut result = 1;
//                 let mut node = head;
//                 while let Some(next) = &node.next {
//                     node = next;
//                     result += 1;
//                 }
//                 result
//             }
//         }
//     }

//     pub fn push(&mut self, element: T) {
//         self.head = Some(Box::new(Node {
//             data: element,
//             next: self.head.take(),
//         }));
//     }

//     pub fn pop(&mut self) -> Option<T> {
//         match self.head.take() {
//             None => None,
//             Some(node) => {
//                 self.head = node.next;
//                 Some(node.data)
//             }
//         }
//     }

//     pub fn peek(&self) -> Option<&T> {
//         self.head.as_ref().map(|node| &node.data)
//     }

//     pub fn rev(mut self) -> SimpleLinkedList<T> {
//         let mut prev = None;
//         loop {
//             match &mut self.head {
//                 None => break,
//                 Some(head) => {
//                     mem::swap(&mut head.next, &mut prev);
//                     if prev.is_none() {
//                         break;
//                     }
//                     mem::swap(&mut prev, &mut self.head);
//                 }
//             }
//         }
//         self
//     }
// }

// impl<T> FromIterator<T> for SimpleLinkedList<T> {
//     fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
//         let mut result = Self::new();
//         for item in iter {
//             result.push(item)
//         }
//         result
//     }
// }

// // In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// // instead of implementing an explicit conversion to a vector. This is because, together,
// // FromIterator and IntoIterator enable conversion between arbitrary collections.
// // Given that implementation, converting to a vector is trivial:
// //
// // let vec: Vec<_> = simple_linked_list.into_iter().collect();
// //
// // The reason this exercise's API includes an explicit conversion to Vec<T> instead
// // of IntoIterator is that implementing that interface is fairly complicated, and
// // demands more of the student than we expect at this point in the track.

// impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
//     fn into(mut self) -> Vec<T> {
//         let mut result = Vec::new();
//         self = self.rev();
//         while let Some(element) = self.pop() {
//             result.push(element);
//         }
//         result
//     }
// }

// Community solution 3
#![feature(box_patterns)]

use std::iter::FromIterator;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
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
        let mut node: &Option<Box<Node<T>>> = &self.head;
        let mut count = 0;

        while let Some(head) = &*node {
            count += 1;
            node = &head.next;
        }

        count
    }

    pub fn push(&mut self, _element: T) {
        self.head = Some(Box::new(Node {
            data: _element,
            next: self.head.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut popped: Option<T> = None;

        if let Some(box Node { data, next }) = self.head.take() {
            self.head = next;
            popped = Some(data);
        }

        popped
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut reversed: SimpleLinkedList<T> = SimpleLinkedList::new();
        let mut node: &Option<Box<Node<T>>> = &self.head;

        while let Some(head) = &*node {
            reversed.push(head.data.clone());
            node = &head.next;
        }

        reversed
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut elems: SimpleLinkedList<T> = SimpleLinkedList::new();
        for elem in _iter {
            elems.push(elem)
        }
        elems
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T: Clone> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut vec: Vec<T> = vec![];
        let mut node: &Option<Box<Node<T>>> = &self.head;

        while let Some(head) = &*node {
            vec.push(head.data.clone());
            node = &head.next;
        }
        vec.reverse();
        vec
    }
}
