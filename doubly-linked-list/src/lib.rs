use std::ptr::NonNull;

// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;

type OptionLink<T> = Option<NonNull<Node<T>>>;
type Link<T> = NonNull<Node<T>>;

pub struct LinkedList<T> {
    len: usize,
    front_ptr: OptionLink<T>,
    back_ptr: OptionLink<T>,
}

pub struct Cursor<'a, T> {
    list: &'a mut LinkedList<T>,
    node_ptr: OptionLink<T>,
}

pub struct Node<T> {
    prev_ptr: OptionLink<T>,
    next_ptr: OptionLink<T>,
    elem: T,
}

impl<T> Node<T> {
    pub unsafe fn link(mut left: Link<T>, mut right: Link<T>) {
        left.as_mut().next_ptr = Some(right);
        right.as_mut().prev_ptr = Some(left);
    }
}

pub struct Iter<'a, T> {
    next_node: OptionLink<T>,
    marker: std::marker::PhantomData<&'a LinkedList<T>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            len: 0,
            front_ptr: None,
            back_ptr: None,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for LinkedList)
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<T> {
        Cursor {
            node_ptr: self.front_ptr,
            list: self,
        }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<T> {
        Cursor {
            node_ptr: self.back_ptr,
            list: self,
        }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next_node: self.front_ptr,
            marker: std::marker::PhantomData,
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut cursor = self.cursor_front();
        while let Some(_) = cursor.take() {}
    }
}

unsafe impl<T: Send> Send for LinkedList<T> {}

unsafe impl<T: Sync> Sync for LinkedList<T> {}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe { self.node_ptr.map(|node| &mut (*node.as_ptr()).elem) }
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        unsafe {
            self.node_ptr.and_then(|current| {
                (*current.as_ptr())
                    .next_ptr
                    .map(|next| {
                        self.node_ptr = Some(next);
                        &mut (*next.as_ptr()).elem
                    })
            })
        }
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        unsafe {
            self.node_ptr.and_then(|current| {
                (*current.as_ptr())
                    .prev_ptr
                    .map(|prev| {
                        self.node_ptr = Some(prev);
                        &mut (*prev.as_ptr()).elem
                    })
            })
        }
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        unsafe {
            let mut node = self.node_ptr?;
            let &mut Node {
                prev_ptr, next_ptr, ..
            } = node.as_mut();
            self.node_ptr = next_ptr.or(prev_ptr);

            match next_ptr {
                Some(mut next) => next.as_mut().prev_ptr = prev_ptr,
                None => self.list.back_ptr = prev_ptr,
            }

            match prev_ptr {
                Some(mut prev) => prev.as_mut().next_ptr = next_ptr,
                None => self.list.front_ptr = next_ptr,
            }

            self.list.len -= 1;

            Some(Box::from_raw(node.as_ptr()).elem)
        }
    }

    pub fn insert_after(&mut self, elem: T) {
        unsafe {
            self._insert(
                elem,
                |list| &mut list.back_ptr,
                |mut current, new| {
                    if let Some(next) = current.as_mut().next_ptr {
                        Node::link(new, next);
                    }

                    Node::link(current, new);
                },
            )
        }
    }

    pub fn insert_before(&mut self, elem: T) {
        unsafe {
            self._insert(
                elem,
                |list| &mut list.front_ptr,
                |mut current, new| {
                    if let Some(prev) = current.as_mut().prev_ptr {
                        Node::link(prev, new);
                    }

                    Node::link(new, current);
                },
            )
        }
    }

    pub unsafe fn _insert(
        &mut self,
        new_elem: T,
        end_node: impl Fn(&mut LinkedList<T>) -> &mut OptionLink<T>,
        link_new_node: impl Fn(Link<T>, Link<T>) -> (),
    ) {
        let node = Node {
            prev_ptr: None,
            next_ptr: None,
            elem: new_elem,
        };

        let new_ptr = NonNull::new_unchecked(Box::into_raw(Box::new(node)));
        let current = match self.node_ptr {
            Some(node_ptr) => node_ptr,
            None => {
                self.node_ptr = Some(new_ptr);
                self.list.back_ptr = self.node_ptr;
                self.list.front_ptr = self.node_ptr;
                self.list.len += 1;
                return;
            }
        };

        link_new_node(current, new_ptr);

        let end = end_node(self.list);
        if *end == Some(current) {
            *end = Some(new_ptr);
        }

        self.list.len += 1;
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let node = self.next_node?;
        unsafe {
            let current = &*node.as_ptr();
            self.next_node = current.next_ptr;
            Some(&current.elem)
        }
    }
}
