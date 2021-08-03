use std::iter::FromIterator;

type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    pub next: Link<T>,
    pub data: T,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Node {
            data,
            next: None,
        }
    }
}

#[derive(Default)]
pub struct SimpleLinkedList<T> {
    head: Link<T>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None
        }
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
        let mut l = 0;
        let mut node = &self.head;
        while let Some(n) = node {
            l += 1;
            node = &n.next;
        }
        l
    }

    pub fn push(&mut self, element: T) {
        let new_node = Node {
            next: self.head.take(),
            data: element,
        };
        self.head = Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            self.head = head.next;
            head.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|head| &head.data)
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut s = SimpleLinkedList::new();
        let mut node = self.head;
        while let Some(n) = node.take() {
            s.push(n.data);
            node = n.next;
        }
        s
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut s = SimpleLinkedList::new();
        for n in iter {
            s.push(n);
        }
        s
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

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut v: Vec<T> = vec![];
        let mut node = self.head;
        while let Some(n) = node {
            v.push(n.data);
            node = n.next;
        }
        v.reverse();
        v
    }
}
