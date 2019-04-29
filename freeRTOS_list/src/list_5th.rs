use std::rc::{Rc, Weak};
use std::cell::{Ref, RefMut, RefCell};

type BaseType = u16;  // unsighed short
type TickType = u16;  
type TCB = TskTCB;
type StackType = u16;

const portMAX_DELAY :TickType = 0xffff;

// umimplemented!
pub struct TskTCB {
    topOfStack: *mut StackType,         // Points to the location of the last item placed on the tasks stack.
    genericListItem: ListItem,     // The list that the state list item of a task is reference from denotes the state of that task (Ready, Blocked, Suspended ). 
    eventListItem: ListItem,       // Used to reference a task from an event list. 
    priority: BaseType,                 
    stack: *mut StackType,                   // // Points to the start of the stack.
    taskName: String,
}

#[derive(Copy, Clone)]
type Link = Option<Rc<RefCell<ListItem>>>;

#[derive(Clone, Copy)] // we implement the Copy trait
pub struct List {
    number_of_items: BaseType,
    head: Link,
    tail: Link,
}


struct ListItem {
    item_value: TickType,
    owner: Option<Rc<RefCell<TCB>>>,
    container: Option<Rc<RefCell<List>>>,
    next: Link,
    prev: Link,
}


impl ListItem {
    pub fn new(item_value: TickType) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(ListItem {
            item_value: item_value,
            owner: None,
            container: None,
            prev: None,
            next: None,
        }))
    }
}

impl List {
    pub fn new() -> Self {
        List { 
            number_of_items: 0,
            head: None, 
            tail: None,
        }
    }

    pub fn push_front(&mut self, item_value: TickType) {
        let new_head = ListItem::new(item_value);
        match self.head.take() {
            Some(old_head) => {
                new_head.borrow_mut().container = old_head.borrow().container.as_ref().cloned();
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                // new_head.borrow_mut().container = Some(old_head.borrow().container.unwrap().clone());
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                new_head.borrow_mut().container = Some(Rc::new(RefCell::new(*self)));
                self.head = Some(new_head);
            }
        }
    }

//     pub fn push_back(&mut self, elem: T) {
//         let new_tail = Node::new(elem);
//         match self.tail.take() {
//             Some(old_tail) => {
//                 old_tail.borrow_mut().next = Some(new_tail.clone());
//                 new_tail.borrow_mut().prev = Some(old_tail);
//                 self.tail = Some(new_tail);
//             }
//             None => {
//                 self.head = Some(new_tail.clone());
//                 self.tail = Some(new_tail);
//             }
//         }
//     }

//     pub fn pop_back(&mut self) -> Option<T> {
//         self.tail.take().map(|old_tail| {
//             match old_tail.borrow_mut().prev.take() {
//                 Some(new_tail) => {
//                     new_tail.borrow_mut().next.take();
//                     self.tail = Some(new_tail);
//                 }
//                 None => {
//                     self.head.take();
//                 }
//             }
//             Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
//         })
//     }

//     pub fn pop_front(&mut self) -> Option<T> {
//         self.head.take().map(|old_head| {
//             match old_head.borrow_mut().next.take() {
//                 Some(new_head) => {
//                     new_head.borrow_mut().prev.take();
//                     self.head = Some(new_head);
//                 }
//                 None => {
//                     self.tail.take();
//                 }
//             }
//             Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
//         })
//     }

//     pub fn peek_front(&self) -> Option<Ref<T>> {
//         self.head.as_ref().map(|node| {
//             Ref::map(node.borrow(), |node| &node.elem)
//         })
//     }

//     pub fn peek_back(&self) -> Option<Ref<T>> {
//         self.tail.as_ref().map(|node| {
//             Ref::map(node.borrow(), |node| &node.elem)
//         })
//     }

//     pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
//         self.tail.as_ref().map(|node| {
//             RefMut::map(node.borrow_mut(), |node| &mut node.elem)
//         })
//     }

//     pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
//         self.head.as_ref().map(|node| {
//             RefMut::map(node.borrow_mut(), |node| &mut node.elem)
//         })
//     }

//     pub fn into_iter(self) -> IntoIter<T> {
//         IntoIter(self)
//     }
}

// impl<T> Drop for List<T> {
//     fn drop(&mut self) {
//         while self.pop_front().is_some() {}
//     }
// }

// pub struct IntoIter<T>(List<T>);

// impl<T> Iterator for IntoIter<T> {
//     type Item = T;

//     fn next(&mut self) -> Option<T> {
//         self.0.pop_front()
//     }
// }

// impl<T> DoubleEndedIterator for IntoIter<T> {
//     fn next_back(&mut self) -> Option<T> {
//         self.0.pop_back()
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::List;

//     #[test]
//     fn basics() {
//         let mut list = List::new();

//         // Check empty list behaves right
//         assert_eq!(list.pop_front(), None);

//         // Populate list
//         list.push_front(1);
//         list.push_front(2);
//         list.push_front(3);

//         // Check normal removal
//         assert_eq!(list.pop_front(), Some(3));
//         assert_eq!(list.pop_front(), Some(2));

//         // Push some more just to make sure nothing's corrupted
//         list.push_front(4);
//         list.push_front(5);

//         // Check normal removal
//         assert_eq!(list.pop_front(), Some(5));
//         assert_eq!(list.pop_front(), Some(4));

//         // Check exhaustion
//         assert_eq!(list.pop_front(), Some(1));
//         assert_eq!(list.pop_front(), None);

//         // ---- back -----

//         // Check empty list behaves right
//         assert_eq!(list.pop_back(), None);

//         // Populate list
//         list.push_back(1);
//         list.push_back(2);
//         list.push_back(3);

//         // Check normal removal
//         assert_eq!(list.pop_back(), Some(3));
//         assert_eq!(list.pop_back(), Some(2));

//         // Push some more just to make sure nothing's corrupted
//         list.push_back(4);
//         list.push_back(5);

//         // Check normal removal
//         assert_eq!(list.pop_back(), Some(5));
//         assert_eq!(list.pop_back(), Some(4));

//         // Check exhaustion
//         assert_eq!(list.pop_back(), Some(1));
//         assert_eq!(list.pop_back(), None);
//     }

//     #[test]
//     fn peek() {
//         let mut list = List::new();
//         assert!(list.peek_front().is_none());
//         assert!(list.peek_back().is_none());
//         assert!(list.peek_front_mut().is_none());
//         assert!(list.peek_back_mut().is_none());

//         list.push_front(1); list.push_front(2); list.push_front(3);

//         assert_eq!(&*list.peek_front().unwrap(), &3);
//         assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 3);
//         assert_eq!(&*list.peek_back().unwrap(), &1);
//         assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 1);
//     }

//     #[test]
//     fn into_iter() {
//         let mut list = List::new();
//         list.push_front(1); list.push_front(2); list.push_front(3);

//         let mut iter = list.into_iter();
//         assert_eq!(iter.next(), Some(3));
//         assert_eq!(iter.next_back(), Some(1));
//         assert_eq!(iter.next(), Some(2));
//         assert_eq!(iter.next_back(), None);
//         assert_eq!(iter.next(), None);
//     }
// }